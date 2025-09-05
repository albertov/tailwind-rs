use super::*;
use super::transform_utility::{TransformUtility, sealed};

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTranslate {
    axis: AxisXY,
    kind: UnitValue,
}

impl Display for TailwindTranslate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        match self.axis {
            AxisXY::X => write!(f, "translate-x-{}", self.kind),
            AxisXY::Y => write!(f, "translate-y-{}", self.kind),
            AxisXY::N => write!(f, "translate-{}", self.kind),
        }
    }
}

// Implement the sealed trait to prevent external implementations
impl sealed::Sealed for TailwindTranslate {}

// Implement the TransformUtility trait
impl TransformUtility for TailwindTranslate {
    type ValueType = UnitValue;
    
    fn get_value(&self) -> &Self::ValueType {
        &self.kind
    }
    
    fn get_axis(&self) -> Option<AxisXY> {
        Some(self.axis)
    }
    
    fn css_var_prefix(&self) -> &'static str {
        "--tw-translate"
    }
    
    fn format_css_value(&self, value: &Self::ValueType) -> String {
        value.get_properties(|n| format!("{}rem", n / 4.0))
    }
}

// noinspection DuplicatedCode
impl TailwindTranslate {
    /// <https://tailwindcss.com/docs/translate>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Self> {
        let (axis, rest) = AxisXY::split_xyn(pattern);
        let kind = match rest {
            ["px"] => UnitValue::px(1.0),
            ["full"] => UnitValue::radio(1, 1),
            _ => UnitValue::negative_parser("translate", |_| false, true, false, false)(rest, arbitrary, negative)?,
        };
        Ok(Self { kind, axis })
    }
}
