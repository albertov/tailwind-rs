use super::*;
use crate::Base62;
use crate::systems::instruction::TailwindVariant;
use crate::systems::variants::VariantType;
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
    /// Gets the full class name, including variant prefixes when not obfuscated.
    /// 
    /// When obfuscated, returns a hash-based identifier.
    /// When not obfuscated, returns the full class name with variant prefixes
    /// (e.g., "sm:hover:bg-blue-500" for a class with sm and hover variants).
    pub fn get_class(&self) -> String {
        match self.obfuscate {
            true => Self::obfuscate(self),
            false => {
                // Reconstruct full class name with variant prefixes
                // Pre-allocate capacity: each variant adds its name + ':' separator
                let capacity = self.selector.len() 
                    + self.variants.iter()
                        .map(|v| v.to_class_prefix().len() + 1)
                        .sum::<usize>();
                
                let mut full_class = String::with_capacity(capacity);
                for variant in &self.variants {
                    full_class.push_str(&variant.to_class_prefix());
                    full_class.push(':');
                }
                full_class.push_str(&self.selector);
                full_class
            }
        }
    }
    /// Writes CSS output to the provided buffer, properly handling variants.
    ///
    /// This method generates CSS with proper nesting of media queries and pseudo-selectors.
    /// The order is critical: media queries wrap the entire rule, and pseudo-selectors
    /// are appended to the class selector.
    ///
    /// # Example output:
    /// ```css
    /// @media(min-width:640px) {
    ///   .sm\:hover\:bg-blue-500:hover { background-color: rgb(59 130 246); }
    /// }
    /// ```
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
                            // Custom breakpoint not found in configuration - silently ignore
                            // The variant will be handled as Unknown type
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
                    }
                    // Silently ignore truly unknown variants
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
        // Use get_class() which already includes variant prefixes
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::instruction::TailwindVariant;
    use crate::systems::css_global::attribute::CssAttributes;
    
    #[test]
    fn test_get_class_preserves_single_variant() {
        let instance = CssInstance {
            inlineable: false,
            selector: "bg-blue-500".to_string(),
            attribute: CssAttributes::default(),
            addition: String::new(),
            obfuscate: false,
            variants: vec![
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["hover".to_string()] 
                }
            ],
        };
        
        assert_eq!(instance.get_class(), "hover:bg-blue-500");
    }
    
    #[test]
    fn test_get_class_preserves_multiple_variants() {
        let instance = CssInstance {
            inlineable: false,
            selector: "text-white".to_string(),
            attribute: CssAttributes::default(),
            addition: String::new(),
            obfuscate: false,
            variants: vec![
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["sm".to_string()] 
                },
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["hover".to_string()] 
                },
            ],
        };
        
        assert_eq!(instance.get_class(), "sm:hover:text-white");
    }
    
    #[test]
    fn test_get_class_no_variants() {
        let instance = CssInstance {
            inlineable: false,
            selector: "p-4".to_string(),
            attribute: CssAttributes::default(),
            addition: String::new(),
            obfuscate: false,
            variants: vec![],
        };
        
        assert_eq!(instance.get_class(), "p-4");
    }
    
    #[test]
    fn test_get_class_complex_variant_names() {
        let instance = CssInstance {
            inlineable: false,
            selector: "font-bold".to_string(),
            attribute: CssAttributes::default(),
            addition: String::new(),
            obfuscate: false,
            variants: vec![
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["dark".to_string()] 
                },
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["first".to_string(), "child".to_string()] 
                },
            ],
        };
        
        assert_eq!(instance.get_class(), "dark:first-child:font-bold");
    }
}
