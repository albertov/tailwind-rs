use super::*;
use crate::Base62;
use crate::systems::instruction::TailwindVariant;
use crate::systems::variants::{VariantType, Breakpoint};
use crate::systems::builder::TailwindBuilder;
use std::fmt::Write;

mod traits;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Hash)]
pub(crate) struct CssInstance {
    pub inlineable: bool,
    pub obfuscate: bool,
    pub selector: String,
    pub attribute: CssAttributes,
    pub addition: String,
    pub variants: Vec<TailwindVariant>,
}

// noinspection DuplicatedCode
impl CssInstance {
    pub fn new(
        item: &dyn TailwindInstance, 
        ctx: &TailwindBuilder, 
        obfuscate: bool,
        variants: Vec<TailwindVariant>
    ) -> Self {
        Self {
            obfuscate,
            inlineable: item.inlineable(),
            selector: item.id(),
            attribute: item.attributes(ctx),
            addition: item.additional(ctx),
            variants,
        }
    }

    pub fn obfuscate(css: &Self) -> String {
        let mut hasher = Xxh3::new();
        css.attribute.hash(&mut hasher);
        css.addition.hash(&mut hasher);
        hasher.finish().base62()
    }
    pub fn get_class(&self) -> String {
        match self.obfuscate {
            true => Self::obfuscate(self),
            false => self.selector.to_string(),
        }
    }
    /// write css to buffers
    pub fn write_css(&self, f: &mut (dyn Write), tw: &TailwindBuilder) -> Result<()> {
        // Collect media queries and pseudo-selectors from variants
        let mut media_queries = Vec::new();
        let mut pseudo_selectors = Vec::new();
        
        for variant in &self.variants {
            match variant.get_type() {
                VariantType::Responsive(ref breakpoint) => {
                    // Look up breakpoint width from TailwindBuilder's configuration
                    match tw.screens.try_get_width(breakpoint.name()) {
                        Ok(width) => {
                            media_queries.push(format!("@media(min-width:{}px)", width));
                        },
                        Err(_) => {
                            // Custom breakpoint not found in configuration
                            if let Breakpoint::Custom(ref name) = breakpoint {
                                eprintln!("Warning: Custom breakpoint '{}' not found in configuration", name);
                            }
                        }
                    }
                },
                VariantType::PseudoClass(ref pseudo_class) => {
                    pseudo_selectors.push(format!(":{}", pseudo_class.as_str()));
                },
                VariantType::PseudoElement(ref pseudo_element) => {
                    pseudo_selectors.push(format!("::{}", pseudo_element.as_str()));
                },
                VariantType::Dark => {
                    media_queries.push("@media(prefers-color-scheme:dark)".to_string());
                },
                VariantType::State(ref state) => {
                    pseudo_selectors.push(format!(":{}", state.as_str()));
                },
                VariantType::Unknown(ref name) => {
                    // Check if it's a custom breakpoint registered in BreakPointSystem
                    if let Ok(width) = tw.screens.try_get_width(name) {
                        media_queries.push(format!("@media(min-width:{}px)", width));
                    } else {
                        eprintln!("Warning: Unknown variant '{}'", name);
                    }
                }
            }
        }
        
        // CSS generation order is critical:
        // Media queries must wrap the entire rule including pseudo-selectors
        // Correct: @media(min-width:640px) { .sm\:hover\:bg-blue-500:hover { ... } }
        // Wrong: .hover\:bg-blue-500:hover { @media(min-width:640px) { ... } }
        
        // Open media queries first (outermost wrappers)
        for query in &media_queries {
            writeln!(f, "{} {{", query)?;
        }
        
        // Proper indentation for nested CSS
        let indent = "  ".repeat(media_queries.len());
        
        // Write the CSS rule with proper selector
        write!(f, "{}", indent)?;
        f.write_char('.')?;
        if !self.obfuscate {
            // Include variant prefixes in non-obfuscated mode
            for variant in &self.variants {
                write!(f, "{}\\:", variant.to_class_prefix())?;
            }
        }
        normalize_class_name(f, &self.get_class())?;
        
        // Append pseudo-selectors AFTER the class name
        for pseudo in &pseudo_selectors {
            write!(f, "{}", pseudo)?;
        }
        
        // Write CSS properties
        f.write_str(" { ")?;
        write!(f, "{}", self.attribute)?;
        f.write_str(" }")?;
        
        // Close media queries in reverse order (innermost to outermost)
        for _ in &media_queries {
            writeln!(f)?;  // New line before closing brace
            f.write_char('}')?;
        }
        
        // Add any additional CSS
        if !self.addition.is_empty() {
            writeln!(f)?;
            write!(f, "{}", self.addition)?;
        }
        
        Ok(())
    }
}
