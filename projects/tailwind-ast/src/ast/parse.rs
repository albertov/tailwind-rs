use nom::error::{ErrorKind, ParseError};

use super::*;

impl<'a> AstGroup<'a> {
    /// `v:a?(a(a b))[!]?`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (head, children, important)) =
            tuple((AstStyle::parse, Self::parse_pair, opt(char('!'))))(input)?;
        Ok((rest, Self { important: important.is_some(), head, children }))
    }
    #[inline]
    fn parse_pair(input: &'a str) -> IResult<&'a str, Vec<AstGroupItem>> {
        let (rest, paired) = delimited_paired('(', ')')(input)?;
        Ok((rest, AstGroupItem::parse_many(paired.trim())?.1))
    }
}

impl<'a> AstGroupItem<'a> {
    /// [`AstGroup`] or [`AstStyle`]
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((Self::maybe_group, Self::maybe_style))(input)
    }
    #[inline]
    fn parse_many(input: &'a str) -> IResult<&'a str, Vec<Self>> {
        let head = AstGroupItem::parse;
        let rest = many0(tuple((multispace1, AstGroupItem::parse)));
        let (rest, (first, other)) = tuple((head, rest))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter().map(|s| s.1));
        Ok((rest, out))
    }
    fn maybe_group(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, o) = AstGroup::parse(input)?;
        Ok((rest, Self::Grouped(o)))
    }
    fn maybe_style(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, o) = AstStyle::parse(input)?;
        Ok((rest, Self::Styled(o)))
    }
}

impl<'a> AstStyle<'a> {
    /// `v:v::-?a-a-a-[A]/opacity`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (variants, negative, elements, arbitrary, opacity, important)) = tuple((
            many0(ASTVariant::parse),
            opt(char('-')),
            opt(AstElements::parse),
            opt(AstArbitrary::parse),
            opt(Self::parse_opacity),
            opt(char('!')),
        ))(input)?;

        Ok((
            rest,
            Self {
                important: important.is_some(),
                negative: negative.is_some(),
                variants,
                elements: elements.unwrap_or_default().elements,
                arbitrary: arbitrary.map(|s| s.arbitrary),
                opacity,
            },
        ))
    }
    
    /// Parse opacity modifier like `/50` or `/[0.23]`
    #[inline]
    fn parse_opacity(input: &'a str) -> IResult<&'a str, &'a str> {
        let (rest, (_, opacity)) = tuple((
            char('/'),
            alt((
                // Arbitrary value in brackets
                delimited(char('['), take_till1(|c| c == ']'), char(']')),
                // Predefined opacity value
                alphanumeric1,
            )),
        ))(input)?;
        Ok((rest, opacity))
    }
}

impl<'a> AstElements<'a> {
    /// `a(-a)*`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (first, other)) = tuple((Self::parse_head, many0(Self::parse_rest)))(input)?;
        let mut out = vec![first];
        out.extend(other.into_iter());
        Ok((rest, Self { elements: out }))
    }
    #[inline]
    fn parse_head(input: &'a str) -> IResult<&'a str, &'a str> {
        // Try to parse as a fraction first (e.g., "1/2", "3/4")
        // But only if it looks like a valid fraction (small numbers on both sides)
        if let Ok((rest, (num, denom))) = crate::utils::parse_fraction(input) {
            // Check if this looks like a legitimate fraction (both parts are small numbers)
            // This helps distinguish "1/2" (fraction) from "500/50" (not a fraction, likely color-500 with /50 opacity)
            if num <= 12 && denom <= 12 {
                // If it's a valid fraction, return the whole fraction string
                let fraction_len = input.len() - rest.len();
                return Ok((rest, &input[..fraction_len]));
            }
        }
        
        let stop = |c: char| -> bool {
            // space and delimiters
            matches!(c, ' ' | '\n' | '\r' | '-' | '[' | ']' | '(' | ')' | '/')
        };
        take_till1(stop)(input)
    }
    #[inline]
    fn parse_rest(input: &'a str) -> IResult<&'a str, &'a str> {
        let (rest, (_, out)) = tuple((char('-'), Self::parse_head))(input)?;
        Ok((rest, out))
    }
}

