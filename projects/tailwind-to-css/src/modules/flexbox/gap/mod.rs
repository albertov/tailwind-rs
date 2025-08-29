use super::*;
use crate::AxisXY;

#[doc=include_str!("readme.md")]
#[derive(Debug, Copy, Clone)]
pub struct TailwindGap {
    size: LengthUnit,
    axis: AxisXY,
}

impl Display for TailwindGap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            AxisXY::N => write!(f, "gap-{}", self.size.get_class_arbitrary()),
            AxisXY::X => write!(f, "gap-x-{}", self.size.get_class_arbitrary()),
            AxisXY::Y => write!(f, "gap-y-{}", self.size.get_class_arbitrary()),
        }
    }
}

impl TailwindInstance for TailwindGap {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.axis {
            AxisXY::N => "gap",
            AxisXY::X => "column-gap",
            AxisXY::Y => "row-gap",
        };
        css_attributes! {
            class => self.size.get_properties()
        }
    }
}

impl TailwindGap {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            ["x", rest @ ..] => Ok(Self { size: parse_size(rest, arbitrary)?, axis: AxisXY::X }),
            ["y", rest @ ..] => Ok(Self { size: parse_size(rest, arbitrary)?, axis: AxisXY::Y }),
            _ => Ok(Self { size: parse_size(pattern, arbitrary)?, axis: AxisXY::N }),
        }
    }
}

fn parse_size(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<LengthUnit> {
    let size = match pattern {
        [] => arbitrary.as_length_or_fraction()?,
        ["0"] => LengthUnit::px(0.0),
        ["px"] => LengthUnit::px(1.0),
        ["0.5"] => LengthUnit::rem(0.125),
        ["1"] => LengthUnit::rem(0.25),
        ["1.5"] => LengthUnit::rem(0.375),
        ["2"] => LengthUnit::rem(0.5),
        ["2.5"] => LengthUnit::rem(0.625),
        ["3"] => LengthUnit::rem(0.75),
        ["3.5"] => LengthUnit::rem(0.875),
        ["4"] => LengthUnit::rem(1.0),
        ["5"] => LengthUnit::rem(1.25),
        ["6"] => LengthUnit::rem(1.5),
        ["7"] => LengthUnit::rem(1.75),
        ["8"] => LengthUnit::rem(2.0),
        ["9"] => LengthUnit::rem(2.25),
        ["10"] => LengthUnit::rem(2.5),
        ["11"] => LengthUnit::rem(2.75),
        ["12"] => LengthUnit::rem(3.0),
        ["14"] => LengthUnit::rem(3.5),
        ["16"] => LengthUnit::rem(4.0),
        ["20"] => LengthUnit::rem(5.0),
        ["24"] => LengthUnit::rem(6.0),
        ["28"] => LengthUnit::rem(7.0),
        ["32"] => LengthUnit::rem(8.0),
        ["36"] => LengthUnit::rem(9.0),
        ["40"] => LengthUnit::rem(10.0),
        ["44"] => LengthUnit::rem(11.0),
        ["48"] => LengthUnit::rem(12.0),
        ["52"] => LengthUnit::rem(13.0),
        ["56"] => LengthUnit::rem(14.0),
        ["60"] => LengthUnit::rem(15.0),
        ["64"] => LengthUnit::rem(16.0),
        ["72"] => LengthUnit::rem(18.0),
        ["80"] => LengthUnit::rem(20.0),
        ["96"] => LengthUnit::rem(24.0),
        [n] => {
            // Fallback to arbitrary value
            let a = TailwindArbitrary::from(*n);
            a.as_length_or_fraction()?
        },
        _ => return syntax_error!("Unknown gap instructions"),
    };
    Ok(size)
}
