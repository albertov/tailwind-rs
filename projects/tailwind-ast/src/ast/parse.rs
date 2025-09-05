use nom::error::{ErrorKind, ParseError};
use nom::combinator::{success, map};

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
    fn parse_pair(input: &'a str) -> IResult<&'a str, Vec<AstGroupItem<'a>>> {
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
    /// `v:v::-?a-a-a-[A]/opacity` or `v:v::!-?a-a-a-[A]/opacity` or `!v:v::-?a-a-a-[A]/opacity`
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        // Check for important flag at the very beginning (before variants)
        let (input_after_first_important, important_at_start) = opt(char('!'))(input)?;
        
        // Parse variants
        let (input_after_variants, variants) = many0(ASTVariant::parse)(input_after_first_important)?;
        
        // Check for important flag after variants (legacy position)
        let (input_after_important, important_after_variants) = opt(char('!'))(input_after_variants)?;
        
        let (rest, (negative, elements, arbitrary, opacity, important_suffix)) = tuple((
            opt(char('-')),
            opt(AstElements::parse),
            opt(AstArbitrary::parse),
            opt(Self::parse_opacity),
            opt(char('!')),
        ))(input_after_important)?;

        Ok((
            rest,
            Self {
                // Important can be at the very beginning, after variants, or at the end
                important: important_at_start.is_some() || important_after_variants.is_some() || important_suffix.is_some(),
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
            // space and delimiters (including ! for important)
            matches!(c, ' ' | '\n' | '\r' | '-' | '[' | ']' | '(' | ')' | '/' | '!')
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
    /// `(not-)?variant(/modifier)?:pseudo::` or `@container-query:` or `has-[selector]:`
    ///
    /// ## Reference
    /// -
    #[inline]
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        // Check for @ prefix first (container queries)
        if input.starts_with('@') {
            return Self::parse_container_query(input);
        }
        
        // Check for min-[...] and max-[...] arbitrary responsive breakpoints
        if input.starts_with("min-[") || input.starts_with("max-[") {
            return Self::parse_arbitrary_responsive(input);
        }
        
        // Check for has-[ pattern (arbitrary has selector)
        // Note: Must handle both has-[...] and has-[[...]] (nested brackets for attribute selectors)
        // This must be checked before parse_one to handle group-has-[...] and peer-has-[...] properly
        if input.starts_with("has-[") || input.starts_with("group-has-[") || input.starts_with("peer-has-[") {
            return Self::parse_has_arbitrary(input);
        }
        
        // Check for group-[arbitrary] and peer-[arbitrary] patterns
        if input.starts_with("group-[") || input.starts_with("peer-[") {
            return Self::parse_group_peer_arbitrary(input);
        }
        
        let (rest, (mut v, s)) = tuple((Self::parse_one, alt((tag("::"), tag(":")))))(input)?;
        if s == "::" {
            v.pseudo = true
        }
        else {
            v.pseudo = Self::check_pseudo(&v.names.iter().map(<_>::as_ref).collect::<Vec<_>>());
        }
        Ok((rest, v))
    }
    /// `(not-)?(ALPHA)(-ALPHA)*(/modifier)?`
    ///
    /// eg:
    /// - `not-focus`
    /// - `not-last-child`
    /// - `group-hover/sidebar`
    /// - `peer-checked/input`
    /// - `has-checked`
    /// - `has-[>img]`
    /// - `group-has-focus`
    /// - `has-not-checked`
    #[inline]
    fn parse_one(input: &'a str) -> IResult<&'a str, Self> {
        // Parse not- prefix
        let (input, not) = opt(tuple((tag("not"), tag("-"))))(input)?;
        
        // Parse variant names
        let (input, names) = separated_list0(tag("-"), alphanumeric1)(input)?;
        
        // Check if this is a has variant pattern
        let has_index = names.iter().position(|&n| n == "has");
        
        // Determine if this is a has variant and extract selector
        let (has, has_selector, final_names, has_not) = if let Some(idx) = has_index {
            // Found "has" in the names
            // Check if the next element is "not" (for has-not-* patterns)
            let is_has_not = idx + 1 < names.len() && names[idx + 1] == "not";
            
            // Check if there's an arbitrary selector in brackets
            if input.starts_with('[') {
                // Parse arbitrary selector
                let (rest, selector) = delimited::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
                    char('['),
                    take_till1(|c: char| c == ']'),
                    char(']')
                )(input)?;
                
                // Names before "has" are the variant prefix (e.g., "group" or "peer")
                let prefix_names = if idx > 0 {
                    names[..idx].to_vec()
                } else {
                    vec![]
                };
                
                // Parse modifier after the selector if present
                let (rest, modifier) = opt(Self::parse_modifier)(rest)?;
                
                return Ok((rest, Self { 
                    not: not.is_some() || is_has_not, 
                    pseudo: false, 
                    names: prefix_names,
                    modifier,
                    container: false,
                    container_type: None,
                    has: true,
                    has_selector: Some(selector),
                    arbitrary_selector: None,
                }));
            } else if is_has_not {
                // Handle has-not-* patterns
                let prefix_names = if idx > 0 {
                    names[..idx].to_vec()
                } else {
                    vec![]
                };
                
                // Skip "not" and include the rest as the selector names
                let mut final_names = prefix_names;
                final_names.push("has");
                if idx + 2 < names.len() {
                    final_names.extend_from_slice(&names[idx + 2..]);
                }
                
                (true, None, final_names, true)
            } else {
                // No arbitrary selector and not a has-not pattern
                // For non-arbitrary selectors, we keep the full name structure
                // but mark it as a has variant
                (true, None, names.clone(), false)
            }
        } else {
            // Not a has variant
            (false, None, names, false)
        };
        
        // Parse optional modifier
        let (rest, modifier) = opt(Self::parse_modifier)(input)?;
        
        Ok((rest, Self { 
            not: not.is_some() || has_not, 
            pseudo: false, 
            names: final_names,
            modifier,
            container: false,
            container_type: None,
            has,
            has_selector,
            arbitrary_selector: None,
        }))
    }
    
    /// Parse modifier like `/sidebar` or `/[data-state=open]`
    #[inline]
    fn parse_modifier(input: &'a str) -> IResult<&'a str, &'a str> {
        let (rest, _) = char('/')(input)?;
        alt((
            // Arbitrary modifier in brackets
            delimited(char('['), take_till1(|c| c == ']'), char(']')),
            // Named modifier
            alphanumeric1,
        ))(rest)
    }
    
    /// Parse has-[selector]: patterns
    #[inline]
    fn parse_has_arbitrary(input: &'a str) -> IResult<&'a str, Self> {
        // Parse prefix (has, group-has, peer-has)
        let (input, prefix) = alt((
            tag("peer-has-"),
            tag("group-has-"),
            tag("has-"),
        ))(input)?;
        
        // Parse the arbitrary selector in brackets - handle nested brackets
        let (input, _) = char('[')(input)?;
        
        // Find the matching closing bracket, accounting for nested brackets
        let mut bracket_count = 1;
        let mut end_pos = 0;
        let mut chars = input.char_indices();
        
        while bracket_count > 0 {
            match chars.next() {
                Some((pos, '[')) => {
                    bracket_count += 1;
                    end_pos = pos;
                }
                Some((pos, ']')) => {
                    bracket_count -= 1;
                    end_pos = pos;
                }
                Some((pos, _)) => {
                    end_pos = pos;
                }
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::TakeWhile1,
                    )));
                }
            }
        }
        
        let selector = &input[..end_pos];
        let input = &input[end_pos + 1..]; // Skip the closing ]
        
        // Parse optional modifier
        let (input, modifier) = opt(Self::parse_modifier)(input)?;
        
        // Consume the colon
        let (rest, _) = char(':')(input)?;
        
        // Determine the prefix names based on what was matched
        let names = match prefix {
            "peer-has-" => vec!["peer"],
            "group-has-" => vec!["group"],
            _ => vec![],
        };
        
        Ok((rest, Self {
            not: false,
            pseudo: false,
            names,
            modifier,
            container: false,
            container_type: None,
            has: true,
            has_selector: Some(selector),
            arbitrary_selector: None,
        }))
    }
    
    /// Parse group-[arbitrary]: and peer-[arbitrary]: patterns
    #[inline]
    fn parse_group_peer_arbitrary(input: &'a str) -> IResult<&'a str, Self> {
        // Parse prefix (group- or peer-)
        let (input, prefix) = alt((
            tag("peer-"),
            tag("group-"),
        ))(input)?;
        
        // Parse the arbitrary selector in brackets - handle nested brackets
        let (input, _) = char('[')(input)?;
        
        // Find the matching closing bracket, accounting for nested brackets
        let mut bracket_count = 1;
        let mut end_pos = 0;
        let mut chars = input.char_indices();
        
        while bracket_count > 0 {
            match chars.next() {
                Some((pos, '[')) => {
                    bracket_count += 1;
                    end_pos = pos;
                }
                Some((pos, ']')) => {
                    bracket_count -= 1;
                    end_pos = pos;
                }
                Some((pos, _)) => {
                    end_pos = pos;
                }
                None => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::TakeWhile1,
                    )));
                }
            }
        }
        
        let selector = &input[..end_pos];
        let input = &input[end_pos + 1..]; // Skip the closing ]
        
        // Parse optional modifier
        let (input, modifier) = opt(Self::parse_modifier)(input)?;
        
        // Consume the colon
        let (rest, _) = char(':')(input)?;
        
        // Determine the names based on the prefix
        let names = match prefix {
            "peer-" => vec!["peer"],
            "group-" => vec!["group"],
            _ => vec![],
        };
        
        Ok((rest, Self {
            not: false,
            pseudo: false,
            names,
            modifier,
            container: false,
            container_type: None,
            has: false,
            has_selector: None,
            // Store the arbitrary selector in a new field
            arbitrary_selector: Some(selector),
        }))
    }
    
    /// Parse container query patterns like @lg:, @max-lg:, @[123px]:, @lg/sidebar:
    #[inline]
    fn parse_container_query(input: &'a str) -> IResult<&'a str, Self> {
        // Skip the @ prefix
        let (input, _) = char('@')(input)?;
        
        // Parse the query pattern
        let (rest, (query_type, names, modifier)) = alt((
            // @[arbitrary]:
            Self::parse_arbitrary_container,
            // @max-lg: or @min-lg: or @lg:
            Self::parse_named_container,
        ))(input)?;
        
        // Consume the colon
        let (rest, _) = char(':')(rest)?;
        
        Ok((rest, Self {
            not: false,
            pseudo: false,
            names,
            modifier,
            container: true,
            container_type: Some(query_type),
            has: false,
            has_selector: None,
            arbitrary_selector: None,
        }))
    }
    
    /// Parse arbitrary container query like @[123px]
    #[inline]
    fn parse_arbitrary_container(input: &'a str) -> IResult<&'a str, (ContainerQueryType, Vec<&'a str>, Option<&'a str>)> {
        // Parse @[123px] pattern
        let (rest, value) = delimited(
            char('['),
            take_till1(|c| c == ']'),
            char(']')
        )(input)?;
        
        // Parse optional modifier like /sidebar
        let (rest, modifier) = opt(Self::parse_modifier)(rest)?;
        
        Ok((rest, (
            ContainerQueryType::Arbitrary,
            vec![value], // Store arbitrary value in names
            modifier
        )))
    }
    
    /// Parse named container query like @lg, @max-lg, @min-lg
    #[inline]
    fn parse_named_container(input: &'a str) -> IResult<&'a str, (ContainerQueryType, Vec<&'a str>, Option<&'a str>)> {
        // Check for max- or min- prefix
        let (input, query_type) = alt((
            map(tag("max-"), |_| ContainerQueryType::Max),
            map(tag("min-"), |_| ContainerQueryType::Min),
            success(ContainerQueryType::Min), // Default to min
        ))(input)?;
        
        // Parse the breakpoint name (e.g., lg, md, sm)
        let (rest, breakpoint) = alphanumeric1(input)?;
        
        // Parse optional modifier like /sidebar
        let (rest, modifier) = opt(Self::parse_modifier)(rest)?;
        
        Ok((rest, (query_type, vec![breakpoint], modifier)))
    }
    
    /// Parse arbitrary responsive breakpoints like min-[400px]: and max-[600px]:
    #[inline]
    fn parse_arbitrary_responsive(input: &'a str) -> IResult<&'a str, Self> {
        // Parse min- or max- prefix
        let (input, is_max) = alt((
            map(tag("max-"), |_| true),
            map(tag("min-"), |_| false),
        ))(input)?;
        
        // Parse the arbitrary value in brackets
        let (input, _) = char('[')(input)?;
        let (input, value) = take_till1(|c| c == ']')(input)?;
        let (input, _) = char(']')(input)?;
        
        // Consume the colon
        let (rest, _) = char(':')(input)?;
        
        // Create a variant that represents this arbitrary breakpoint
        // We'll use the names field to store "min" or "max" and the arbitrary_selector to store the value
        let names = if is_max {
            vec!["max"]
        } else {
            vec!["min"]
        };
        
        Ok((rest, Self {
            not: false,
            pseudo: false,
            names,
            modifier: None,
            container: false,
            container_type: None,
            has: false,
            has_selector: None,
            arbitrary_selector: Some(value), // Store the breakpoint value here
        }))
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
