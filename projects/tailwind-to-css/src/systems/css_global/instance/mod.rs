use super::*;
use crate::Base62;
use crate::systems::instruction::TailwindVariant;
use crate::systems::variants::VariantType;
use crate::systems::builder::TailwindBuilder;
use std::fmt::Write;

mod traits;

/// Processes a has-selector to generate the correct CSS selector.
/// 
/// # Arguments
/// * `selector` - The selector to process (e.g., "checked", ">img", ".error")
/// * `arbitrary` - Whether this is an arbitrary selector from brackets
/// 
/// # Returns
/// The processed CSS selector ready for use in :has()
fn process_has_selector(selector: &str, arbitrary: bool) -> String {
    if arbitrary {
        // Handle arbitrary selectors
        let processed = selector
            .replace('&', "*")  // Replace & with *
            .trim()
            .to_string();
        
        // Handle relative selectors - preserve spacing
        if processed.starts_with('>') 
            || processed.starts_with('+') 
            || processed.starts_with('~') {
            format!(" {}", processed)
        } else {
            processed
        }
    } else {
        // Standard pseudo-class or selector
        // Check if it already starts with a colon (for complex selectors)
        if selector.starts_with(':') {
            selector.to_string()
        } else {
            // Add colon for standard pseudo-classes
            format!(":{}", selector)
        }
    }
}

