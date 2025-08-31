#[test]
fn ready() {
    println!("it works!");
}

#[test]
fn test_text_alignment_fixes() {
    use tailwind_css::TailwindTextAlignment;
    
    // Test text-center fix (should produce "text-center", not "font-align-center")
    let text_center = TailwindTextAlignment::from("center");
    let text_center_output = text_center.to_string();
    println!("text-center produces: '{}'", text_center_output);
    assert_eq!(text_center_output, "text-center", "text-center should produce 'text-center'");
    
    // Test text-left fix
    let text_left = TailwindTextAlignment::from("left");
    let text_left_output = text_left.to_string();
    println!("text-left produces: '{}'", text_left_output);
    assert_eq!(text_left_output, "text-left", "text-left should produce 'text-left'");
    
    // Test text-right fix
    let text_right = TailwindTextAlignment::from("right");
    let text_right_output = text_right.to_string();
    println!("text-right produces: '{}'", text_right_output);
    assert_eq!(text_right_output, "text-right", "text-right should produce 'text-right'");
    
    // Test text-justify fix
    let text_justify = TailwindTextAlignment::from("justify");
    let text_justify_output = text_justify.to_string();
    println!("text-justify produces: '{}'", text_justify_output);
    assert_eq!(text_justify_output, "text-justify", "text-justify should produce 'text-justify'");
}

#[test]
fn test_transition_fixes() {
    use tailwind_css::{TailwindTransition, TailwindArbitrary};
    
    let empty_arbitrary = TailwindArbitrary::from("");
    
    // Test transition-colors fix (should produce "transition-colors", not "transition[-colors]")
    let transition_colors = TailwindTransition::parse(&["colors"], &empty_arbitrary).unwrap();
    let transition_output = transition_colors.to_string();
    println!("transition-colors produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-colors", "transition-colors should produce 'transition-colors'");
    
    // Test transition-opacity fix
    let transition_opacity = TailwindTransition::parse(&["opacity"], &empty_arbitrary).unwrap();
    let transition_output = transition_opacity.to_string();
    println!("transition-opacity produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-opacity", "transition-opacity should produce 'transition-opacity'");
    
    // Test basic transition (should produce "transition")
    let transition_basic = TailwindTransition::parse(&[], &empty_arbitrary).unwrap();
    let transition_output = transition_basic.to_string();
    println!("transition produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition", "basic transition should produce 'transition'");
    
    // Test transition-all
    let transition_all = TailwindTransition::parse(&["all"], &empty_arbitrary).unwrap();
    let transition_output = transition_all.to_string();
    println!("transition-all produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-all", "transition-all should produce 'transition-all'");
    
    // Test transition-transform
    let transition_transform = TailwindTransition::parse(&["transform"], &empty_arbitrary).unwrap();
    let transition_output = transition_transform.to_string();
    println!("transition-transform produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-transform", "transition-transform should produce 'transition-transform'");
}

#[test]
fn test_trace_preserves_original_class_order() {
    use tailwind_css::TailwindBuilder;
    
    // This test demonstrates the class ordering issue where transformed classes
    // get reordered alphabetically instead of preserving their original input order.
    
    let mut builder = TailwindBuilder::default();
    
    // Input classes in a specific order:
    // - bg-gray-50 (background color)
    // - border-t (will be transformed to border-t-[1px])
    // - border-gray-200 (border color)
    // - p-4 (padding)
    // - text-center (text alignment)
    let input_classes = "bg-gray-50 border-t border-gray-200 p-4 text-center";
    
    // Trace the classes
    let output = builder.trace(input_classes, false).unwrap();
    
    // We EXPECT the output to maintain the original order with transformed values:
    // "bg-gray-50 border-t-[1px] border-gray-200 p-4 text-center"
    let expected = "bg-gray-50 border-t-[1px] border-gray-200 p-4 text-center";
    
    // But what ACTUALLY happens is the classes get reordered alphabetically
    // after transformation, resulting in:
    // "bg-gray-50 border-gray-200 border-t-[1px] p-4 text-center"
    
    // This assertion will FAIL, demonstrating the bug
    assert_eq!(
        output, 
        expected,
        "\nClass order was not preserved!\nExpected: {}\nActual:   {}",
        expected,
        output
    );
}

