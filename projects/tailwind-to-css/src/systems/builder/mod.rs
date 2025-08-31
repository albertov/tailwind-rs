use std::fmt::Debug;
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
    pub fn trace(&mut self, style: &str, obfuscate: bool) -> Result<String> {
        let out = try_trace(self, style, obfuscate)?;
        Ok(out.as_traced())
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

fn try_trace(tw: &mut TailwindBuilder, style: &str, obfuscate: bool) -> Result<CssBundle> {
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
                            let i = CssInstance::new(&*instance, tw, obfuscate, variants);
                            out.add_trace(&i);
                            tw.objects.insert(i);
                        }
                        Err(_) => {
                            // If we can't get the instance, keep the original class
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
    
    Ok(out)
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
                            let i = CssInstance::new(&*instance, tw, true, variants);
                            match &i.inlineable {
                                true => out.add_inline(i),
                                false => {
                                    out.add_trace(&i);
                                    tw.objects.insert(i);
                                },
                            };
                        }
                        Err(_) => {
                            // If we can't get the instance, keep the original class
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
