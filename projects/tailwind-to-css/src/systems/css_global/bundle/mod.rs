use super::*;
use crate::Base62;
use indexmap::IndexSet;
mod traits;

/// A collection of css objects
///
/// Separate or merge as needed
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(crate) struct CssBundle {
    mode: CssInlineMode,
    non_inlined_classes: IndexSet<String>,
    attribute: CssAttributes,
    addition: IndexSet<String>,
}

// noinspection DuplicatedCode
impl CssBundle {
    pub fn add_trace(&mut self, item: &CssInstance) {
        self.non_inlined_classes.insert(item.get_class());
    }
    /// Add an unparsed class name directly (for classes that can't be parsed)
    pub fn add_unparsed_class(&mut self, class: &str) {
        self.non_inlined_classes.insert(class.to_string());
    }
    /// insert new css instance to the html tag
    pub fn add_inline(&mut self, item: CssInstance) {
        self.attribute += item.attribute;
        self.addition.insert(item.addition);
    }
    pub fn obfuscate(css: &Self) -> String {
        let mut hasher = Xxh3::new();
        css.attribute.hash(&mut hasher);
        // Hash the items in the IndexSet in order
        for item in &css.addition {
            item.hash(&mut hasher);
        }
        hasher.finish().base62()
    }
    /// # Returns
    /// - css classes
    pub fn as_traced(&self) -> String {
        self.non_inlined_classes.iter().join(" ")
    }
    /// # Returns
    /// - `.0`: css classes rest, maybe empty
    /// - `.1`: css style
    pub fn as_inlined(&self) -> (String, String) {
        (self.as_traced(), self.attribute.to_string())
    }
    /// # Returns
    /// - scoped class name
    pub fn as_scope(&self) -> (String, String) {
        let id = Self::obfuscate(self);
        (self.as_traced(), id)
    }
    /// # Returns
    /// - data name without value
    pub fn as_dataset(&self) -> (String, String) {
        let id = Self::obfuscate(self);
        (self.as_traced(), id)
    }
    pub fn set_mode(&mut self, mode: CssInlineMode) {
        self.mode = mode
    }
    pub fn write_css(&self, f: &mut (dyn Write), _tw: &crate::TailwindBuilder) -> Result<()> {
        let id = Self::obfuscate(self);
        match self.mode {
            CssInlineMode::None => unreachable!(),
            CssInlineMode::Inline => return Ok(()),
            CssInlineMode::Scoped => write!(f, ".{}", id)?,
            CssInlineMode::DataKey => write!(f, "[data-tw-{}]", id)?,
            CssInlineMode::DataValue => write!(f, "[data-tw=\"{}\"]", id)?,
        }
        f.write_char('{')?;
        write!(f, "{}", self.attribute)?;
        f.write_char('}')?;
        for item in &self.addition {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Inlined {
    pub class: String,
    pub style: String,
}
