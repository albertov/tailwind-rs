use crate::{css_attributes, CssAttributes, Result, StandardValue, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

pub use self::{
    border_collapse::TailwindBorderCollapse, 
    border_spacing::TailwindBorderSpacing,
    caption_side::TailwindCaptionSide,
    table_layout::TailwindTableLayout
};

mod border_collapse;
mod border_spacing;
mod caption_side;
mod table_layout;
