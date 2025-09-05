pub use self::{
    origin::TailwindOrigin, 
    perspective::TailwindPerspective,
    perspective_origin::TailwindPerspectiveOrigin,
    rotate::TailwindRotate, 
    scale::TailwindScale, 
    skew::TailwindSkew, 
    transform_base::TailwindTransformBase,
    transform_style::TailwindTransformStyle,
    transform_utility::TransformUtility,
    translate::TailwindTranslate,
};
use crate::{
    css_attributes, AnchorPoint, AxisXY, CssAttributes, Negative, NumericValue, Result, TailwindArbitrary, TailwindBuilder,
    TailwindInstance, UnitValue,
};
use std::fmt::{Debug, Display, Formatter};

mod origin;
mod perspective;
mod perspective_origin;
mod rotate;
mod scale;
mod skew;
mod transform_base;
mod transform_style;
mod transform_utility;
mod translate;

/// Helper function to generate the complete transform property
/// This ensures all transform utilities include the actual transform property
/// that uses the CSS variables set by individual utilities
fn get_transform_property() -> (&'static str, &'static str) {
    (
        "transform",
        "translate(var(--tw-translate-x, 0), var(--tw-translate-y, 0)) rotate(var(--tw-rotate, 0)) skewX(var(--tw-skew-x, 0)) skewY(var(--tw-skew-y, 0)) scaleX(var(--tw-scale-x, 1)) scaleY(var(--tw-scale-y, 1))"
    )
}
