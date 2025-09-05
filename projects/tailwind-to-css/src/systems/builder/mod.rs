use std::fmt::Debug;
use std::borrow::Cow;
use indexmap::IndexSet;

use crate::{systems::instruction::TailwindInstruction, *};

pub use self::base62::{Base62, BASE62};

mod base62;
mod methods;
mod setter;

///
#[derive(Debug)]
pub struct TailwindBuilder {
    ///
    pub preflight: PreflightSystem,
    /// All dynamic color properties
    ///
    /// Only determined when packing
    pub palettes: PaletteSystem,
    /// All dynamic break points
    ///
    /// Only determined when packing
    pub screens: BreakPointSystem,
    /// Container-specific breakpoints for container queries
    ///
    /// Separate from viewport breakpoints
    pub container_screens: ContainerBreakpointSystem,
    /// All dynamically registered font properties
    ///
    /// Only determined when packing
    pub fonts: FontSystem,
    /// All dynamically registered effect properties
    ///
    /// Only determined when packing
    pub effects: EffectSystem,
    pub(crate) objects: IndexSet<CssInstance>,
    pub(crate) bundles: IndexSet<CssBundle>,
}

impl TailwindBuilder {
    /// ## Trace mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn trace<'a>(&mut self, style: &'a str, obfuscate: bool) -> Result<Cow<'a, str>> {
        try_trace(self, style, obfuscate)
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn inline(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::Inline)?;
        Ok(out.as_inlined())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn scope(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::Scoped)?;
        Ok(out.as_scope())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn data_key(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::DataKey)?;
        Ok(out.as_dataset())
    }
    /// ## Inline mode
    ///
    ///
    /// # Returns
    /// **Not all instructions can be inline, if not, it will fall back to trace mode**
    ///
    /// - Anonymous style sheets, which can be placed inside `style` tags
    ///
    /// ## Example
    /// - input
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// ```
    /// - output
    /// ```html
    /// <div class="p-auto px-px pt-2 pb-2">Test</div>
    /// <style> {} </style>
    /// ```
    #[inline]
    pub fn data_value(&mut self, style: &str) -> Result<(String, String)> {
        let out = try_inline(self, style, CssInlineMode::DataValue)?;
        Ok(out.as_dataset())
    }
    /// Bundle all used stylesheets
    pub fn bundle(&self) -> Result<String> {
        let mut out = String::with_capacity(1024 * 10);
        if !self.preflight.disable {
            out.push_str(&self.preflight.to_string());
        }
        for item in &self.objects {
            item.write_css(&mut out, self)?;
        }
        for item in &self.bundles {
            item.write_css(&mut out, self)?;
        }
        Ok(out)
    }
}

fn parse_tailwind(input: &str) -> Result<Vec<TailwindInstruction>> {
    let styles = tailwind_ast::parse_tailwind(input)?;
    Ok(styles.into_iter().map(TailwindInstruction::from).collect())
}

/// Tokenize a class string respecting bracket boundaries for arbitrary values
fn tokenize_classes(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth -= 1;
                current.push(ch);
            }
            ' ' | '\t' | '\n' | '\r' if bracket_depth == 0 => {
                // Only split on whitespace when not inside brackets
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    // Don't forget the last token
    if !current.is_empty() {
        tokens.push(current);
    }
    
    tokens
}

fn try_trace<'a>(tw: &mut TailwindBuilder, style: &'a str, obfuscate: bool) -> Result<Cow<'a, str>> {
    let mut out = CssBundle::default();
    let mut has_transformations = false;
    
    // Preserve leading and trailing whitespace
    let trimmed = style.trim();
    let leading_ws = &style[..style.len() - style.trim_start().len()];
    let trailing_ws = &style[style.trim_end().len()..];
    
    // Tokenize input respecting bracket boundaries
    for class in tokenize_classes(trimmed) {
        // Skip empty strings
        if class.is_empty() {
            continue;
        }
        
        // Try to parse and transform this individual class
        match parse_tailwind(&class) {
            Ok(parsed) => {
                // Successfully parsed - transform it
                has_transformations = true;
                for item in parsed {
                    let variants = item.view_variants().to_vec();
                    match item.get_instance() {
                        Ok(instance) => {
                            let i = CssInstance::new(&*instance, tw, obfuscate, variants, item.is_important());
                            out.add_trace(&i);
                            tw.objects.insert(i);
                        }
                        Err(_) => {
                            // If we can't get the instance, preserve the original class string
                            // This is crucial for non-Tailwind strings like "react-dom/client"
                            // which parse successfully but aren't valid Tailwind classes
                            out.add_unparsed_class(&class);
                        }
                    }
                }
            }
            Err(_) => {
                // Can't parse this class - keep it as-is (it might be already transformed)
                out.add_unparsed_class(&class);
            }
        }
    }
    
    // If no transformations were made, return the original string unchanged
    if !has_transformations {
        Ok(Cow::Borrowed(style))
    } else {
        // Preserve original whitespace when reassembling
        let mut result = String::with_capacity(leading_ws.len() + out.as_traced().len() + trailing_ws.len());
        result.push_str(leading_ws);
        result.push_str(&out.as_traced());
        result.push_str(trailing_ws);
        Ok(Cow::Owned(result))
    }
}

fn try_inline(tw: &mut TailwindBuilder, style: &str, mode: CssInlineMode) -> Result<CssBundle> {
    let mut out = CssBundle::default();
    
    // Tokenize input respecting bracket boundaries
    for class in tokenize_classes(style) {
        // Skip empty strings
        if class.is_empty() {
            continue;
        }
        
        // Try to parse and transform this individual class
        match parse_tailwind(&class) {
            Ok(parsed) => {
                // Successfully parsed - transform it
                for item in parsed {
                    let variants = item.view_variants().to_vec();
                    match item.get_instance() {
                        Ok(instance) => {
                            // Don't obfuscate marker utilities (group/peer) - they need to keep their class names
                            let should_obfuscate = !matches!(class.as_str(), "group" | "peer");
                            let i = CssInstance::new(&*instance, tw, should_obfuscate, variants, item.is_important());
                            match &i.inlineable {
                                true => out.add_inline(i),
                                false => {
                                    out.add_trace(&i);
                                    tw.objects.insert(i);
                                },
                            };
                        }
                        Err(_) => {
                            // If we can't get the instance, preserve the original class string
                            // This is crucial for non-Tailwind strings like "react-dom/client"
                            // which parse successfully but aren't valid Tailwind classes
                            out.add_unparsed_class(&class);
                        }
                    }
                }
            }
            Err(_) => {
                // Can't parse this class - keep it as-is (it might be already transformed)
                out.add_unparsed_class(&class);
            }
        }
    }
    
    out.set_mode(mode);
    tw.bundles.insert(out.to_owned());
    Ok(out)
}
