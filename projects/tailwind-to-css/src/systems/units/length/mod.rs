use std::ops::Rem;

use tailwind_ast::parse_fraction;

use super::*;

/// Process underscores in CSS expressions
/// Converts underscores to spaces, with special handling for calc() operators
fn process_css_expression_underscores(value: &str) -> String {
    // For calc expressions, we need to be smart about underscores
    // "_+_" should become " + ", "_-_" should become " - ", etc.
    let mut result = String::with_capacity(value.len());
    let chars: Vec<char> = value.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if i > 0 && i < chars.len() - 1 && chars[i] == '_' {
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
}

#[derive(Debug, Clone)]
pub enum LengthUnit {
    Fraction(u32, u32),
    Unit(f32, &'static str),
    CssExpression(String),
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fraction(a, b) => write!(f, "{}/{}", a, b),
            Self::Unit(a, b) => write!(f, "{}{}", a, b),
            Self::CssExpression(expr) => write!(f, "{}", expr),
        }
    }
}

impl LengthUnit {
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/length#syntax>
    pub fn parse_faction(input: &str) -> Result<Self> {
        let (a, b) = parse_fraction(input)?.1;
        Ok(Self::radio(a as u32, b as u32))
    }
    pub fn parse_length(input: &str) -> Result<Self> {
        // Check for CSS functions first
        if Self::is_css_function(input) {
            return Ok(Self::CssExpression(input.to_string()));
        }
        
        // Support all CSS length units
        // Absolute units
        let absolute = (unit("px"), unit("cm"), unit("mm"), unit("in"), unit("pt"), unit("pc"), unit("Q"));
        // Relative units
        let relative = (unit("em"), unit("rem"), unit("ex"), unit("ch"), unit("lh"), unit("rlh"));
        // Viewport units
        let viewport = (unit("vw"), unit("vh"), unit("vmin"), unit("vmax"), unit("vi"), unit("vb"), 
                        unit("svw"), unit("svh"), unit("lvw"), unit("lvh"), unit("dvw"), unit("dvh"));
        // Container units
        let container = (unit("cqw"), unit("cqh"), unit("cqi"), unit("cqb"), unit("cqmin"), unit("cqmax"));
        // Percentage
        let percent = unit("%");
        
        let valid = alt((alt(absolute), alt(relative), alt(viewport), alt(container), percent));
        let (f, unit) = tuple((parse_f32, valid))(input)?.1;
        Ok(Self::Unit(f, unit))
    }
    pub fn parse_angle(input: &str) -> Result<Self> {
        let valid = (unit("deg"), unit("rad"), unit("grad"), unit("turn"));
        let (f, unit) = tuple((parse_f32, alt(valid)))(input)?.1;
        Ok(Self::Unit(f, unit))
    }
    pub fn px(x: f32) -> Self {
        Self::Unit(x, "px")
    }
    pub fn em(x: f32) -> Self {
        Self::Unit(x, "em")
    }
    pub fn rem(x: f32) -> Self {
        Self::Unit(x, "rem")
    }
    pub fn percent(x: f32) -> Self {
        Self::Unit(x, "%")
    }
    pub fn radio(a: u32, b: u32) -> Self {
        if b.eq(&0) {
            return Self::Fraction(0, 1);
        }
        let n = gcd(a, b);
        Self::Fraction(a / n, b / n)
    }
    
    /// Check if input is a CSS function (calc, var, clamp, min, max)
    fn is_css_function(input: &str) -> bool {
        let functions = ["calc(", "var(", "clamp(", "min(", "max("];
        functions.iter().any(|f| input.starts_with(f))
    }
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: PartialEq + Rem<Output = T> + Default + Copy,
{
    if b == T::default() { a } else { gcd(b, a % b) }
}

fn unit(unit: &'static str) -> impl Fn(&str) -> IResult<&str, &'static str> {
    move |input: &str| tag(unit)(input).map(|(s, _)| (s, unit))
}

impl LengthUnit {
    #[inline]
    pub fn get_class(&self) -> String {
        self.to_string()
    }
    #[inline]
    pub fn get_class_arbitrary(&self) -> String {
        format!("[{}]", self)
    }
    #[inline]
    pub fn get_properties(&self) -> String {
        match self {
            Self::Fraction(a, b) => {
                // Handle common fractions with exact decimal representations
                // to match Tailwind CSS output exactly
                match (*a, *b) {
                    (1, 2) => "50%".to_string(),
                    (1, 3) => "33.333333%".to_string(),
                    (2, 3) => "66.666667%".to_string(),
                    (1, 4) => "25%".to_string(),
                    (3, 4) => "75%".to_string(),
                    (1, 5) => "20%".to_string(),
                    (2, 5) => "40%".to_string(),
                    (3, 5) => "60%".to_string(),
                    (4, 5) => "80%".to_string(),
                    (1, 6) => "16.666667%".to_string(),
                    (5, 6) => "83.333333%".to_string(),
                    (a, b) => {
                        // For other fractions, use the standard calculation
                        let p = a as f64 / b as f64;
                        let percentage = 100.0 * p;
                        
                        // Format with appropriate precision
                        if percentage.fract() == 0.0 {
                            format!("{}%", percentage as i32)
                        } else {
                            // Use 6 decimal places for consistency with Tailwind
                            format!("{:.6}%", percentage).trim_end_matches('0').trim_end_matches('.').to_string() + "%"
                        }
                    }
                }
            },
            Self::Unit(a, b) => format!("{}{}", a, b),
            Self::CssExpression(expr) => {
                // Process underscores in CSS expressions (convert to spaces, handle operators)
                process_css_expression_underscores(expr)
            },
        }
    }

    pub fn is_fraction(&self) -> bool {
        matches!(self, Self::Fraction { .. })
    }
    pub fn is_fraction_eq(&self) -> bool {
        match self {
            Self::Fraction(a, b) => a.eq(b),
            _ => false,
        }
    }
    pub fn is_fraction_zero(&self) -> bool {
        match self {
            Self::Fraction(a, _) => a.eq(&0),
            _ => false,
        }
    }
}
