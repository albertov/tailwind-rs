mod arbitrary;
mod display;
mod methods;
mod resolver;
pub use self::arbitrary::TailwindArbitrary;
use crate::{TailwindBuilder, *};
use css_color::Srgb;
use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};
use tailwind_ast::{parse_fraction, ASTVariant, AstStyle};

/// `v:v:-a-a-[A]/opacity`
#[derive(Debug, Clone)]
pub struct TailwindInstruction {
    negative: Negative,
    variants: Vec<TailwindVariant>,
    elements: TailwindElements,
    arbitrary: TailwindArbitrary,
    opacity: Option<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TailwindVariant {
    pub not: bool,
    pub pseudo: bool,
    pub names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TailwindElements {
    inner: Vec<String>,
}

/// <https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts>
#[derive(Copy, Clone, Debug)]
pub enum TailwindVariantKind {
    Dark,
    Sticky,
    MotionReduce,
    MotionSafe,
    First,
    Last,
    Even,
    Odd,
    Children,
    Siblings,
    Sibling,
    Override,
}
