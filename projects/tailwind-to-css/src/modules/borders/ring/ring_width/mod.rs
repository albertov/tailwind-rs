use crate::NumericValue;

use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingWidth {
    kind: NumericValue,
    is_default: bool,
}

impl Display for TailwindRingWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_default {
            write!(f, "ring")
        } else {
            write!(f, "ring-{}", self.kind)
        }
    }
}

impl TailwindInstance for TailwindRingWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Ring utilities use a complex box-shadow calculation
        // The ring shadow is calculated as: calc(width + offset-width)
        let ring_width = if self.is_default {
            // CRITICAL: v4 default is 1px (was 3px in v3)
            "1px".to_string()
        } else {
            self.kind.get_properties(|f| format!("{}px", f))
        };
        
        css_attributes! {
            "--tw-ring-shadow" => format!("var(--tw-ring-inset, ) 0 0 0 calc({} + var(--tw-ring-offset-width, 0px)) var(--tw-ring-color, currentColor)", ring_width),
            "box-shadow" => "var(--tw-inset-shadow, 0 0 #0000), var(--tw-inset-ring-shadow, 0 0 #0000), var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)"
        }
    }
}

impl TailwindRingWidth {
    /// <https://tailwindcss.com/docs/ring-width>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (kind, is_default) = match pattern {
            [] if arbitrary.is_some() => {
                // Handle arbitrary values like ring-[3px]
                (NumericValue::positive_parser("ring", Self::check_valid)(pattern, arbitrary)?, false)
            },
            [] => (NumericValue::from(1u32), true), // v4 default: 1px (was 3px in v3)
            _ => (NumericValue::positive_parser("ring", Self::check_valid)(pattern, arbitrary)?, false),
        };
        Ok(Self { kind, is_default })
    }
    
    /// Check for valid CSS keywords
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}
