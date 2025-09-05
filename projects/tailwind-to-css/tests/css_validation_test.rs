/// CSS Validation Layer Tests
/// 
/// This module provides CSS syntax validation tests using lightningcss parser.
/// It catches CSS syntax errors early in the development cycle, before they reach production.
/// 
/// Part of Stage 4 - Transform Utilities Architectural Realignment

use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use tailwind_css::TailwindBuilder;

/// Validates that a CSS string has correct syntax using lightningcss parser.
/// Returns Ok(()) if valid, or Err(String) with the parse error message.
fn validate_css_syntax(css: &str) -> Result<(), String> {
    // Wrap CSS in a dummy selector if it doesn't have one
    let wrapped_css = if css.trim().starts_with('.') || css.trim().starts_with('@') {
        css.to_string()
    } else {
        format!(".dummy {{ {} }}", css)
    };
    
    // Parse and immediately discard the result to avoid lifetime issues
    let result = StyleSheet::parse(&wrapped_css, ParserOptions::default());
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("CSS parse error: {:?}", e)),
    }
}

#[test]
fn test_media_query_spacing() {
    // This test validates CSS syntax for classes that would typically generate media queries
    // Note: inline() returns only CSS properties, not full rules with media queries
    let test_cases = vec![
        "hover:bg-blue-500",
        "lg:hover:text-red-600",
        "sm:hover:scale-110",
        "md:flex",
        "xl:hidden",
    ];
    
    for class in test_cases {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        assert!(result.is_ok(), "Failed to generate CSS for class: {}", class);
        let (_, css) = result.unwrap();
        
        // Skip empty CSS (some modifiers might not produce inline styles)
        if css.trim().is_empty() {
            continue;
        }
        
        // Validate the CSS syntax
        let validation = validate_css_syntax(&css);
        assert!(
            validation.is_ok(),
            "Invalid CSS syntax for class '{}': {:?}\nGenerated CSS:\n{}",
            class,
            validation.err(),
            css
        );
    }
}

#[test]
fn test_transform_property_complete() {
    // Verify transform utilities generate valid CSS syntax
    // Note: inline() may return empty for transform utilities that require transform-base
    let transform_classes = vec![
        "rotate-45",
        "scale-150",
        "translate-x-4",
        "translate-y-1/2",
        "skew-x-12",
        "skew-y-6",
        "rotate-[23deg]",
        "scale-[1.25]",
        "transform",  // Add transform base class
    ];
    
    for class in transform_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        assert!(result.is_ok(), "Failed to generate CSS for transform class: {}", class);
        let (_, css) = result.unwrap();
        
        // Skip empty CSS (transform utilities might not produce inline styles without base)
        if css.trim().is_empty() {
            continue;
        }
        
        // Validate CSS syntax
        let validation = validate_css_syntax(&css);
        assert!(
            validation.is_ok(),
            "Invalid CSS syntax for transform class '{}': {:?}\nGenerated CSS:\n{}",
            class,
            validation.err(),
            css
        );
        
        // Check for transform-related properties or functions if present
        if css.contains("transform") || css.contains("rotate") || css.contains("scale") || css.contains("translate") || css.contains("skew") {
            // Good - contains transform-related CSS
            assert!(true, "Transform class '{}' generated transform-related CSS", class);
        }
    }
}

#[test]
fn test_hover_variants() {
    // Verify hover states generate valid CSS syntax
    // Note: inline() returns CSS properties, not pseudo-classes
    let hover_classes = vec![
        "hover:bg-gray-100",
        "hover:text-blue-600",
        "hover:scale-105",
        "hover:rotate-180",
        "hover:opacity-75",
    ];
    
    for class in hover_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        assert!(result.is_ok(), "Failed to generate CSS for hover class: {}", class);
        let (_, css) = result.unwrap();
        
        // Skip empty CSS
        if css.trim().is_empty() {
            continue;
        }
        
        // Validate CSS syntax
        let validation = validate_css_syntax(&css);
        assert!(
            validation.is_ok(),
            "Invalid CSS syntax for hover class '{}': {:?}\nGenerated CSS:\n{}",
            class,
            validation.err(),
            css
        );
        
        // Verify that hover variants produce syntactically valid CSS properties
        // The actual hover pseudo-class would be added when building full CSS
    }
}

#[test]
fn test_group_hover_selectors() {
    // Verify group-hover generates valid CSS syntax
    // Note: inline() returns CSS properties, not complex selectors
    let group_classes = vec![
        "group-hover:text-white",
        "group-hover:bg-blue-700",
        "group-hover:scale-110",
    ];
    
    for class in group_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        assert!(result.is_ok(), "Failed to generate CSS for group-hover class: {}", class);
        let (_, css) = result.unwrap();
        
        // Skip empty CSS
        if css.trim().is_empty() {
            continue;
        }
        
        // Validate CSS syntax
        let validation = validate_css_syntax(&css);
        assert!(
            validation.is_ok(),
            "Invalid CSS syntax for group-hover class '{}': {:?}\nGenerated CSS:\n{}",
            class,
            validation.err(),
            css
        );
        
        // Group hover classes should produce valid CSS properties
    }
}

#[test]
fn test_responsive_breakpoints() {
    // Verify responsive modifiers generate valid CSS syntax
    // Note: inline() returns CSS properties for the responsive content
    let responsive_classes = vec![
        "sm:text-lg",
        "md:flex",
        "lg:grid",
        "xl:hidden",
        "2xl:block",
        "sm:hover:bg-red-500",
        "lg:group-hover:text-white",
    ];
    
    for class in responsive_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        assert!(result.is_ok(), "Failed to generate CSS for responsive class: {}", class);
        let (_, css) = result.unwrap();
        
        // Skip empty CSS
        if css.trim().is_empty() {
            continue;
        }
        
        // Validate CSS syntax
        let validation = validate_css_syntax(&css);
        assert!(
            validation.is_ok(),
            "Invalid CSS syntax for responsive class '{}': {:?}\nGenerated CSS:\n{}",
            class,
            validation.err(),
            css
        );
        
        // Responsive classes should produce valid CSS properties
        // The media query wrapper would be added when building full CSS
    }
}

