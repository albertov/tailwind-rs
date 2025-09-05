use super::*;
use super::transform_utility::{TransformUtility, sealed};

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindScale {
    kind: NumericValue,
    axis: AxisXY,
}
impl Display for TailwindScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        match self.axis {
            AxisXY::N => write!(f, "scale-{}", self.kind),
            AxisXY::X => write!(f, "scale-x-{}", self.kind),
            AxisXY::Y => write!(f, "scale-y-{}", self.kind),
        }
    }
}

// Implement the sealed trait to prevent external implementations
impl sealed::Sealed for TailwindScale {}

// Implement TransformUtility trait for TailwindScale
impl TransformUtility for TailwindScale {
    type ValueType = NumericValue;
    
    fn get_value(&self) -> &Self::ValueType {
        &self.kind
    }
    
    fn get_axis(&self) -> Option<AxisXY> {
        Some(self.axis)
    }
    
    fn css_var_prefix(&self) -> &'static str {
        "--tw-scale"
    }
    
    fn format_css_value(&self, value: &Self::ValueType) -> String {
        // Scale values can be negative in CSS for flipping elements
        // Convert percentage to decimal (e.g., "125" -> "1.25")
        value.get_properties(|f| (f / 100.0).to_string())
    }
}

// noinspection DuplicatedCode
impl TailwindScale {
    // https://tailwindcss.com/docs/scale
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = if negative == true {
            NumericValue::negative_parser("scale", |_| false)(rest, arbitrary, negative)?
        } else {
            NumericValue::positive_parser("scale", |_| false)(rest, arbitrary)?
        };
        Ok(TailwindScale { kind, axis })
    }
}
