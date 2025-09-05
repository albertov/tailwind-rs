use super::*;

mod traits;

///
#[derive(Clone, Debug)]
pub enum TailwindColor {
    RGB(Srgb),
    Themed(String, u32),
    ThemedWithOpacity(String, u32, f32),
    Keyword(String),
    Arbitrary(TailwindArbitrary),
    /// Modern color space values passed through as-is
    ModernColorSpace(String),
}

impl Display for TailwindColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RGB(c) => {
                // Check if this matches a known color constant
                if c.red == 0.0 && c.green == 0.0 && c.blue == 0.0 && c.alpha == 1.0 {
                    write!(f, "black")
                } else if c.red == 1.0 && c.green == 1.0 && c.blue == 1.0 && c.alpha == 1.0 {
                    write!(f, "white")
                } else {
                    // For other RGB colors, use bracketed format for arbitrary values
                    write!(
                        f,
                        "[#{:02X}{:02X}{:02X}{:02X}]",
                        (255.0 * c.red) as u8,
                        (255.0 * c.green) as u8,
                        (255.0 * c.blue) as u8,
                        (255.0 * c.alpha) as u8
                    )
                }
            },
            Self::Themed(name, weight) => write!(f, "{}-{}", name, weight),
            Self::ThemedWithOpacity(name, weight, opacity) => write!(f, "{}-{}/{}", name, weight, (opacity * 100.0) as u32),
            Self::Arbitrary(a) => a.write(f),
            Self::Keyword(s) => match s.as_str() {
                "transparent" => write!(f, "transparent"),
                "current" => write!(f, "current"),
                _ => write!(f, "{}", s),
            },
            Self::ModernColorSpace(s) => write!(f, "[{}]", s.replace(' ', "_")),
        }
    }
}

