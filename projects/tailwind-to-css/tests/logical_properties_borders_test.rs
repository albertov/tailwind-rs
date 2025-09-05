use tailwind_css::{TailwindBuilder, Result};

fn generate_css(input: &str) -> Result<String> {
    let mut builder = TailwindBuilder::default();
    builder.trace(input, false)?;
    builder.bundle()
}

#[test]
fn test_border_s_width_utilities() -> Result<()> {
    // Test basic width values
    let cases = vec![
        ("border-s", "border-inline-start-width:1px"),
        ("border-s-0", "border-inline-start-width:0px"),
        ("border-s-2", "border-inline-start-width:2px"),
        ("border-s-4", "border-inline-start-width:4px"),
        ("border-s-8", "border-inline-start-width:8px"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_border_e_width_utilities() -> Result<()> {
    // Test basic width values
    let cases = vec![
        ("border-e", "border-inline-end-width:1px"),
        ("border-e-0", "border-inline-end-width:0px"),
        ("border-e-2", "border-inline-end-width:2px"),
        ("border-e-4", "border-inline-end-width:4px"),
        ("border-e-8", "border-inline-end-width:8px"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_border_s_color_utilities() -> Result<()> {
    // Test color values
    let cases = vec![
        ("border-s-red-500", "border-inline-start-color:rgb(239 68 68)"),
        ("border-s-blue-500", "border-inline-start-color:rgb(59 130 246)"),
        ("border-s-green-500", "border-inline-start-color:rgb(34 197 94)"),
        ("border-s-black", "border-inline-start-color:rgb(0 0 0)"),
        ("border-s-white", "border-inline-start-color:rgb(255 255 255)"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_border_e_color_utilities() -> Result<()> {
    // Test color values
    let cases = vec![
        ("border-e-red-500", "border-inline-end-color:rgb(239 68 68)"),
        ("border-e-blue-500", "border-inline-end-color:rgb(59 130 246)"),
        ("border-e-green-500", "border-inline-end-color:rgb(34 197 94)"),
        ("border-e-black", "border-inline-end-color:rgb(0 0 0)"),
        ("border-e-white", "border-inline-end-color:rgb(255 255 255)"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_border_logical_with_modifiers() -> Result<()> {
    // Test with hover modifier
    let result = generate_css("hover:border-s-4")?;
    assert!(
        result.contains(":hover") && result.contains("border-inline-start-width:4px"),
        "hover:border-s-4 should work"
    );
    
    // Test with focus modifier
    let result = generate_css("focus:border-e-2")?;
    assert!(
        result.contains(":focus") && result.contains("border-inline-end-width:2px"),
        "focus:border-e-2 should work"
    );
    
    Ok(())
}

#[test]
fn test_border_logical_arbitrary_values() -> Result<()> {
    // Test arbitrary width values
    let cases = vec![
        ("border-s-[3px]", "border-inline-start-width:3px"),
        ("border-e-[5px]", "border-inline-end-width:5px"),
        ("border-s-[0.5rem]", "border-inline-start-width:0.5rem"),
        ("border-e-[10%]", "border-inline-end-width:10%"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_border_logical_arbitrary_colors() -> Result<()> {
    // Test arbitrary color values
    let cases = vec![
        ("border-s-[#ff0000]", "border-inline-start-color:rgb(255 0 0)"),
        ("border-e-[#00ff00]", "border-inline-end-color:rgb(0 255 0)"),
        ("border-s-[rgb(255,0,0)]", "border-inline-start-color:rgb(255 0 0)"),
        ("border-e-[rgba(0,0,255,0.5)]", "border-inline-end-color:rgb(0 0 255 / 0.5)"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_border_logical_with_opacity() -> Result<()> {
    // Test with opacity modifier
    let cases = vec![
        ("border-s-red-500/50", "border-inline-start-color:rgb(239 68 68 / 0.5)"),
        ("border-e-blue-500/25", "border-inline-end-color:rgb(59 130 246 / 0.25)"),
    ];
    
    for (input, expected) in cases {
        let result = generate_css(input)?;
        assert!(
            result.contains(expected),
            "Input '{}' should generate '{}', but got: {}",
            input,
            expected,
            result
        );
    }
    
    Ok(())
}

#[test]
fn test_multiple_logical_borders() -> Result<()> {
    // Test multiple logical border utilities together
    let result = generate_css("border-s-4 border-e-2 border-s-red-500 border-e-blue-500")?;
    
    assert!(result.contains("border-inline-start-width:4px"));
    assert!(result.contains("border-inline-end-width:2px"));
    assert!(result.contains("border-inline-start-color:rgb(239 68 68)"));
    assert!(result.contains("border-inline-end-color:rgb(59 130 246)"));
    
    Ok(())
}

#[test]
fn test_logical_borders_with_responsive() -> Result<()> {
    // Test with responsive modifiers
    let result = generate_css("sm:border-s-2 md:border-e-4")?;
    
    assert!(
        result.contains("@media") && result.contains("border-inline-start-width:2px"),
        "sm:border-s-2 should work with media query"
    );
    
    assert!(
        result.contains("@media") && result.contains("border-inline-end-width:4px"),
        "md:border-e-4 should work with media query"
    );
    
    Ok(())
}

#[test]
fn test_logical_borders_preserve_physical() -> Result<()> {
    // Ensure physical borders still work
    let result = generate_css("border-l-4 border-r-2")?;
    
    assert!(result.contains("border-left-width:4px"));
    assert!(result.contains("border-right-width:2px"));
    
    // Ensure they don't generate logical properties
    assert!(!result.contains("border-inline-start"));
    assert!(!result.contains("border-inline-end"));
    
    Ok(())
}