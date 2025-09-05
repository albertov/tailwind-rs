use tailwind_css::{TailwindBuilder, Result};

fn generate_css(input: &str) -> Result<String> {
    let mut builder = TailwindBuilder::default();
    builder.trace(input, false)?;
    builder.bundle()
}

#[test]
fn test_rounded_s_utilities() -> Result<()> {
    // Test start corners (top-start + bottom-start in LTR)
    let cases = vec![
        ("rounded-s", "border-end-start-radius:0.25rem;border-start-start-radius:0.25rem"),
        ("rounded-s-none", "border-end-start-radius:0px;border-start-start-radius:0px"),
        ("rounded-s-sm", "border-end-start-radius:0.25rem;border-start-start-radius:0.25rem"),
        ("rounded-s-md", "border-end-start-radius:0.375rem;border-start-start-radius:0.375rem"),
        ("rounded-s-lg", "border-end-start-radius:0.5rem;border-start-start-radius:0.5rem"),
        ("rounded-s-xl", "border-end-start-radius:0.75rem;border-start-start-radius:0.75rem"),
        ("rounded-s-2xl", "border-end-start-radius:1rem;border-start-start-radius:1rem"),
        ("rounded-s-3xl", "border-end-start-radius:1.5rem;border-start-start-radius:1.5rem"),
        ("rounded-s-full", "border-end-start-radius:9999px;border-start-start-radius:9999px"),
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
fn test_rounded_e_utilities() -> Result<()> {
    // Test end corners (top-end + bottom-end in LTR)
    let cases = vec![
        ("rounded-e", "border-end-end-radius:0.25rem;border-start-end-radius:0.25rem"),
        ("rounded-e-none", "border-end-end-radius:0px;border-start-end-radius:0px"),
        ("rounded-e-sm", "border-end-end-radius:0.25rem;border-start-end-radius:0.25rem"),
        ("rounded-e-md", "border-end-end-radius:0.375rem;border-start-end-radius:0.375rem"),
        ("rounded-e-lg", "border-end-end-radius:0.5rem;border-start-end-radius:0.5rem"),
        ("rounded-e-xl", "border-end-end-radius:0.75rem;border-start-end-radius:0.75rem"),
        ("rounded-e-2xl", "border-end-end-radius:1rem;border-start-end-radius:1rem"),
        ("rounded-e-3xl", "border-end-end-radius:1.5rem;border-start-end-radius:1.5rem"),
        ("rounded-e-full", "border-end-end-radius:9999px;border-start-end-radius:9999px"),
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
fn test_rounded_ss_utilities() -> Result<()> {
    // Test start-start corner (top-left in LTR)
    let cases = vec![
        ("rounded-ss", "border-start-start-radius:0.25rem"),
        ("rounded-ss-none", "border-start-start-radius:0px"),
        ("rounded-ss-sm", "border-start-start-radius:0.25rem"),
        ("rounded-ss-md", "border-start-start-radius:0.375rem"),
        ("rounded-ss-lg", "border-start-start-radius:0.5rem"),
        ("rounded-ss-xl", "border-start-start-radius:0.75rem"),
        ("rounded-ss-2xl", "border-start-start-radius:1rem"),
        ("rounded-ss-3xl", "border-start-start-radius:1.5rem"),
        ("rounded-ss-full", "border-start-start-radius:9999px"),
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
fn test_rounded_se_utilities() -> Result<()> {
    // Test start-end corner (top-right in LTR)
    let cases = vec![
        ("rounded-se", "border-start-end-radius:0.25rem"),
        ("rounded-se-none", "border-start-end-radius:0px"),
        ("rounded-se-sm", "border-start-end-radius:0.25rem"),
        ("rounded-se-md", "border-start-end-radius:0.375rem"),
        ("rounded-se-lg", "border-start-end-radius:0.5rem"),
        ("rounded-se-xl", "border-start-end-radius:0.75rem"),
        ("rounded-se-2xl", "border-start-end-radius:1rem"),
        ("rounded-se-3xl", "border-start-end-radius:1.5rem"),
        ("rounded-se-full", "border-start-end-radius:9999px"),
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
fn test_rounded_es_utilities() -> Result<()> {
    // Test end-start corner (bottom-left in LTR)
    let cases = vec![
        ("rounded-es", "border-end-start-radius:0.25rem"),
        ("rounded-es-none", "border-end-start-radius:0px"),
        ("rounded-es-sm", "border-end-start-radius:0.25rem"),
        ("rounded-es-md", "border-end-start-radius:0.375rem"),
        ("rounded-es-lg", "border-end-start-radius:0.5rem"),
        ("rounded-es-xl", "border-end-start-radius:0.75rem"),
        ("rounded-es-2xl", "border-end-start-radius:1rem"),
        ("rounded-es-3xl", "border-end-start-radius:1.5rem"),
        ("rounded-es-full", "border-end-start-radius:9999px"),
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
fn test_rounded_ee_utilities() -> Result<()> {
    // Test end-end corner (bottom-right in LTR)
    let cases = vec![
        ("rounded-ee", "border-end-end-radius:0.25rem"),
        ("rounded-ee-none", "border-end-end-radius:0px"),
        ("rounded-ee-sm", "border-end-end-radius:0.25rem"),
        ("rounded-ee-md", "border-end-end-radius:0.375rem"),
        ("rounded-ee-lg", "border-end-end-radius:0.5rem"),
        ("rounded-ee-xl", "border-end-end-radius:0.75rem"),
        ("rounded-ee-2xl", "border-end-end-radius:1rem"),
        ("rounded-ee-3xl", "border-end-end-radius:1.5rem"),
        ("rounded-ee-full", "border-end-end-radius:9999px"),
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
fn test_rounded_logical_arbitrary_values() -> Result<()> {
    // Test arbitrary values for logical properties
    let cases = vec![
        ("rounded-s-[8px]", "border-end-start-radius:8px;border-start-start-radius:8px"),
        ("rounded-e-[12px]", "border-end-end-radius:12px;border-start-end-radius:12px"),
        ("rounded-ss-[4px]", "border-start-start-radius:4px"),
        ("rounded-se-[16px]", "border-start-end-radius:16px"),
        ("rounded-es-[20px]", "border-end-start-radius:20px"),
        ("rounded-ee-[24px]", "border-end-end-radius:24px"),
        ("rounded-s-[1.5rem]", "border-end-start-radius:1.5rem;border-start-start-radius:1.5rem"),
        ("rounded-e-[50%]", "border-end-end-radius:50%;border-start-end-radius:50%"),
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
fn test_rounded_logical_with_modifiers() -> Result<()> {
    // Test logical border radius with modifiers like hover and responsive
    let cases = vec![
        ("hover:rounded-s-lg", ":hover", "border-end-start-radius:0.5rem", "border-start-start-radius:0.5rem"),
        ("hover:rounded-e-xl", ":hover", "border-end-end-radius:0.75rem", "border-start-end-radius:0.75rem"),
        ("hover:rounded-ss-md", ":hover", "border-start-start-radius:0.375rem", ""),
        ("hover:rounded-se-lg", ":hover", "border-start-end-radius:0.5rem", ""),
        ("hover:rounded-es-sm", ":hover", "border-end-start-radius:0.25rem", ""),
        ("hover:rounded-ee-2xl", ":hover", "border-end-end-radius:1rem", ""),
        ("lg:rounded-s-lg", "@media", "border-end-start-radius:0.5rem", "border-start-start-radius:0.5rem"),
        ("lg:rounded-e-lg", "@media", "border-end-end-radius:0.5rem", "border-start-end-radius:0.5rem"),
    ];
    
    for (input, modifier, prop1, prop2) in cases {
        let result = generate_css(input)?;
        // Normalize whitespace for comparison
        let normalized_result = result.replace(" ", "").replace("\n", "");
        
        assert!(
            normalized_result.contains(modifier),
            "Input '{}' should contain modifier '{}', but got: {}",
            input,
            modifier,
            result
        );
        
        assert!(
            normalized_result.contains(prop1),
            "Input '{}' should generate property '{}', but got: {}",
            input,
            prop1,
            result
        );
        
        if !prop2.is_empty() {
            assert!(
                normalized_result.contains(prop2),
                "Input '{}' should generate property '{}', but got: {}",
                input,
                prop2,
                result
            );
        }
    }
    
    Ok(())
}

#[test]
fn test_rounded_logical_combined() -> Result<()> {
    // Test combinations of logical border radius utilities
    let input = "rounded-s-lg rounded-e-xl";
    let result = generate_css(input)?;
    
    assert!(
        result.contains("border-end-start-radius:0.5rem;border-start-start-radius:0.5rem"),
        "Should contain rounded-s-lg styles"
    );
    assert!(
        result.contains("border-end-end-radius:0.75rem;border-start-end-radius:0.75rem"),
        "Should contain rounded-e-xl styles"
    );
    
    Ok(())
}

#[test]
fn test_all_logical_corners_separately() -> Result<()> {
    // Test all four logical corners set individually
    let input = "rounded-ss-sm rounded-se-md rounded-es-lg rounded-ee-xl";
    let result = generate_css(input)?;
    
    assert!(result.contains("border-start-start-radius:0.25rem"), "Should contain rounded-ss-sm");
    assert!(result.contains("border-start-end-radius:0.375rem"), "Should contain rounded-se-md");
    assert!(result.contains("border-end-start-radius:0.5rem"), "Should contain rounded-es-lg");
    assert!(result.contains("border-end-end-radius:0.75rem"), "Should contain rounded-ee-xl");
    
    Ok(())
}

#[test]
fn test_rounded_xs_size() -> Result<()> {
    // Test the xs size value for logical properties
    let cases = vec![
        ("rounded-s-xs", "border-end-start-radius:0.125rem;border-start-start-radius:0.125rem"),
        ("rounded-e-xs", "border-end-end-radius:0.125rem;border-start-end-radius:0.125rem"),
        ("rounded-ss-xs", "border-start-start-radius:0.125rem"),
        ("rounded-se-xs", "border-start-end-radius:0.125rem"),
        ("rounded-es-xs", "border-end-start-radius:0.125rem"),
        ("rounded-ee-xs", "border-end-end-radius:0.125rem"),
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
fn test_rounded_4xl_size() -> Result<()> {
    // Test the 4xl size value for logical properties
    let cases = vec![
        ("rounded-s-4xl", "border-end-start-radius:2rem;border-start-start-radius:2rem"),
        ("rounded-e-4xl", "border-end-end-radius:2rem;border-start-end-radius:2rem"),
        ("rounded-ss-4xl", "border-start-start-radius:2rem"),
        ("rounded-se-4xl", "border-start-end-radius:2rem"),
        ("rounded-es-4xl", "border-end-start-radius:2rem"),
        ("rounded-ee-4xl", "border-end-end-radius:2rem"),
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