use crate::{
    css_attributes, CssAttributes, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance, UnitValue,
};
use std::fmt::{Debug, Display, Formatter};

/// Perspective utility
/// 
/// Supports:
/// - perspective-none (perspective: none)
/// - perspective-[value] (arbitrary values in px)
#[derive(Clone, Debug)]
pub struct TailwindPerspective {
    value: PerspectiveValue,
}

#[derive(Clone, Debug)]
enum PerspectiveValue {
    None,
    Value(UnitValue),
}

impl Display for TailwindPerspective {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            PerspectiveValue::None => write!(f, "perspective-none"),
            PerspectiveValue::Value(val) => {
                val.write_class(f, "perspective-")
            }
        }
    }
}

impl TailwindInstance for TailwindPerspective {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let value = match &self.value {
            PerspectiveValue::None => "none".to_string(),
            PerspectiveValue::Value(val) => val.get_properties(|f| format!("{}px", f)),
        };
        css_attributes! {
            "perspective" => value,
        }
    }
}

impl TailwindPerspective {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let value = match input {
            ["none"] => PerspectiveValue::None,
            _ => {
                // Parse arbitrary value or numeric value
                let unit_val = UnitValue::positive_parser("perspective", |_| false, false, false, false)(input, arbitrary)?;
                PerspectiveValue::Value(unit_val)
            }
        };
        Ok(Self { value })
    }
}