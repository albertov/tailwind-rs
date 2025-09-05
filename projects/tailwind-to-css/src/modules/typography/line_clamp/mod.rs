use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindLineClamp {
    kind: LineClampKind,
}

#[derive(Debug, Clone)]
enum LineClampKind {
    None,
    Lines(u8),
    Arbitrary(TailwindArbitrary),
}

impl Display for TailwindLineClamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            LineClampKind::None => write!(f, "line-clamp-none"),
            LineClampKind::Lines(n) => write!(f, "line-clamp-{}", n),
            LineClampKind::Arbitrary(s) => write!(f, "line-clamp-{}", s.get_class()),
        }
    }
}

impl TailwindInstance for TailwindLineClamp {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match &self.kind {
            LineClampKind::None => css_attributes! {
                "overflow" => "visible",
                "display" => "block",
                "-webkit-box-orient" => "horizontal",
                "-webkit-line-clamp" => "none"
            },
            LineClampKind::Lines(n) => css_attributes! {
                "overflow" => "hidden",
                "display" => "-webkit-box",
                "-webkit-box-orient" => "vertical",
                "-webkit-line-clamp" => n.to_string()
            },
            LineClampKind::Arbitrary(s) => css_attributes! {
                "overflow" => "hidden",
                "display" => "-webkit-box",
                "-webkit-box-orient" => "vertical",
                "-webkit-line-clamp" => s.get_properties()
            },
        }
    }
}

impl TailwindLineClamp {
    /// Create a line-clamp instance from a string value
    pub fn from(value: &str) -> Result<Self> {
        let kind = match value {
            "none" => LineClampKind::None,
            "1" => LineClampKind::Lines(1),
            "2" => LineClampKind::Lines(2),
            "3" => LineClampKind::Lines(3),
            "4" => LineClampKind::Lines(4),
            "5" => LineClampKind::Lines(5),
            "6" => LineClampKind::Lines(6),
            _ => return syntax_error!("Unknown line-clamp value: {}", value),
        };
        Ok(Self { kind })
    }

    /// Parse line-clamp utilities
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => LineClampKind::Arbitrary(arbitrary.to_owned()),
            ["none"] => LineClampKind::None,
            ["1"] => LineClampKind::Lines(1),
            ["2"] => LineClampKind::Lines(2),
            ["3"] => LineClampKind::Lines(3),
            ["4"] => LineClampKind::Lines(4),
            ["5"] => LineClampKind::Lines(5),
            ["6"] => LineClampKind::Lines(6),
            _ => return syntax_error!("Unknown line-clamp pattern: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_line_clamp_display() {
        assert_eq!(TailwindLineClamp::from("none").unwrap().to_string(), "line-clamp-none");
        assert_eq!(TailwindLineClamp::from("1").unwrap().to_string(), "line-clamp-1");
        assert_eq!(TailwindLineClamp::from("2").unwrap().to_string(), "line-clamp-2");
        assert_eq!(TailwindLineClamp::from("3").unwrap().to_string(), "line-clamp-3");
        assert_eq!(TailwindLineClamp::from("4").unwrap().to_string(), "line-clamp-4");
        assert_eq!(TailwindLineClamp::from("5").unwrap().to_string(), "line-clamp-5");
        assert_eq!(TailwindLineClamp::from("6").unwrap().to_string(), "line-clamp-6");
    }
    
    #[test]
    fn test_line_clamp_attributes() {
        let builder = TailwindBuilder::default();
        
        // Test line-clamp-none
        let none = TailwindLineClamp::from("none").unwrap();
        let attrs = none.attributes(&builder);
        assert!(attrs.to_string().contains("overflow:visible"));
        assert!(attrs.to_string().contains("-webkit-line-clamp:none"));
        
        // Test line-clamp-1
        let one = TailwindLineClamp::from("1").unwrap();
        let attrs = one.attributes(&builder);
        assert!(attrs.to_string().contains("overflow:hidden"));
        assert!(attrs.to_string().contains("display:-webkit-box"));
        assert!(attrs.to_string().contains("-webkit-box-orient:vertical"));
        assert!(attrs.to_string().contains("-webkit-line-clamp:1"));
        
        // Test line-clamp-3
        let three = TailwindLineClamp::from("3").unwrap();
        let attrs = three.attributes(&builder);
        assert!(attrs.to_string().contains("-webkit-line-clamp:3"));
        
        // Test line-clamp-6
        let six = TailwindLineClamp::from("6").unwrap();
        let attrs = six.attributes(&builder);
        assert!(attrs.to_string().contains("-webkit-line-clamp:6"));
    }
    
    #[test]
    fn test_line_clamp_parse() {
        let arbitrary = TailwindArbitrary::from("10");
        
        // Test parsing with pattern
        let clamp = TailwindLineClamp::parse(&["3"], &arbitrary).unwrap();
        assert_eq!(clamp.to_string(), "line-clamp-3");
        
        // Test parsing none
        let none = TailwindLineClamp::parse(&["none"], &arbitrary).unwrap();
        assert_eq!(none.to_string(), "line-clamp-none");
        
        // Test arbitrary value
        let arb = TailwindLineClamp::parse(&[], &arbitrary).unwrap();
        assert_eq!(arb.to_string(), "line-clamp-[10]");
    }
}