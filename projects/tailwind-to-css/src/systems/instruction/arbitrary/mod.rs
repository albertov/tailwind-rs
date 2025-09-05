use super::*;
use std::fmt::Write;

mod methods;

/// Process underscores in arbitrary values
/// Converts underscores to spaces, but preserves them in calc() operators
fn process_underscores(value: &str) -> String {
    // Handle calc() and similar CSS functions specially, including modern color spaces
    if value.starts_with("calc(") || value.starts_with("clamp(") || 
       value.starts_with("min(") || value.starts_with("max(") || value.starts_with("var(") ||
       value.starts_with("oklch(") || value.starts_with("lch(") || 
       value.starts_with("lab(") || value.starts_with("oklab(") ||
       value.starts_with("hwb(") || value.starts_with("color(") {
        // For calc expressions, we need to be smart about underscores
        // "_+_" should become " + ", "_-_" should become " - ", etc.
        let mut result = String::with_capacity(value.len());
        let chars: Vec<char> = value.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            if chars[i] == '_' {
                // Check if this is an operator pattern like "_+_", "_-_", "_*_", "_/_"
                let prev_is_not_underscore = i == 0 || chars[i-1] != '_';
                let next_is_operator = i + 1 < chars.len() && 
                    (chars[i+1] == '+' || chars[i+1] == '-' || chars[i+1] == '*' || chars[i+1] == '/');
                let operator_has_trailing_underscore = i + 2 < chars.len() && chars[i+2] == '_';
                
                if prev_is_not_underscore && next_is_operator && operator_has_trailing_underscore {
                    // This is an operator pattern like "_+_"
                    result.push(' ');
                    result.push(chars[i+1]);
                    result.push(' ');
                    i += 3;
                    continue;
                }
            }
            
            // Regular underscore to space conversion
            if chars[i] == '_' {
                result.push(' ');
            } else {
                result.push(chars[i]);
            }
            i += 1;
        }
        
        result
    } else {
        // For non-calc values, simple underscore to space conversion
        value.replace('_', " ")
    }
}

#[derive(Debug, Clone)]
pub struct TailwindArbitrary {
    inner: Box<str>,
}

impl Display for TailwindArbitrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for c in self.inner.chars() {
            match c {
                ' ' => f.write_char('_')?,
                _ => f.write_char(c)?,
            }
        }
        f.write_char(']')
    }
}

impl From<&str> for TailwindArbitrary {
    fn from(s: &str) -> Self {
        Self { inner: Box::from(s) }
    }
}

impl From<&Self> for TailwindArbitrary {
    fn from(s: &Self) -> Self {
        Self { inner: s.inner.clone() }
    }
}

impl TailwindArbitrary {
    pub fn new<T>(s: T) -> Result<Self>
    where
        T: Into<Self>,
    {
        let out = s.into();
        if cfg!(feature = "compile_time") {
            if out.inner.is_empty() {
                return Err(TailwindError::syntax_error("Arbitrary value cannot be empty"));
            }
            // TODO: Check unbalanced quotes
            if out.inner.contains('\n') {
                return Err(TailwindError::syntax_error("Arbitrary value does balance quotes"));
            }
        }
        Ok(out)
    }

    pub fn get_class(&self) -> String {
        let mut class = String::with_capacity(self.inner.len() + 2);
        class.push('[');
        for c in self.inner.chars() {
            match c {
                ' ' => class.push('_'),
                _ => class.push(c),
            }
        }
        class.push(']');
        class
    }
    pub fn write(&self, f: &mut Formatter) -> std::fmt::Result {
        self.write_class(f, "")
    }
    pub fn write_class(&self, f: &mut Formatter, before: &str) -> std::fmt::Result {
        write!(f, "{}{}", before, self.get_class())
    }
    pub fn get_properties(&self) -> String {
        // Process the arbitrary value for CSS output
        // The inner value is stored WITHOUT brackets (e.g., "4" for order-[4])
        // The brackets are already stripped during parsing
        let value = self.inner.as_ref();
        
        // Simply process underscores (convert to spaces, handle calc operators)
        process_underscores(value)
    }
}
