use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetWidth {
    kind: NumericValue,
}

impl Display for TailwindRingOffsetWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.kind)
    }
}

impl TailwindInstance for TailwindRingOffsetWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let offset_width = self.kind.get_properties(|f| format!("{}px", f));
        css_attributes! {
            "--tw-ring-offset-width" => &offset_width,
            "--tw-ring-offset-shadow" => format!("var(--tw-ring-inset, ) 0 0 0 {} var(--tw-ring-offset-color, #fff)", offset_width),
            "box-shadow" => "var(--tw-inset-shadow, 0 0 #0000), var(--tw-inset-ring-shadow, 0 0 #0000), var(--tw-ring-offset-shadow), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow, 0 0 #0000)"
        }
    }
}

impl TailwindRingOffsetWidth {
    /// <https://tailwindcss.com/docs/ring-offset-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = NumericValue::positive_parser("ring-offset-width", Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    
    /// Check for valid CSS keywords
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