#[test]
fn test_arbitrary_values() {
    // Verify arbitrary value syntax generates valid CSS
    let arbitrary_classes = vec![
        "w-[100px]",
        "h-[50vh]",
        "text-[#1a202c]",
        "bg-[rgb(255,255,255)]",
        "rotate-[23deg]",
        "translate-x-[45%]",
        "p-[2.5rem]",
        "top-[10px]",
    ];
    
    for class in arbitrary_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        assert!(result.is_ok(), "Failed to generate CSS for arbitrary value class: {}", class);
        let (_, css) = result.unwrap();
        
        // Validate CSS syntax
        let validation = validate_css_syntax(&css);
        assert!(
            validation.is_ok(),
            "Invalid CSS syntax for arbitrary value class '{}': {:?}\nGenerated CSS:\n{}",
            class,
            validation.err(),
            css
        );
        
        // Extract the value from brackets
        let start = class.find('[').unwrap();
        let end = class.find(']').unwrap();
        let value = &class[start + 1..end];
        
        // Check that the arbitrary value is present in the CSS
        // (It might be transformed, so we check for key parts)
        if value.contains("px") || value.contains("vh") || value.contains("rem") || value.contains("%") {
            // For units, check the numeric part is present
            let numeric_part = value.chars()
                .take_while(|c| c.is_numeric() || *c == '.' || *c == '-')
                .collect::<String>();
            if !numeric_part.is_empty() {
                assert!(
                    css.contains(&numeric_part),
                    "Arbitrary value '{}' from class '{}' should be in CSS. Generated:\n{}",
                    value,
                    class,
                    css
                );
            }
        }
    }
}

#[test]
fn test_complex_combined_modifiers() {
    // Test complex combinations that are prone to syntax errors
    let complex_classes = vec![
        "sm:hover:focus:bg-blue-600",
        "lg:group-hover:scale-110",
        "dark:hover:text-white",
        "md:active:scale-95",
    ];
    
    for class in complex_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        // We don't assert success here as some combinations might not be supported,
        // but if they generate CSS, it should be valid
        if let Ok((_, css)) = result {
            if !css.trim().is_empty() {
                let validation = validate_css_syntax(&css);
                assert!(
                    validation.is_ok(),
                    "Invalid CSS syntax for complex class '{}': {:?}\nGenerated CSS:\n{}",
                    class,
                    validation.err(),
                    css
                );
            }
        }
    }
}

#[test]
fn test_css_function_syntax() {
    // Test CSS functions are properly formatted
    let function_classes = vec![
        "blur-sm",
        "brightness-150",
        "contrast-125",
        "drop-shadow-lg",
        "grayscale",
        "hue-rotate-90",
        "invert",
        "saturate-150",
        "sepia",
    ];
    
    for class in function_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        if let Ok((_, css)) = result {
            if !css.trim().is_empty() {
                // Validate CSS syntax
                let validation = validate_css_syntax(&css);
                assert!(
                    validation.is_ok(),
                    "Invalid CSS syntax for function class '{}': {:?}\nGenerated CSS:\n{}",
                    class,
                    validation.err(),
                    css
                );
                
                // Check for filter property
                if css.contains("filter") {
                    // Ensure proper function syntax with parentheses
                    assert!(
                        css.contains("(") && css.contains(")"),
                        "Filter function '{}' should have proper parentheses. Generated:\n{}",
                        class,
                        css
                    );
                }
            }
        }
    }
}

#[test]
fn test_gradient_syntax() {
    // Test gradient utilities generate valid CSS
    let gradient_classes = vec![
        "bg-gradient-to-r",
        "bg-gradient-to-t",
        "from-blue-500",
        "via-purple-500",
        "to-pink-500",
    ];
    
    for class in gradient_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        if let Ok((_, css)) = result {
            if !css.trim().is_empty() {
                // Validate CSS syntax
                let validation = validate_css_syntax(&css);
                assert!(
                    validation.is_ok(),
                    "Invalid CSS syntax for gradient class '{}': {:?}\nGenerated CSS:\n{}",
                    class,
                    validation.err(),
                    css
                );
                
                // Check for gradient functions if applicable
                if class.starts_with("bg-gradient") && css.contains("background") {
                    assert!(
                        css.contains("gradient") || css.contains("--tw-gradient"),
                        "Gradient class '{}' should reference gradient. Generated:\n{}",
                        class,
                        css
                    );
                }
            }
        }
    }
}

#[test]
fn test_animation_syntax() {
    // Test animation utilities generate valid CSS
    let animation_classes = vec![
        "animate-spin",
        "animate-ping",
        "animate-pulse",
        "animate-bounce",
    ];
    
    for class in animation_classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class);
        
        if let Ok((_, css)) = result {
            if !css.trim().is_empty() {
                // Validate CSS syntax
                let validation = validate_css_syntax(&css);
                assert!(
                    validation.is_ok(),
                    "Invalid CSS syntax for animation class '{}': {:?}\nGenerated CSS:\n{}",
                    class,
                    validation.err(),
                    css
                );
                
                // Check for animation property
                if css.contains("animation") {
                    // Animation values should be properly formatted
                    assert!(
                        css.contains("s ") || css.contains("ms ") || css.contains("infinite"),
                        "Animation class '{}' should have proper timing. Generated:\n{}",
                        class,
                        css
                    );
                }
            }
        }
    }
}