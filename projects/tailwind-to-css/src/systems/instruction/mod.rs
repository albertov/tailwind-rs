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
    important: bool,
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
    pub modifier: Option<String>,
    pub container: bool,
    pub container_type: Option<tailwind_ast::ContainerQueryType>,
    pub has: bool,
    pub has_selector: Option<String>,
    pub arbitrary_selector: Option<String>,
}

impl TailwindVariant {
    /// Converts the variant to a class prefix string.
    /// 
    /// For container queries, adds the @ prefix and handles max/min modifiers.
    /// For other variants, joins names with hyphens.
    pub fn to_class_prefix(&self) -> String {
        if self.container {
            let mut prefix = String::from("@");
            
            // Add max- prefix for max-width queries
            if let Some(tailwind_ast::ContainerQueryType::Max) = self.container_type {
                prefix.push_str("max-");
            }
            
            // Add the breakpoint name or arbitrary value
            if let Some(name) = self.names.first() {
                // For arbitrary values, they're already in brackets like [123px]
                prefix.push_str(name);
            }
            
            // Add container name modifier if present
            if let Some(modifier) = &self.modifier {
                prefix.push('/');
                prefix.push_str(modifier);
            }
            
            prefix
        } else if self.has {
            // Handle has variants specially
            let base_name = if let Some(selector) = &self.has_selector {
                // Arbitrary has selector: has-[selector]
                if self.names.is_empty() {
                    format!("has-[{}]", selector)
                } else {
                    // Named has with arbitrary selector (e.g., group-has-[selector], peer-has-[selector])
                    let base = self.names.join("-");
                    format!("{}-has-[{}]", base, selector)
                }
            } else {
                // Named has selector (e.g., has-checked, group-has-focus)
                self.names.join("-")
            };
            
            // Add modifier if present (e.g., /sidebar)
            match &self.modifier {
                Some(modifier) => format!("{}/{}", base_name, modifier),
                None => base_name,
            }
        } else if (self.names.len() == 1 && (self.names[0] == "min" || self.names[0] == "max")) && self.arbitrary_selector.is_some() {
            // Handle arbitrary responsive breakpoints (min-[400px], max-[600px])
            let prefix = &self.names[0];
            let value = self.arbitrary_selector.as_ref().unwrap();
            format!("{}-[{}]", prefix, value)
        } else {
            // Standard variant handling
            let base = self.names.join("-");
            match &self.modifier {
                Some(modifier) => format!("{}/{}", base, modifier),
                None => base,
            }
        }
    }
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
