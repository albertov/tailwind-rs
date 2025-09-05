use tailwind_css::TailwindBuilder;

#[test]
fn test_mb_6_direct() {
    let mut tw = TailwindBuilder::default();
    let result = tw.inline("mb-6");
    println!("mb-6 generates: {:?}", result);
    
    match result {
        Ok((transformed, css)) => {
            println!("  Transformed: {}", transformed);
            println!("  CSS: {}", css);
            // Expected: margin-bottom: 1.5rem;
            assert!(css.contains("margin-bottom"), "mb-6 should generate margin-bottom CSS, got: {}", css);
            assert!(css.contains("1.5rem"), "mb-6 should use 1.5rem value, got: {}", css);
        }
        Err(e) => {
            panic!("Failed to process mb-6: {:?}", e);
        }
    }
}

#[test]
fn test_various_spacing_utilities() {
    let mut tw = TailwindBuilder::default();
    
    let test_cases = vec![
        ("m-4", "margin:", "1rem"),
        ("p-4", "padding:", "1rem"),
        ("mt-2", "margin-top:", "0.5rem"),
        ("mb-6", "margin-bottom:", "1.5rem"),
        ("px-8", "padding-left:", "2rem"),
        ("py-3", "padding-top:", "0.75rem"),
    ];
    
    for (class, expected_property, expected_value) in test_cases {
        println!("\nTesting {}", class);
        match tw.inline(class) {
            Ok((transformed, css)) => {
                println!("  Transformed: {}", transformed);
                println!("  CSS: {}", css);
                if css.is_empty() {
                    println!("  WARNING: {} generated no CSS!", class);
                } else {
                    assert!(css.contains(expected_property), 
                        "{} should contain '{}', got: {}", class, expected_property, css);
                    assert!(css.contains(expected_value), 
                        "{} should contain '{}', got: {}", class, expected_value, css);
                }
            }
            Err(e) => {
                println!("  ERROR processing {}: {:?}", class, e);
            }
        }
    }
}

#[test]
fn test_edge_case_spacing() {
    let mut tw = TailwindBuilder::default();
    
    let edge_cases = vec![
        ("m-0", "margin:", "0"),
        ("m-px", "margin:", "1px"),
        ("m-0.5", "margin:", "0.125rem"),
        ("-mt-4", "margin-top:", "-1rem"),
    ];
    
    for (class, expected_property, expected_value) in edge_cases {
        println!("\nTesting edge case: {}", class);
        match tw.inline(class) {
            Ok((transformed, css)) => {
                println!("  Transformed: {}", transformed);
                println!("  CSS: {}", css);
                if !css.is_empty() {
                    assert!(css.contains(expected_property), 
                        "{} should contain '{}', got: {}", class, expected_property, css);
                    assert!(css.contains(expected_value), 
                        "{} should contain '{}', got: {}", class, expected_value, css);
                } else {
                    println!("  WARNING: {} generated no CSS!", class);
                }
            }
            Err(e) => {
                println!("  ERROR processing {}: {:?}", class, e);
            }
        }
    }
}

#[test]
fn test_mb_6_in_context() {
    let mut tw = TailwindBuilder::default();
    
    // Test with HTML context using trace
    println!("\nTesting mb-6 with trace method");
    match tw.trace(r#"<div class="mb-6">Test</div>"#, false) {
        Ok(transformed) => {
            println!("  Trace result: {}", transformed);
            // Now get the CSS
            match tw.inline("mb-6") {
                Ok((_, css)) => {
                    println!("  Generated CSS: {}", css);
                    if !css.is_empty() {
                        assert!(css.contains("margin-bottom"), "CSS should contain margin-bottom from mb-6");
                        assert!(css.contains("1.5rem"), "CSS should contain 1.5rem value from mb-6");
                    } else {
                        println!("  WARNING: mb-6 generated no CSS even after trace!");
                    }
                }
                Err(e) => println!("  ERROR getting CSS: {:?}", e),
            }
        }
        Err(e) => {
            println!("  ERROR in trace: {:?}", e);
        }
    }
}

#[test]
fn test_compare_working_vs_broken() {
    let mut tw = TailwindBuilder::default();
    
    println!("\nTesting various margin-bottom values:");
    // Test various margin-bottom values to find which work
    for i in 0..10 {
        let class = format!("mb-{}", i);
        match tw.inline(&class) {
            Ok((_transformed, css)) => {
                if !css.is_empty() {
                    println!("  {} -> CSS: {}", class, css);
                    assert!(css.contains("margin-bottom"), "{} should generate margin-bottom", class);
                } else {
                    println!("  {} -> NO CSS GENERATED", class);
                }
            }
            Err(e) => {
                println!("  {} -> ERROR: {:?}", class, e);
            }
        }
    }
}

#[test] 
fn test_exact_css_output() {
    let mut tw = TailwindBuilder::default();
    
    println!("\nTesting exact CSS output vs Tailwind v3 spec:");
    // Test exact CSS output matches Tailwind v3 spec
    let test_cases = vec![
        ("mb-6", vec!["margin-bottom:1.5rem"]),
        ("m-4", vec!["margin:1rem"]),
        ("p-4", vec!["padding:1rem"]),
        ("mt-2", vec!["margin-top:0.5rem"]),
        ("px-8", vec!["padding-left:2rem", "padding-right:2rem"]),
        ("py-3", vec!["padding-top:0.75rem", "padding-bottom:0.75rem"]),
        ("m-0", vec!["margin:0"]),
        ("m-px", vec!["margin:1px"]),
        ("-mt-4", vec!["margin-top:-1rem"]),
    ];
    
    let mut failures = Vec::new();
    
    for (class, expected_props) in test_cases {
        match tw.inline(class) {
            Ok((_, css)) => {
                if css.is_empty() {
                    println!("  {} -> NO CSS GENERATED (expected: {:?})", class, expected_props);
                    failures.push(format!("{} generates no CSS", class));
                } else {
                    println!("  {} -> {}", class, css);
                    // Check that all expected properties are present (order-agnostic)
                    let mut all_present = true;
                    for prop in &expected_props {
                        if !css.contains(prop) {
                            all_present = false;
                            println!("    MISSING: {}", prop);
                        }
                    }
                    if !all_present {
                        failures.push(format!("{} doesn't contain all expected properties", class));
                    }
                }
            }
            Err(e) => {
                println!("  {} -> ERROR: {:?}", class, e);
                failures.push(format!("{} failed with error", class));
            }
        }
    }
    
    if !failures.is_empty() {
        println!("\nFAILURES:");
        for failure in &failures {
            println!("  - {}", failure);
        }
        panic!("Spacing utilities test failed with {} failures", failures.len());
    }
}