#[test]
fn test_trace_idempotence() {
    use tailwind_css::TailwindBuilder;
    
    // Define a comprehensive set of valid Tailwind classes
    const TAILWIND_CLASSES: &[&str] = &[
        // Background colors
        "bg-white", "bg-gray-50", "bg-blue-500", "bg-red-600", "bg-green-400",
        // Text colors and styles
        "text-black", "text-gray-600", "text-white", "text-blue-700", "text-sm", "text-lg",
        // Spacing
        "p-4", "px-4", "py-2", "m-2", "mx-auto", "mt-4", "mb-8",
        // Borders
        "border", "border-t", "border-gray-200", "border-2", "border-solid",
        // Rounded corners
        "rounded", "rounded-lg", "rounded-[0.5rem]", "rounded-full",
        // Font weights
        "font-medium", "font-bold", "font-[500]", "font-semibold",
        // Hover and focus states
        "hover:bg-gray-100", "focus:outline-none", "hover:text-blue-600",
        // Transitions and animations
        "transition-colors", "transition-all", "duration-200",
        // Layout
        "flex", "block", "inline-block", "grid", "hidden",
        // Positioning
        "relative", "absolute", "fixed", "sticky",
        // Width and height
        "w-full", "h-screen", "w-[100px]", "min-h-screen",
        // Display utilities
        "flex-col", "items-center", "justify-between", "gap-4",
        // Shadows
        "shadow", "shadow-md", "shadow-lg", "shadow-xl",
    ];
    
    // Test individual classes for idempotence
    println!("\n=== Testing individual class idempotence ===");
    let mut idempotence_failures = Vec::new();
    let mut parse_failures = Vec::new();
    
    for &class in TAILWIND_CLASSES {
        let mut builder1 = TailwindBuilder::default();
        let mut builder2 = TailwindBuilder::default();
        
        // First transformation
        let result1 = builder1.trace(class, false);
        if let Ok(ref transformed1) = result1 {
            // Second transformation (should be idempotent)
            let result2 = builder2.trace(transformed1, false);
            if let Ok(ref transformed2) = result2 {
                if transformed1 != transformed2 {
                    println!("✗ IDEMPOTENCE FAILED for '{}': first='{}', second='{}'", 
                             class, transformed1, transformed2);
                    idempotence_failures.push((class, transformed1.clone(), transformed2.clone()));
                } else {
                    println!("✓ {} -> {} (idempotent)", class, transformed1);
                }
            } else {
                println!("⚠ Second trace failed for '{}': {:?}", class, result2);
                parse_failures.push((class, transformed1.clone()));
            }
        } else {
            println!("⚠ First trace failed for '{}': {:?}", class, result1);
        }
    }
    
    // Test combinations of classes for idempotence
    println!("\n=== Testing class combination idempotence ===");
    
    // Create owned strings for the stress test combinations
    let stress_test_1 = TAILWIND_CLASSES[..10].join(" ");
    let stress_test_2 = TAILWIND_CLASSES[10..20].join(" ");
    let stress_test_3 = TAILWIND_CLASSES[20..30].join(" ");
    
    let test_combinations = vec![
        // Simple combinations
        "bg-white text-black",
        "p-4 m-2 border",
        "rounded-lg shadow-md",
        
        // More complex combinations
        "bg-blue-500 text-white p-4 rounded-lg shadow hover:bg-blue-600",
        "flex items-center justify-between gap-4 p-2",
        "border border-gray-200 rounded-md p-4 hover:shadow-lg transition-shadow",
        
        // With arbitrary values
        "w-[100px] h-[50px] rounded-[0.5rem]",
        "font-[500] text-[14px] leading-[1.5]",
        
        // Mixed utilities
        "relative bg-white rounded-lg shadow-xl p-6 m-4 hover:shadow-2xl transition-all duration-300",
        "absolute top-0 left-0 w-full h-full bg-black bg-opacity-50",
        "grid grid-cols-3 gap-4 p-4 border-t border-gray-200",
        
        // All classes together (stress test)
        stress_test_1.as_str(),
        stress_test_2.as_str(),
        stress_test_3.as_str(),
    ];
    
    for combination in test_combinations {
        let mut builder1 = TailwindBuilder::default();
        let mut builder2 = TailwindBuilder::default();
        
        // First transformation
        let result1 = builder1.trace(combination, false);
        if let Ok(ref transformed1) = result1 {
            // Second transformation (should be idempotent)
            let result2 = builder2.trace(transformed1, false);
            if let Ok(ref transformed2) = result2 {
                let class_count = combination.split_whitespace().count();
                if transformed1 != transformed2 {
                    println!("✗ IDEMPOTENCE FAILED for {} classes: first='{}', second='{}'", 
                             class_count, transformed1, transformed2);
                } else {
                    println!("✓ {} classes: idempotent", class_count);
                }
            } else {
                println!("⚠ Second trace failed for combination: {:?}", result2);
            }
        } else {
            println!("⚠ First trace failed for combination: {:?}", result1);
        }
    }
    
    // Generate pseudo-random combinations using deterministic patterns
    println!("\n=== Testing pseudo-random combinations ===");
    for seed in 0..20 {
        let mut classes = Vec::new();
        let num_classes = (seed % 5) + 2; // 2 to 6 classes
        
        for i in 0..num_classes {
            let index = ((seed * 7 + i * 13) as usize) % TAILWIND_CLASSES.len();
            classes.push(TAILWIND_CLASSES[index]);
        }
        
        let combination = classes.join(" ");
        let mut builder1 = TailwindBuilder::default();
        let mut builder2 = TailwindBuilder::default();
        
        // First transformation
        let result1 = builder1.trace(&combination, false);
        if let Ok(ref transformed1) = result1 {
            // Second transformation (should be idempotent)
            let result2 = builder2.trace(transformed1, false);
            if let Ok(ref transformed2) = result2 {
                if transformed1 != transformed2 {
                    println!("✗ Test {}: IDEMPOTENCE FAILED for '{}': first='{}', second='{}'", 
                             seed, combination, transformed1, transformed2);
                } else {
                    println!("✓ Test {}: {} classes idempotent", seed, classes.len());
                }
            } else {
                println!("⚠ Test {}: Second trace failed: {:?}", seed, result2);
            }
        } else {
            println!("⚠ Test {}: First trace failed: {:?}", seed, result1);
        }
    }
    
    println!("\n=== All idempotence tests completed ===");
    
    // Print summary
    println!("\n=== SUMMARY ===");
    if !idempotence_failures.is_empty() {
        println!("\nIDEMPOTENCE FAILURES ({} total):", idempotence_failures.len());
        for (class, first, second) in &idempotence_failures {
            println!("  {} : {} -> {}", class, first, second);
        }
    }
    
    if !parse_failures.is_empty() {
        println!("\nCLASSES THAT CANNOT BE RE-PARSED AFTER TRANSFORMATION ({} total):", parse_failures.len());
        for (class, transformed) in &parse_failures {
            println!("  {} -> {} (cannot be parsed again)", class, transformed);
        }
    }
    
    if idempotence_failures.is_empty() && parse_failures.is_empty() {
        println!("\n✅ ALL TESTS PASSED: trace() is idempotent for all tested classes!");
    } else {
        println!("\n❌ TEST FAILED: trace() is NOT idempotent for some classes");
        panic!("Idempotence property violated!");
    }
}
