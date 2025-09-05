use super::*;
use crate::TailwindContent;

/// CSS Content property for pseudo-elements
/// Handles utilities like content-none, content-['text'], content-[attr(data-label)]
#[derive(Debug, Clone)]
pub struct TailwindPseudoContent {
    kind: ContentKind,
}

#[derive(Debug, Clone)]
enum ContentKind {
    None,
    Arbitrary(TailwindArbitrary),
}

impl Display for TailwindPseudoContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ContentKind::None => write!(f, "content-none"),
            ContentKind::Arbitrary(arb) => write!(f, "content-{}", arb.get_class()),
        }
    }
}

impl TailwindInstance for TailwindPseudoContent {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match &self.kind {
            ContentKind::None => css_attributes! {
                "--tw-content" => "none",
                "content" => "none"
            },
            ContentKind::Arbitrary(arb) => {
                // Use the raw value directly to preserve escaped underscores
                let raw_value = arb.as_str();
                let value = decode_content_value(raw_value);
                css_attributes! {
                    "--tw-content" => &value,
                    "content" => "var(--tw-content)"
                }
            }
        }
    }
}

impl TailwindPseudoContent {
    /// Create a content instance
    pub fn from(value: &str) -> Self {
        let kind = match value {
            "none" => ContentKind::None,
            _ => ContentKind::None, // Default to none for unknown values
        };
        Self { kind }
    }
    
    /// Convert to boxed trait object
    pub fn boxed(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }

    /// Parse content utilities
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] if arbitrary.is_some() => ContentKind::Arbitrary(arbitrary.to_owned()),
            ["none"] => ContentKind::None,
            _ => return syntax_error!("Unknown content pattern: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }

    /// Adapt method to handle content utilities
    pub fn adapt(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        // Check if this is actually a flexbox content-align utility
        if matches!(pattern, ["center" | "start" | "end" | "between" | "around" | "evenly"] | ["align", ..]) {
            // Delegate to flexbox content module
            return TailwindContent::adapt(pattern, arbitrary);
        }
        
        // Handle content-none and content-[...] patterns
        match pattern {
            ["none"] => Ok(Self::from("none").boxed()),
            [] if arbitrary.is_some() => Ok(Self::parse(pattern, arbitrary)?.boxed()),
            _ => {
                // Try delegating to flexbox content for other patterns
                TailwindContent::adapt(pattern, arbitrary)
            }
        }
    }
}

/// Decode content value from arbitrary string
/// Handles underscore to space conversion and preserves CSS functions
fn decode_content_value(input: &str) -> String {
    // Check if it's a CSS function (attr, url, counter, etc.)
    if input.starts_with("attr(") || 
       input.starts_with("url(") || 
       input.starts_with("counter(") ||
       input.starts_with("var(") ||
       input.starts_with("counters(") {
        // Return functions as-is
        return input.to_string();
    }
    
    // Check for special CSS keywords
    if matches!(input, "normal" | "none" | "initial" | "inherit" | "unset" | "revert") {
        return input.to_string();
    }
    
    // For string content, handle underscore conversion
    // First, handle escaped underscores
    let mut result = String::new();
    
    // Check if the input is quoted
    let is_quoted = (input.starts_with('"') && input.ends_with('"')) ||
                    (input.starts_with('\'') && input.ends_with('\''));
    
    if is_quoted {
        // If already quoted, preserve the quotes but still process underscores
        result.push(input.chars().next().unwrap());
        let inner = &input[1..input.len()-1];
        let processed = process_underscores(inner);
        result.push_str(&processed);
        result.push(input.chars().last().unwrap());
    } else {
        // Add quotes and process underscores
        result.push('\'');
        result.push_str(&process_underscores(input));
        result.push('\'');
    }
    
    result
}

fn process_underscores(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Check if next char is underscore
            if chars.peek() == Some(&'_') {
                chars.next(); // consume the underscore
                result.push('_'); // add literal underscore
            } else {
                result.push(ch); // preserve the backslash
            }
        } else if ch == '_' {
            result.push(' '); // convert underscore to space
        } else {
            result.push(ch);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_none() {
        let content = TailwindPseudoContent::from("none");
        assert_eq!(content.to_string(), "content-none");
        
        let builder = TailwindBuilder::default();
        let attrs = content.attributes(&builder);
        let css = attrs.to_string();
        assert!(css.contains("--tw-content:none"));
        assert!(css.contains("content:none"));
    }

    #[test]
    fn test_decode_content_value() {
        // Test string with underscores
        assert_eq!(decode_content_value("hello_world"), "'hello world'");
        
        // Test already quoted string (double quotes)
        assert_eq!(decode_content_value("\"hello_world\""), "\"hello world\"");
        
        // Test already quoted string (single quotes)
        assert_eq!(decode_content_value("'hello_world'"), "'hello world'");
        
        // Test escaped underscore
        assert_eq!(decode_content_value("hello\\_world"), "'hello_world'");
        
        // Test CSS functions
        assert_eq!(decode_content_value("attr(data-label)"), "attr(data-label)");
        assert_eq!(decode_content_value("url(/icon.svg)"), "url(/icon.svg)");
        assert_eq!(decode_content_value("counter(chapter)"), "counter(chapter)");
        assert_eq!(decode_content_value("var(--my-content)"), "var(--my-content)");
        
        // Test CSS keywords
        assert_eq!(decode_content_value("normal"), "normal");
        assert_eq!(decode_content_value("inherit"), "inherit");
    }

    #[test]
    fn test_process_underscores() {
        assert_eq!(process_underscores("hello_world"), "hello world");
        assert_eq!(process_underscores("__foo__"), "  foo  ");
        assert_eq!(process_underscores("test\\_underscore"), "test_underscore");
        assert_eq!(process_underscores("mixed_and\\_escaped"), "mixed and_escaped");
    }
}