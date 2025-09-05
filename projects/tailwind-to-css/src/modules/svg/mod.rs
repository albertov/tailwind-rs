pub use self::{
    fill::TailwindFillColor,
    fill_opacity::TailwindFillOpacity,
    stroke::{
        stroke_color::TailwindStrokeColor, 
        stroke_width::TailwindStrokeWidth,
        stroke_dasharray::TailwindStrokeDasharray,
        stroke_dashoffset::TailwindStrokeDashoffset,
        stroke_opacity::TailwindStrokeOpacity,
        stroke_linecap::TailwindStrokeLinecap,
        stroke_linejoin::TailwindStrokeLinejoin,
        TailwindStroke
    },
};
use crate::{
    css_attributes, CssAttributes, LengthUnit, NumericValue, Result, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindError, TailwindInstance,
};
use std::fmt::{Display, Formatter};

mod fill;
mod fill_opacity;
mod stroke;
