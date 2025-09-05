use super::*;

pub use self::{
    bottom::TailwindBottom, end::TailwindEnd, inset::TailwindInset, left::TailwindLeft,
    right::TailwindRight, start::TailwindStart, top::TailwindTop,
};

mod bottom;
mod end;
mod inset;
mod left;
mod right;
mod start;
mod top;

pub(crate) fn get_kind_px_full_auto_fact(
    id: &'static str,
    pattern: &[&str],
    arbitrary: &TailwindArbitrary,
    negative: Negative,
) -> Result<UnitValue> {
    let kind = match pattern {
        ["px"] => {
            if negative == true {
                UnitValue::Length(LengthUnit::px(-1.0))
            } else {
                UnitValue::px(1.0)
            }
        },
        ["full"] => {
            if negative == true {
                UnitValue::Length(LengthUnit::Unit(-100.0, "%"))
            } else {
                UnitValue::radio(100, 100)
            }
        },
        _ => UnitValue::negative_parser(id, check_valid_auto, true, false, true)(pattern, arbitrary, negative)?,
    };
    Ok(kind)
}

pub(crate) fn check_valid_auto(mode: &str) -> bool {
    ["auto", "inherit", "initial", "revert", "unset"].contains(&mode)
}
