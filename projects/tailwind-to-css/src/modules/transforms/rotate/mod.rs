use super::*;
use super::transform_utility::{TransformUtility, sealed};

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRotate {
    kind: UnitValue,
}

impl Display for TailwindRotate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "rotate-")
    }
}

// Implement sealed trait to prevent external implementations
impl sealed::Sealed for TailwindRotate {}

// Implement TransformUtility trait for unified transform handling
impl TransformUtility for TailwindRotate {
    type ValueType = UnitValue;
    
    fn get_value(&self) -> &Self::ValueType {
        &self.kind
    }
    
    fn get_axis(&self) -> Option<AxisXY> {
        // Rotate doesn't have axis-specific variants
        None
    }
    
    fn css_var_prefix(&self) -> &'static str {
        "--tw-rotate"
    }
    
    fn format_css_value(&self, value: &Self::ValueType) -> String {
        value.get_properties(|f| format!("{}deg", f))
    }
}

impl TailwindRotate {
    // <https://tailwindcss.com/docs/rotate>
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let kind = if negative == true {
            UnitValue::negative_parser("rotate", |_| false, false, false, false)(input, arbitrary, negative)?
        } else {
            UnitValue::positive_parser("rotate", |_| false, false, false, false)(input, arbitrary)?
        };
        Ok(Self { kind })
    }
}
