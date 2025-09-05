use super::*;

/// Handles combined font-size and line-height syntax like `text-sm/6` or `text-lg/tight`
#[derive(Debug, Clone)]
pub struct TailwindFontSizeWithLineHeight {
    font_size: UnitValue,
    line_height: LineHeight,
}

impl Display for TailwindFontSizeWithLineHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Format the display name based on font size and line height
        let size_part = match &self.font_size {
            UnitValue::Number{ n, .. } => format!("text-{}", n),
            UnitValue::Length(s) => format!("text-{}", s),
            UnitValue::Keyword(s) => format!("text-{}", s),
            UnitValue::Arbitrary(s) => format!("text-{}", s),
        };
        
        let height_part = match &self.line_height {
            LineHeight::Length(l) => format!("/{}", l.get_class_arbitrary()),
            LineHeight::Standard(s) => format!("/{}", s),
        };
        
        write!(f, "{}{}", size_part, height_part)
    }
}

impl TailwindInstance for TailwindFontSizeWithLineHeight {
    fn attributes(&self, _ctx: &TailwindBuilder) -> CssAttributes {
        // Get font-size CSS
        let font_size_css = match &self.font_size {
            UnitValue::Keyword(s) => {
                // Manually map keyword sizes to rem values
                format!("{}rem", match s.as_str() {
                    "xs" => 0.75,
                    "sm" => 0.875,
                    "base" | "md" => 1.0,
                    "lg" => 1.125,
                    "xl" => 1.25,
                    "2xl" => 1.5,
                    "3xl" => 1.875,
                    "4xl" => 2.25,
                    "5xl" => 3.0,
                    "6xl" => 3.75,
                    "7xl" => 4.5,
                    "8xl" => 6.0,
                    "9xl" => 8.0,
                    _ => 1.0,
                })
            }
            _ => self.font_size.get_properties_rem(),
        };
        
        // Get line-height CSS
        let line_height_css = match &self.line_height {
            LineHeight::Length(n) => n.get_properties(),
            LineHeight::Standard(g) => g.to_string(),
        };
        
        css_attributes! {
            "font-size" => font_size_css,
            "line-height" => line_height_css
        }
    }
}

impl TailwindFontSizeWithLineHeight {
    /// Parse a combined font-size/line-height for built-in sizes
    pub fn parse(size: &str, line_height: &str, _arbitrary: &TailwindArbitrary) -> Result<Self> {
        // Parse the font size (should be a valid keyword)
        let font_size = UnitValue::Keyword(size.to_string());
        
        // Parse the line height
        let line_height = Self::parse_line_height(line_height)?;
        
        Ok(Self { font_size, line_height })
    }
    
    /// Parse arbitrary font-size with line-height
    pub fn parse_arbitrary(font_arbitrary: &TailwindArbitrary, line_height: &str) -> Result<Self> {
        // Parse the arbitrary font size using the correct method
        let font_size = UnitValue::positive_parser("text-size", |_| false, true, false, false)(&[], font_arbitrary)?;
        
        // Parse the line height
        let line_height = Self::parse_line_height(line_height)?;
        
        Ok(Self { font_size, line_height })
    }
    
    /// Parse line-height value which can be:
    /// - Numeric (3, 4, 5, 6, 7, 8, 9, 10) - rem values  
    /// - Keywords (none, tight, snug, normal, relaxed, loose) - multipliers
    /// - Arbitrary values (already stripped of brackets by AST parser)
    fn parse_line_height(value: &str) -> Result<LineHeight> {
        // Check for keyword line-heights first
        match value {
            "none" => Ok(LineHeight::Length(LengthUnit::percent(100.0))),
            "tight" => Ok(LineHeight::Length(LengthUnit::percent(125.0))),
            "snug" => Ok(LineHeight::Length(LengthUnit::percent(137.5))),
            "normal" => Ok(LineHeight::Standard("normal".to_string())),
            "relaxed" => Ok(LineHeight::Length(LengthUnit::percent(162.5))),
            "loose" => Ok(LineHeight::Length(LengthUnit::percent(200.0))),
            // Numeric line-heights (3-10) - converted to rem
            "3" => Ok(LineHeight::Length(LengthUnit::rem(0.75))),   // 3 * 0.25rem
            "4" => Ok(LineHeight::Length(LengthUnit::rem(1.0))),    // 4 * 0.25rem
            "5" => Ok(LineHeight::Length(LengthUnit::rem(1.25))),   // 5 * 0.25rem
            "6" => Ok(LineHeight::Length(LengthUnit::rem(1.5))),    // 6 * 0.25rem
            "7" => Ok(LineHeight::Length(LengthUnit::rem(1.75))),   // 7 * 0.25rem
            "8" => Ok(LineHeight::Length(LengthUnit::rem(2.0))),    // 8 * 0.25rem
            "9" => Ok(LineHeight::Length(LengthUnit::rem(2.25))),   // 9 * 0.25rem
            "10" => Ok(LineHeight::Length(LengthUnit::rem(2.5))),   // 10 * 0.25rem
            // Try parsing as other values
            _ => {
                // Try to parse as a length value (e.g., "20px", "1.5rem")
                // Note: AST parser strips brackets from /[20px] giving us just "20px"
                if let Ok(length) = LengthUnit::parse_length(value) {
                    return Ok(LineHeight::Length(length));
                }
                // Check if it's a pure number (unitless line-height like "2" or "1.5")
                if let Ok(_num) = value.parse::<f32>() {
                    // Unitless numbers are treated as multipliers
                    return Ok(LineHeight::Standard(value.to_string()));
                }
                // Otherwise treat as an error since we don't know what this is
                syntax_error!("Unknown line-height value: {}", value)
            }
        }
    }
}

/// LineHeight enum for this module
#[derive(Debug, Clone)]
pub enum LineHeight {
    Length(LengthUnit),
    Standard(String),
}
