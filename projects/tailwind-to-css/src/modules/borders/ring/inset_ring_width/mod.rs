use crate::NumericValue;

use super::*;

/// Inset ring width utilities (inset-ring, inset-ring-0, inset-ring-1, etc.)
#[derive(Clone, Debug)]
pub struct TailwindInsetRingWidth {
    kind: NumericValue,
    is_default: bool,
}

impl Display for TailwindInsetRingWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_default {
            write!(f, "inset-ring")
        } else {
            write!(f, "inset-ring-{}", self.kind)
        }
    }
}

impl TailwindInstance for TailwindInsetRingWidth {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Inset ring uses inset box-shadow
        let ring_width = if self.is_default {
            // v4 default: 1px
            "1px".to_string()
        } else {
            self.kind.get_properties(|f| format!("{}px", f))
        };
        
        css_attributes! {
            "--tw-inset-ring-shadow" => format!("inset 0 0 0 {} var(--tw-inset-ring-color, currentColor)", ring_width),
            "box-shadow" => "var(--tw-inset-shadow, 0 0 #0000), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow, 0 0 #0000)"
        }
    }
}

impl TailwindInsetRingWidth {
    /// Parse inset-ring width utilities
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (kind, is_default) = match pattern {
            [] => (NumericValue::from(1u32), true), // v4 default: 1px
            _ => (NumericValue::positive_parser("inset-ring", Self::check_valid)(pattern, arbitrary)?, false),
        };
        Ok(Self { kind, is_default })
    }
    
    /// Check for valid CSS keywords
    pub fn check_valid(mode: &str) -> bool {
        ["inherit", "initial", "revert", "unset"].contains(&mode)
    }
}