/// Escapes CSS identifier characters for use in class names.
/// 
/// # Arguments
/// * `s` - The string to escape
/// 
/// # Returns
/// The escaped string suitable for CSS identifiers
fn escape_css(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => result.push(c),
            _ => {
                result.push('\\');
                result.push(c);
            }
        }
    }
    result
}

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
        variants: Vec<TailwindVariant>,
        important: bool
    ) -> Self {
        let mut attributes = item.attributes(ctx);
        // If important flag is set, mark all attributes as important
        if important {
            attributes = Self::make_important(attributes);
        }
        Self {
            obfuscate,
            inlineable: item.inlineable(),
            selector: item.id(),
            attribute: attributes,
            addition: item.additional(ctx),
            variants,
        }
    }

    fn make_important(attrs: CssAttributes) -> CssAttributes {
        // Create a new CssAttributes with all properties marked as important
        let mut important_attrs = CssAttributes::default();
        
        // This is a bit of a hack - we need to extract and re-insert with important flag
        // Since CssAttributes doesn't expose its internal fields, we'll use the Display trait
        // to get the CSS string and parse it back
        let css_str = attrs.to_string();
        
        // Parse the CSS string and re-insert with important flag
        for prop in css_str.split(';') {
            let prop = prop.trim();
            if prop.is_empty() {
                continue;
            }
            
            // Remove any existing !important
            let prop = prop.replace("!important", "").trim().to_string();
            
            if let Some(colon_pos) = prop.find(':') {
                let key = prop[..colon_pos].trim();
                let value = prop[colon_pos + 1..].trim();
                important_attrs.insert_important(key, value);
            }
        }
        
        important_attrs
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
    /// @media (min-width: 640px) {
    ///   .sm\:hover\:bg-blue-500:hover { background-color: rgb(59 130 246); }
    /// }
    /// ```
    pub fn write_css(&self, f: &mut (dyn Write), tw: &TailwindBuilder) -> Result<()> {
        // Collect media queries, container queries, and selectors from variants
        let mut media_queries = Vec::new();
        let mut container_queries = Vec::new();
        let mut pseudo_selectors = Vec::new();
        let mut group_selector: Option<(String, String)> = None;
        let mut peer_selector: Option<(String, String)> = None;
        let mut has_selectors: Vec<String> = Vec::new();
        let mut group_has_selector: Option<(String, String)> = None;
        let mut peer_has_selector: Option<(String, String)> = None;
        
        for variant in &self.variants {
            match variant.get_type() {
                VariantType::Responsive(ref breakpoint) => {
                    // Look up breakpoint width from TailwindBuilder's configuration
                    match tw.screens.try_get_width(breakpoint.name()) {
                        Ok(width) => {
                            media_queries.push(format!("@media (min-width: {}px)", width));
                        },
                        Err(_) => {
                            // Custom breakpoint not found in configuration - silently ignore
                            // The variant will be handled as Unknown type
                        }
                    }
                },
                VariantType::PseudoClass(ref pseudo_class) => {
                    // Wrap hover variants in hover media query for better accessibility
                    if pseudo_class.as_str() == "hover" {
                        media_queries.push("@media (hover: hover)".to_string());
                    }
                    pseudo_selectors.push(format!(":{}", pseudo_class.as_str()));
                },
                VariantType::PseudoElement(ref pseudo_element) => {
                    pseudo_selectors.push(format!("::{}", pseudo_element.as_str()));
                },
                VariantType::Dark => {
                    media_queries.push("@media (prefers-color-scheme: dark)".to_string());
                },
                VariantType::State(ref state) => {
                    pseudo_selectors.push(format!(":{}", state.as_str()));
                },
                VariantType::Group { ref state, ref modifier, ref arbitrary_selector } => {
                    // Wrap hover variants in hover media query
                    if state == "hover" {
                        media_queries.push("@media (hover: hover)".to_string());
                    }
                    
                    let group_class = match modifier {
                        Some(mod_name) => format!("group\\/{}", mod_name),
                        None => "group".to_string(),
                    };
                    
                    // Handle arbitrary state selectors
                    let state_selector = if let Some(arb_sel) = arbitrary_selector {
                        // Use the arbitrary selector directly
                        arb_sel.clone()
                    } else if state.starts_with('[') && state.ends_with(']') {
                        // Arbitrary selector like [data-state="open"]
                        state.to_string()
                    } else if !state.is_empty() {
                        // Standard pseudo-class like hover, focus
                        format!(":{}", state)
                    } else {
                        // No state, just arbitrary selector
                        String::new()
                    };
                    
                    group_selector = Some((group_class, state_selector));
                },
                VariantType::Peer { ref state, ref modifier, ref arbitrary_selector } => {
                    // Wrap hover variants in hover media query
                    if state == "hover" {
                        media_queries.push("@media (hover: hover)".to_string());
                    }
                    
                    let peer_class = match modifier {
                        Some(mod_name) => format!("peer\\/{}", mod_name),
                        None => "peer".to_string(),
                    };
                    
                    // Handle arbitrary state selectors
                    let state_selector = if let Some(arb_sel) = arbitrary_selector {
                        // Use the arbitrary selector directly
                        arb_sel.clone()
                    } else if state.starts_with('[') && state.ends_with(']') {
                        // Arbitrary selector like [data-state="open"]
                        state.to_string()
                    } else if !state.is_empty() {
                        // Standard pseudo-class like hover, focus, checked
                        format!(":{}", state)
                    } else {
                        // No state, just arbitrary selector
                        String::new()
                    };
                    
                    peer_selector = Some((peer_class, state_selector));
                },
                VariantType::ArbitraryResponsive { is_max, ref value } => {
                    // Generate media query for arbitrary breakpoints
                    let query = if is_max {
                        format!("@media (max-width: {})", value)
                    } else {
                        format!("@media (min-width: {})", value)
                    };
                    media_queries.push(query);
                },
                VariantType::Unknown(ref name) => {
                    // Check if it's a custom breakpoint registered in BreakPointSystem
                    if let Ok(width) = tw.screens.try_get_width(name) {
                        media_queries.push(format!("@media (min-width: {}px)", width));
                    }
                    // Silently ignore truly unknown variants
                },
                VariantType::Has { ref selector, arbitrary, negated } => {
                    // Store the processed selector for later use
                    let mut sel = process_has_selector(selector, arbitrary);
                    if negated {
                        // Wrap in :not() for negated has selectors
                        sel = format!(":not({})", sel);
                    }
                    has_selectors.push(sel);
                },
                VariantType::GroupHas { ref selector, ref modifier, arbitrary, negated } => {
                    let group_class = match modifier {
                        Some(mod_name) => format!("group\\/{}", escape_css(mod_name)),
                        None => "group".to_string(),
                    };
                    
                    let mut has_sel = process_has_selector(selector, arbitrary);
                    if negated {
                        has_sel = format!(":not({})", has_sel);
                    }
                    group_has_selector = Some((group_class, has_sel));
                },
                VariantType::PeerHas { ref selector, ref modifier, arbitrary, negated } => {
                    let peer_class = match modifier {
                        Some(mod_name) => format!("peer\\/{}", escape_css(mod_name)),
                        None => "peer".to_string(),
                    };
                    
                    let mut has_sel = process_has_selector(selector, arbitrary);
                    if negated {
                        has_sel = format!(":not({})", has_sel);
                    }
                    peer_has_selector = Some((peer_class, has_sel));
                },
                VariantType::ContainerQuery { ref query_type, ref breakpoint, ref container_name } => {
                    // Generate @container rule based on query type
                    let condition = match query_type {
                        crate::systems::variants::ContainerQueryType::MinWidth => {
                            // Try to resolve from container breakpoint system
                            match tw.container_screens.get(breakpoint) {
                                Ok(value) => format!("(min-width: {})", value),
                                Err(_) => {
                                    // Fall back to CSS variable for undefined breakpoints
                                    format!("(min-width: {})", tw.container_screens.get_css_var(breakpoint))
                                }
                            }
                        },
                        crate::systems::variants::ContainerQueryType::MaxWidth => {
                            // For max-width queries
                            match tw.container_screens.get(breakpoint) {
                                Ok(value) => format!("(max-width: {})", value),
                                Err(_) => {
                                    // Fall back to CSS variable for undefined breakpoints
                                    format!("(max-width: {})", tw.container_screens.get_css_var(breakpoint))
                                }
                            }
                        },
                        crate::systems::variants::ContainerQueryType::Arbitrary => {
                            // Use the arbitrary value directly
                            // Assume it's a min-width query if not specified otherwise
                            format!("(min-width: {})", breakpoint)
                        },
                    };
                    
                    // Build the container query rule with optional container name
                    let container_rule = match container_name {
                        Some(name) => format!("@container {} {}", name, condition),
                        None => format!("@container {}", condition),
                    };
                    
                    container_queries.push(container_rule);
                }
            }
        }
        
        // CSS generation order is critical:
        // Media queries and container queries must wrap the entire rule including pseudo-selectors
        // Container queries can be nested inside media queries
        
        // Open media queries first (outermost wrappers)
        for query in &media_queries {
            writeln!(f, "{} {{", query)?;
        }
        
        // Then open container queries (can be nested inside media queries)
        for query in &container_queries {
            writeln!(f, "{} {{", query)?;
        }
        
        // Proper indentation for nested CSS
        let indent = "  ".repeat(media_queries.len() + container_queries.len());
        
        // Write the CSS rule with proper selector
        write!(f, "{}", indent)?;
        
        // Handle has/group/peer selectors with appropriate patterns
        if !has_selectors.is_empty() {
            // Basic has: .has-checked\:bg-green:has(:checked)
            // Multiple has: .has-checked\:has-focus\:bg-green:has(:checked):has(:focus)
            f.write_char('.')?;
            normalize_class_name(f, &self.get_class())?;
            for selector in &has_selectors {
                write!(f, ":has({})", selector)?;
            }
        } else if let Some((group_class, has_sel)) = group_has_selector {
            // Group-has: .group-has-checked\:bg-green:is(:where(.group):has(:checked) *)
            f.write_char('.')?;
            normalize_class_name(f, &self.get_class())?;
            write!(f, ":is(:where(.{}):has({}) *)", group_class, has_sel)?;
        } else if let Some((peer_class, has_sel)) = peer_has_selector {
            // Peer-has: .peer-has-checked\:bg-green:is(:where(.peer):has(:checked) ~ *)
            f.write_char('.')?;
            normalize_class_name(f, &self.get_class())?;
            write!(f, ":is(:where(.{}):has({}) ~ *)", peer_class, has_sel)?;
        } else if let Some((group_class, state_selector)) = group_selector {
            // Group pattern: .group:hover .group-hover\:flex
            write!(f, ".{}{} .", group_class, state_selector)?;
            normalize_class_name(f, &self.get_class())?;
        } else if let Some((peer_class, state_selector)) = peer_selector {
            // Peer pattern: .peer:checked ~ .peer-checked\:opacity-100
            write!(f, ".{}{} ~ .", peer_class, state_selector)?;
            normalize_class_name(f, &self.get_class())?;
        } else {
            // Standard selector
            f.write_char('.')?;
            normalize_class_name(f, &self.get_class())?;
            
            // Append pseudo-selectors AFTER the class name
            for pseudo in &pseudo_selectors {
                write!(f, "{}", pseudo)?;
            }
        }
        
        // Write CSS properties
        let attr_str = self.attribute.to_string();
        if attr_str.is_empty() {
            f.write_str(" { }")?;
        } else {
            f.write_str(" { ")?;
            write!(f, "{}", self.attribute)?;
            f.write_str(" }")?;
        }
        
        // Close container queries first (innermost)
        for _ in &container_queries {
            writeln!(f)?;  // New line before closing brace
            f.write_char('}')?;
        }
        
        // Then close media queries (outermost)
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
                    names: vec!["hover".to_string()],
                    modifier: None,
                    container: false,
                    container_type: None,
                    has: false,
                    has_selector: None,
                    arbitrary_selector: None,
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
                    names: vec!["sm".to_string()],
                    modifier: None,
                    container: false,
                    container_type: None,
                    has: false,
                    has_selector: None,
                    arbitrary_selector: None,
                },
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["hover".to_string()],
                    modifier: None,
                    container: false,
                    container_type: None,
                    has: false,
                    has_selector: None,
                    arbitrary_selector: None,
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
                    names: vec!["dark".to_string()],
                    modifier: None,
                    container: false,
                    container_type: None,
                    has: false,
                    has_selector: None,
                    arbitrary_selector: None,
                },
                TailwindVariant { 
                    not: false,
                    pseudo: false,
                    names: vec!["first".to_string(), "child".to_string()],
                    modifier: None,
                    container: false,
                    container_type: None,
                    has: false,
                    has_selector: None,
                    arbitrary_selector: None,
                },
            ],
        };
        
        assert_eq!(instance.get_class(), "dark:first-child:font-bold");
    }
}
