use super::*;
use super::transform_utility::{TransformUtility, sealed};

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindSkew {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindSkew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        match self.axis {
            AxisXY::X => write!(f, "skew-x-{}", self.kind),
            AxisXY::Y => write!(f, "skew-y-{}", self.kind),
            AxisXY::N => write!(f, "skew-{}", self.kind),
        }
    }
}

// Implement the sealed trait to prevent external implementations
impl sealed::Sealed for TailwindSkew {}

// Implement the TransformUtility trait for TailwindSkew
impl TransformUtility for TailwindSkew {
    type ValueType = UnitValue;
    
    fn get_value(&self) -> &Self::ValueType {
        &self.kind
    }
    
    fn get_axis(&self) -> Option<AxisXY> {
        Some(self.axis)
    }
    
    fn css_var_prefix(&self) -> &'static str {
        "--tw-skew"
    }
    
    fn format_css_value(&self, value: &Self::ValueType) -> String {
        value.get_properties(|n| format!("{}deg", n))
    }
}

impl TailwindSkew {
    // <https://tailwindcss.com/docs/skew>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = if negative == true {
            UnitValue::negative_parser("skew", |_| false, false, false, false)(rest, arbitrary, negative)?
        } else {
            UnitValue::positive_parser("skew", |_| false, false, false, false)(rest, arbitrary)?
        };
        Ok(Self { kind, axis })
    }
}