#[allow(non_upper_case_globals)]
impl TailwindColor {
    /// `black`
    pub const Black: Self = Self::RGB(Srgb { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 });
    /// `white`
    pub const White: Self = Self::RGB(Srgb { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 });
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            ["none"] | ["transparent"] => Self::from("transparent"),
            ["black"] => Self::Black,
            ["white"] => Self::White,
            [s @ ("current" | "inherit" | "initial" | "unset")] => Self::from(*s),
            [] => Self::parse_arbitrary(arbitrary)?,
            [name, weight] => Self::parse_themed(name, weight)?,
            _ => return syntax_error!("Unknown color pattern: {}", pattern.join("-")),
        };
        Ok(out)
    }
    #[inline]
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<TailwindColor> {
        let value = arbitrary.as_str();
        
        // Check if this is a modern color space
        if value.starts_with("oklch(") || value.starts_with("lch(") || 
           value.starts_with("color(display-p3") || value.starts_with("color(rec2020") ||
           value.starts_with("lab(") || value.starts_with("oklab(") ||
           value.starts_with("hwb(") {
            // Pass through modern color space values as-is
            return Ok(Self::ModernColorSpace(value.to_string()));
        }
        
        // Try to parse as regular color, if that fails check if it might be a color space syntax
        match arbitrary.as_color() {
            Ok(srgb) => Ok(Self::RGB(srgb)),
            Err(_) => {
                // Fallback: check if this looks like a color function that we should pass through
                if value.contains('(') && value.contains(')') {
                    // Could be a color function we don't recognize yet
                    Ok(Self::ModernColorSpace(value.to_string()))
                } else {
                    // Not a color at all
                    syntax_error!("Invalid color value: {}", value)
                }
            }
        }
    }
    ///
    #[inline]
    pub fn parse_themed(name: &str, weight: &str) -> Result<TailwindColor> {
        let name = name.to_string();
        let weight = TailwindArbitrary::from(weight).as_integer()? as u32;
        Ok(Self::Themed(name, weight))
    }
    /// get class of `<color>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    #[inline]
    pub fn get_class(&self) -> String {
        self.to_string()
    }
    /// get properties of `<color>`
    ///
    /// - https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
    #[inline]
    pub fn get_properties(&self, ctx: &TailwindBuilder) -> String {
        match self {
            Self::RGB(c) => {
                // Use modern rgb() syntax with space-separated values
                if c.alpha >= 0.999 {
                    // No alpha channel needed
                    format!("rgb({} {} {})", 
                        (255.0 * c.red).round() as u8, 
                        (255.0 * c.green).round() as u8, 
                        (255.0 * c.blue).round() as u8)
                } else {
                    // Include alpha channel with / separator
                    format!("rgb({} {} {} / {})", 
                        (255.0 * c.red).round() as u8, 
                        (255.0 * c.green).round() as u8, 
                        (255.0 * c.blue).round() as u8, 
                        c.alpha)
                }
            },
            Self::Arbitrary(a) => a.get_properties(),
            Self::Keyword(s) => match s.as_str() {
                "transparent" => "transparent".to_string(),
                "current" => "currentColor".to_string(),
                _ => s.to_string(),
            },
            Self::ModernColorSpace(s) => {
                // Modern color spaces are stored with underscores, convert back to spaces
                s.replace('_', " ")
            },
            Self::Themed(name, weight) => match ctx.palettes.try_get_color(name, *weight) {
                Ok(c) => {
                    // Use modern rgb() syntax
                    if c.alpha >= 0.999 {
                        format!("rgb({} {} {})", 
                            (255.0 * c.red).round() as u8, 
                            (255.0 * c.green).round() as u8, 
                            (255.0 * c.blue).round() as u8)
                    } else {
                        format!("rgb({} {} {} / {})", 
                            (255.0 * c.red).round() as u8, 
                            (255.0 * c.green).round() as u8, 
                            (255.0 * c.blue).round() as u8, 
                            c.alpha)
                    }
                },
                Err(_) => "currentColor".to_string(),
            },
            Self::ThemedWithOpacity(name, weight, opacity) => match ctx.palettes.try_get_color(name, *weight) {
                Ok(c) => {
                    // Always include opacity when explicitly set
                    format!("rgb({} {} {} / {})", 
                        (255.0 * c.red).round() as u8, 
                        (255.0 * c.green).round() as u8, 
                        (255.0 * c.blue).round() as u8, 
                        opacity)
                },
                Err(_) => "currentColor".to_string(),
            },
        }
    }
    
    /// Parse color with optional opacity modifier
    pub fn parse_with_opacity(pattern: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Self> {
        let mut color = Self::parse(pattern, arbitrary)?;
        
        // Apply opacity if provided
        if let Some(opacity_str) = opacity {
            color = color.with_opacity(opacity_str)?;
        }
        
        Ok(color)
    }
    
    /// Apply opacity to a color
    pub fn with_opacity(self, opacity_str: &str) -> Result<Self> {
        // Parse opacity value
        let opacity_value = if opacity_str.starts_with('[') && opacity_str.ends_with(']') {
            // Arbitrary value like 0.23 (without brackets as they're already stripped)
            opacity_str.parse::<f32>()?
        } else {
            // Predefined value like 50
            let percentage = opacity_str.parse::<f32>()?;
            percentage / 100.0
        };
        
        // Clamp opacity to 0..1 range
        let opacity_value = opacity_value.clamp(0.0, 1.0);
        
        match self {
            Self::RGB(mut c) => {
                c.alpha = opacity_value;
                Ok(Self::RGB(c))
            }
            Self::Themed(name, weight) => {
                // Create a new ThemedWithOpacity variant
                Ok(Self::ThemedWithOpacity(name, weight, opacity_value))
            }
            Self::ThemedWithOpacity(name, weight, _) => {
                // Update existing opacity
                Ok(Self::ThemedWithOpacity(name, weight, opacity_value))
            }
            Self::Keyword(s) if s == "black" => {
                Ok(Self::RGB(Srgb { red: 0.0, green: 0.0, blue: 0.0, alpha: opacity_value }))
            }
            Self::Keyword(s) if s == "white" => {
                Ok(Self::RGB(Srgb { red: 1.0, green: 1.0, blue: 1.0, alpha: opacity_value }))
            }
            Self::ModernColorSpace(s) => {
                // Add opacity to modern color space by wrapping in color-mix
                if opacity_value < 0.999 {
                    let mixed = format!("color-mix(in srgb, {} {}%, transparent)", s, (opacity_value * 100.0) as u32);
                    Ok(Self::ModernColorSpace(mixed))
                } else {
                    Ok(Self::ModernColorSpace(s))
                }
            }
            _ => Ok(self),
        }
    }
}
