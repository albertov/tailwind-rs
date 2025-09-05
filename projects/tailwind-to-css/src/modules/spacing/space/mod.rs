use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSpace {
    axis: bool,
    negative: Negative,
    size: SpacingSize,
}

impl Display for TailwindSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.negative.write(f)?;
        match self.axis {
            true => write!(f, "space-x-{}", self.size),
            false => write!(f, "space-y-{}", self.size),
        }
    }
}

impl TailwindInstance for TailwindSpace {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Return empty attributes since we handle everything in additional()
        CssAttributes::default()
    }
    
    fn additional(&self, _: &TailwindBuilder) -> String {
        // Generate the complete CSS rule with child combinator selector
        let property = match self.axis {
            true => "margin-left",
            false => "margin-top",
        };
        let value = self.size.get_properties();
        
        // Get the class name using id() which returns the escaped version
        let class_name = self.id();
        
        // Escape special characters for CSS selector
        let escaped_class = class_name
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('%', "\\%")
            .replace(' ', "_")
            .replace('.', "\\.");
        
        // Use the > * + * selector to target all children except the first
        format!(".{} > * + * {{ {}: {}; }}", 
                escaped_class,
                property, 
                value)
    }
}

impl TailwindSpace {
    /// https://tailwindcss.com/docs/space
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            ["x", rest @ ..] => Self::parse_axis(rest, arbitrary, true, negative),
            ["y", rest @ ..] => Self::parse_axis(rest, arbitrary, false, negative),
            _ => syntax_error!("Unknown space instructions: {}", pattern.join("-")),
        }
    }
    fn parse_axis(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        axis: bool,
        negative: Negative,
    ) -> Result<Box<dyn TailwindInstance>> {
        match pattern {
            [] => Ok(Self::parse_arbitrary(arbitrary, negative, axis)?.boxed()),
            ["reverse"] => Ok(TailwindSpaceReverse::from(axis).boxed()),
            _ => {
                let size = SpacingSize::parse(pattern, arbitrary, &Self::check_valid)?;
                Ok(Self { axis, negative, size }.boxed())
            },
        }
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: Negative, axis: bool) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { axis, negative, size })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/margin#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["auto", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}