impl<'a> ASTVariant<'a> {
    /// `(not-)?variant:pseudo::`
    ///
    /// ## Reference
    /// -
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (rest, (mut v, s)) = tuple((Self::parse_one, alt((tag("::"), tag(":")))))(input)?;
        if s == "::" {
            v.pseudo = true
        }
        else {
            v.pseudo = Self::check_pseudo(&v.names.iter().map(<_>::as_ref).collect::<Vec<_>>());
        }
        Ok((rest, v))
    }
    /// `(not-)?(ALPHA)(-ALPHA)*`
    ///
    /// eg:
    /// - `not-focus`
    /// - `not-last-child`
    #[inline]
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        let not = opt(tuple((tag("not"), tag("-"))));
        let vs = separated_list0(tag("-"), alphanumeric1);
        let (rest, (not, names)) = tuple((not, vs))(input)?;
        Ok((rest, Self { not: not.is_some(), pseudo: false, names }))
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-elements#index
    #[rustfmt::skip] #[inline]
    fn check_pseudo(names: &[&str]) -> bool {
        matches!(names
            , ["after"]
            | ["before"]
            | ["backdrop"]
            | ["marker"]
            | ["placeholder"]
            | ["selection"]
            | ["first", "line"]
            | ["first", "litter"]
            | ["first", "selector", "button"]
            | ["target", "text"]
        )
    }
}

impl<'a> AstArbitrary<'a> {
    /// `-[ANY+]`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        // Parse the opening -[
        let (input, _) = char('-')(input)?;
        let (input, _) = char('[')(input)?;
        
        // Custom parser that handles quotes, escapes, and nested brackets
        let mut byte_index = 0;
        let mut bracket_depth = 0;
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escaped = false;
        
        let mut chars = input.chars();
        
        while let Some(ch) = chars.next() {
            if escaped {
                // Skip escaped character
                escaped = false;
                byte_index += ch.len_utf8();
                continue;
            }
            
            if ch == '\\' {
                // Next character is escaped
                escaped = true;
                byte_index += ch.len_utf8();
                continue;
            }
            
            // Handle quotes (but not if escaped)
            if ch == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
                byte_index += ch.len_utf8();
                continue;
            }
            
            if ch == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
                byte_index += ch.len_utf8();
                continue;
            }
            
            // Only process brackets if not inside quotes
            if !in_single_quote && !in_double_quote {
                if ch == '[' {
                    bracket_depth += 1;
                } else if ch == ']' {
                    if bracket_depth == 0 {
                        // Found the closing bracket for our arbitrary value
                        break;
                    }
                    bracket_depth -= 1;
                }
            }
            
            byte_index += ch.len_utf8();
        }
        
        // Extract the arbitrary value (everything up to the closing ])
        let arbitrary = &input[..byte_index];
        
        // Fail if the arbitrary value is empty
        if arbitrary.is_empty() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TakeWhile1,
            )));
        }
        
        // Skip the closing ]
        let rest = &input[byte_index..];
        let (rest, _) = char(']')(rest)?;
        
        Ok((rest, Self { arbitrary }))
    }
}

impl AstReference {
    /// `&`
    #[inline]
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, _) = char('&')(input)?;
        Ok((rest, Self {}))
    }
}

fn delimited_paired(opening: char, closing: char) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input: &str| {
        delimited(char(opening), take_until_unbalanced(opening, closing), char(closing))(input)
    }
}

/// https://stackoverflow.com/questions/70630556/parse-allowing-nested-parentheses-in-nom
fn take_until_unbalanced(
    opening_bracket: char,
    closing_bracket: char,
) -> impl Fn(&str) -> IResult<&str, &str> {
    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(n) = &i[index..].find(&[opening_bracket, closing_bracket, '\\'][..]) {
            index += n;
            let mut it = i[index..].chars();
            match it.next().unwrap_or_default() {
                c if c == '\\' => {
                    // Skip the escape char `\`.
                    index += '\\'.len_utf8();
                    // Skip also the following char.
                    if let Some(c) = it.next() {
                        index += c.len_utf8();
                    }
                }
                c if c == opening_bracket => {
                    bracket_counter += 1;
                    index += opening_bracket.len_utf8();
                }
                c if c == closing_bracket => {
                    // Closing bracket.
                    bracket_counter -= 1;
                    index += closing_bracket.len_utf8();
                }
                // Can not happen.
                _ => unreachable!(),
            };
            // We found the unmatched closing bracket.
            if bracket_counter == -1 {
                // We do not consume it.
                index -= closing_bracket.len_utf8();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        }
        else {
            Err(Err::Error(Error::from_error_kind(i, ErrorKind::TakeUntil)))
        }
    }
}
