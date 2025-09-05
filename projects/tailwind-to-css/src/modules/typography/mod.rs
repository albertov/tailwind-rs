pub(crate) use self::list::list_adaptor;
pub use self::{
    align::TailwindAlign,
    breaking::TailwindBreak,
    content::TailwindPseudoContent,
    decoration::{
        color::TailwindDecorationColor, line::TailwindDecorationLine, style::TailwindDecorationStyle,
        thickness::TailwindDecorationThickness, TailwindDecoration,
    },
    font::{
        font_adaptor, font_family::TailwindFontFamily, font_size::TailwindFontSize, font_size_with_line_height::TailwindFontSizeWithLineHeight,
        font_smoothing::TailwindFontSmoothing, font_style::TailwindFontStyle, font_variant_numeric::TailwindFontVariantNumeric, 
        font_weight::TailwindFontWeight,
    },
    hyphens::TailwindHyphens,
    indent::TailwindIndent,
    leading::TailwindLeading,
    line_clamp::TailwindLineClamp,
    list::{list_position::TailwindListPosition, list_type::TailwindListStyle},
    prose::TailwindProse,
    text::{
        text_adaptor, text_align::TailwindTextAlignment, text_color::TailwindTextColor, text_overflow::TailwindTextOverflow,
        text_transform::TailwindTextTransform, text_wrap::TailwindTextWrap,
    },
    tracking::TailwindTracking,
    underline_offset::TailwindUnderlineOffset,
    whitespace::TailwindWhiteSpace,
};
use crate::{
    css_attributes, syntax_error, CssAttributes, LengthUnit, Result, StandardValue, TailwindArbitrary, TailwindBreakAfter,
    TailwindBreakBefore, TailwindBreakInside, TailwindBuilder, TailwindColor, TailwindInstance, UnitValue
};
use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

mod align;
mod breaking;
mod content;
mod decoration;
mod font;
mod hyphens;
mod indent;
mod leading;
mod line_clamp;
mod list;
mod prose;
mod text;
mod tracking;
mod underline_offset;
mod whitespace;
