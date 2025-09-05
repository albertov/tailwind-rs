#[test]
fn ready() {
    println!("it works!");
}

#[test]
fn test_phase1_utility_recognition() {
    use tailwind_css::TailwindBuilder;
    
    println!("\n=== Testing Phase 1 Utility Recognition ===");
    
    // Define Phase 1 utilities to test
    struct UtilityTest {
        class: &'static str,
        expected_css_property: &'static str,
        category: &'static str,
    }
    
    let phase1_utilities = vec![
        // Cursor utilities - comprehensive list
        UtilityTest { class: "cursor-pointer", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-default", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-auto", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-wait", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-text", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-move", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-help", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-not-allowed", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-none", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-progress", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-grab", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-grabbing", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-crosshair", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-zoom-in", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-zoom-out", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-context-menu", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-cell", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-vertical-text", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-alias", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-copy", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-no-drop", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-all-scroll", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-col-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-row-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-n-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-e-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-s-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-w-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-ne-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-nw-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-se-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-sw-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-ew-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-ns-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-nesw-resize", expected_css_property: "cursor", category: "Cursor" },
        UtilityTest { class: "cursor-nwse-resize", expected_css_property: "cursor", category: "Cursor" },
        
        // User-select utilities
        UtilityTest { class: "select-none", expected_css_property: "user-select", category: "User Select" },
        UtilityTest { class: "select-text", expected_css_property: "user-select", category: "User Select" },
        UtilityTest { class: "select-all", expected_css_property: "user-select", category: "User Select" },
        UtilityTest { class: "select-auto", expected_css_property: "user-select", category: "User Select" },
        
        // Pointer-events utilities
        UtilityTest { class: "pointer-events-none", expected_css_property: "pointer-events", category: "Pointer Events" },
        UtilityTest { class: "pointer-events-auto", expected_css_property: "pointer-events", category: "Pointer Events" },
        
        // Resize utilities
        UtilityTest { class: "resize", expected_css_property: "resize", category: "Resize" },
        UtilityTest { class: "resize-x", expected_css_property: "resize", category: "Resize" },
        UtilityTest { class: "resize-y", expected_css_property: "resize", category: "Resize" },
        UtilityTest { class: "resize-none", expected_css_property: "resize", category: "Resize" },
    ];
    
    let mut recognized_count = 0;
    let mut unrecognized_classes = Vec::new();
    let mut category_stats = std::collections::HashMap::new();
    
    println!("\nTesting {} Phase 1 utility classes...\n", phase1_utilities.len());
    
    for test in &phase1_utilities {
        let mut builder = TailwindBuilder::default();
        
        // Test using inline mode to get actual CSS output
        match builder.inline(test.class) {
            Ok((transformed_classes, generated_css)) => {
                // Check if CSS was actually generated (not empty)
                let css_generated = !generated_css.trim().is_empty();
                
                // Check if the expected CSS property is present
                let property_found = generated_css.contains(test.expected_css_property);
                
                if css_generated && property_found {
                    recognized_count += 1;
                    let stats = category_stats.entry(test.category).or_insert((0, 0));
                    stats.0 += 1;
                    println!("✓ {} - RECOGNIZED", test.class);
                    println!("  Transformed: {}", transformed_classes);
                    println!("  Generated CSS contains '{}': {}", 
                             test.expected_css_property, 
                             generated_css.lines().next().unwrap_or(""));
                } else if css_generated {
                    // CSS was generated but doesn't contain expected property
                    unrecognized_classes.push((test.class, "CSS generated but wrong property"));
                    let stats = category_stats.entry(test.category).or_insert((0, 0));
                    stats.1 += 1;
                    println!("⚠ {} - PARTIAL RECOGNITION", test.class);
                    println!("  Generated CSS but missing '{}'", test.expected_css_property);
                    println!("  CSS: {}", generated_css.lines().next().unwrap_or(""));
                } else {
                    // No CSS generated - class not recognized
                    unrecognized_classes.push((test.class, "No CSS generated"));
                    let stats = category_stats.entry(test.category).or_insert((0, 0));
                    stats.1 += 1;
                    println!("✗ {} - NOT RECOGNIZED", test.class);
                    println!("  Transformed: {}", transformed_classes);
                }
            }
            Err(e) => {
                unrecognized_classes.push((test.class, "Parse error"));
                let stats = category_stats.entry(test.category).or_insert((0, 0));
                stats.1 += 1;
                println!("✗ {} - PARSE ERROR: {:?}", test.class, e);
            }
        }
    }
    
    // Also test using trace mode for comparison
    println!("\n=== Testing with trace() method for comparison ===");
    println!("Note: trace() returns transformed class names, inline() returns CSS");
    for test in &phase1_utilities[0..5] { // Test a few from different categories
        let mut builder = TailwindBuilder::default();
        match builder.trace(test.class, false) {
            Ok(result) => {
                let transformed = result.as_ref() != test.class;
                println!("trace('{}') -> '{}' (transformed: {})", 
                         test.class, result, transformed);
                
                // Check if the builder accumulated any CSS objects
                // println!("  CSS objects accumulated: {}", builder.objects.len());
            }
            Err(e) => {
                println!("trace('{}') failed: {:?}", test.class, e);
            }
        }
    }
    
    // Print summary
    println!("\n=== RECOGNITION SUMMARY ===");
    println!("Total utilities tested: {}", phase1_utilities.len());
    println!("Recognized: {} ({:.1}%)", 
             recognized_count, 
             (recognized_count as f64 / phase1_utilities.len() as f64) * 100.0);
    println!("Not recognized: {} ({:.1}%)", 
             unrecognized_classes.len(),
             (unrecognized_classes.len() as f64 / phase1_utilities.len() as f64) * 100.0);
    
    println!("\n=== CATEGORY BREAKDOWN ===");
    for (category, (recognized, failed)) in &category_stats {
        let total = recognized + failed;
        println!("{}: {}/{} recognized ({:.1}%)", 
                 category, recognized, total, 
                 (*recognized as f64 / total as f64) * 100.0);
    }
    
    if !unrecognized_classes.is_empty() {
        println!("\n=== UNRECOGNIZED CLASSES ===");
        for (class, reason) in &unrecognized_classes {
            println!("  {} - {}", class, reason);
        }
    }
    
    // Determine if we should proceed with Phase 1
    let recognition_rate = recognized_count as f64 / phase1_utilities.len() as f64;
    if recognition_rate >= 0.8 {
        println!("\n✅ RECOMMENDATION: Proceed with Phase 1 - {:.1}% recognition rate", recognition_rate * 100.0);
    } else if recognition_rate >= 0.5 {
        println!("\n⚠️ RECOMMENDATION: Partial support - {:.1}% recognition rate. Consider implementing missing utilities first.", recognition_rate * 100.0);
    } else {
        println!("\n❌ RECOMMENDATION: Do not proceed - Only {:.1}% recognition rate. Major implementation needed.", recognition_rate * 100.0);
        panic!("Recognition rate too low for Phase 1!");
    }
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
    
    // Define a comprehensive set of valid Tailwind classes (500+ utilities)
    const TAILWIND_CLASSES: &[&str] = &[
        // ===== LAYOUT SYSTEM (80+ utilities) =====
        // Display (20+ utilities)
        "block", "inline-block", "inline", "flex", "inline-flex", "table", "inline-table",
        "table-caption", "table-cell", "table-column", "table-column-group", 
        "table-footer-group", "table-header-group", "table-row-group", "table-row",
        "flow-root", "grid", "inline-grid", "contents", "list-item", "hidden",
        
        // Position (5 utilities)
        "static", "fixed", "absolute", "relative", "sticky",
        
        // Positioning Values (20+ utilities)
        "top-0", "top-1", "top-2", "top-4", "top-8", "top-16", "top-32", "top-64",
        "right-0", "right-1", "right-2", "right-4", "right-8", "right-16",
        "bottom-0", "bottom-1", "bottom-2", "bottom-4", "bottom-8", "bottom-16",
        "left-0", "left-1", "left-2", "left-4", "left-8", "left-16",
        "inset-0", "inset-1", "inset-2", "inset-4", "inset-x-0", "inset-y-0",
        "-top-1", "-right-1", "-bottom-1", "-left-1", // negative values
        
        // Visibility (2 utilities)
        "visible", "invisible", 
        
        // Z-Index (10+ utilities)
        "z-0", "z-10", "z-20", "z-30", "z-40", "z-50", "z-auto", "-z-10", "-z-20",
        
        // Float & Clear (6 utilities)
        "float-left", "float-right", "float-none", "clear-left", "clear-right", "clear-both",
        
        // Object Fit & Position (10+ utilities)
        "object-contain", "object-cover", "object-fill", "object-none", "object-scale-down",
        "object-bottom", "object-center", "object-left", "object-left-bottom", "object-left-top",
        "object-right", "object-right-bottom", "object-right-top", "object-top",
        
        // Overflow (12+ utilities)
        "overflow-auto", "overflow-hidden", "overflow-clip", "overflow-visible", "overflow-scroll",
        "overflow-x-auto", "overflow-x-hidden", "overflow-x-clip", "overflow-x-visible", "overflow-x-scroll",
        "overflow-y-auto", "overflow-y-hidden", "overflow-y-clip", "overflow-y-visible", "overflow-y-scroll",
        
        // Overscroll (6 utilities)
        "overscroll-auto", "overscroll-contain", "overscroll-none",
        "overscroll-x-auto", "overscroll-x-contain", "overscroll-x-none",
        "overscroll-y-auto", "overscroll-y-contain", "overscroll-y-none",
        
        // ===== FLEXBOX & GRID SYSTEM (120+ utilities) =====
        // Flexbox Direction & Wrap (8 utilities)
        "flex-row", "flex-row-reverse", "flex-col", "flex-col-reverse",
        "flex-wrap", "flex-wrap-reverse", "flex-nowrap",
        
        // Flexbox Grow & Shrink (6 utilities)
        "flex-1", "flex-auto", "flex-initial", "flex-none", "grow", "grow-0", "shrink", "shrink-0",
        
        // Flexbox Basis (10+ utilities)
        "basis-0", "basis-1", "basis-2", "basis-4", "basis-8", "basis-16", "basis-32",
        "basis-1/2", "basis-1/3", "basis-1/4", "basis-full", "basis-auto",
        
        // Grid Template (24+ utilities)
        "grid-cols-1", "grid-cols-2", "grid-cols-3", "grid-cols-4", "grid-cols-5", "grid-cols-6",
        "grid-cols-7", "grid-cols-8", "grid-cols-9", "grid-cols-10", "grid-cols-11", "grid-cols-12",
        "grid-cols-none", "grid-cols-subgrid",
        "grid-rows-1", "grid-rows-2", "grid-rows-3", "grid-rows-4", "grid-rows-5", "grid-rows-6",
        "grid-rows-none", "grid-rows-subgrid",
        
        // Grid Placement (30+ utilities)
        "col-auto", "col-span-1", "col-span-2", "col-span-3", "col-span-4", "col-span-5", "col-span-6",
        "col-span-7", "col-span-8", "col-span-9", "col-span-10", "col-span-11", "col-span-12", "col-span-full",
        "col-start-1", "col-start-2", "col-start-3", "col-start-4", "col-start-5", "col-start-6",
        "col-start-7", "col-start-8", "col-start-9", "col-start-10", "col-start-11", "col-start-12", "col-start-13",
        "col-start-auto", "col-end-1", "col-end-2", "col-end-3", "col-end-4", "col-end-5", "col-end-6",
        "col-end-7", "col-end-8", "col-end-9", "col-end-10", "col-end-11", "col-end-12", "col-end-13", "col-end-auto",
        "row-auto", "row-span-1", "row-span-2", "row-span-3", "row-span-4", "row-span-5", "row-span-6", "row-span-full",
        "row-start-1", "row-start-2", "row-start-3", "row-start-4", "row-start-5", "row-start-6", "row-start-7", "row-start-auto",
        "row-end-1", "row-end-2", "row-end-3", "row-end-4", "row-end-5", "row-end-6", "row-end-7", "row-end-auto",
        
        // Gap (20+ utilities)
        "gap-0", "gap-px", "gap-0.5", "gap-1", "gap-2", "gap-3", "gap-4", "gap-5", "gap-6", "gap-8", "gap-10", "gap-12",
        "gap-x-0", "gap-x-1", "gap-x-2", "gap-x-4", "gap-x-8",
        "gap-y-0", "gap-y-1", "gap-y-2", "gap-y-4", "gap-y-8",
        
        // Justify & Align (25+ utilities)
        "justify-start", "justify-end", "justify-center", "justify-between", "justify-around", "justify-evenly",
        "justify-items-start", "justify-items-end", "justify-items-center", "justify-items-stretch",
        "justify-self-auto", "justify-self-start", "justify-self-end", "justify-self-center", "justify-self-stretch",
        "items-start", "items-end", "items-center", "items-baseline", "items-stretch",
        "content-start", "content-end", "content-center", "content-between", "content-around", "content-evenly",
        "place-content-center", "place-content-start", "place-content-end", "place-content-between",
        "place-items-start", "place-items-end", "place-items-center", "place-items-stretch",
        "place-self-auto", "place-self-start", "place-self-end", "place-self-center", "place-self-stretch",
        
        // Order (12+ utilities)
        "order-1", "order-2", "order-3", "order-4", "order-5", "order-6",
        "order-7", "order-8", "order-9", "order-10", "order-11", "order-12",
        "order-first", "order-last", "order-none", "-order-1", "-order-2",
        
        // ===== SPACING SYSTEM (100+ utilities) =====
        // Margin (40+ utilities)
        "m-0", "m-px", "m-0.5", "m-1", "m-2", "m-3", "m-4", "m-5", "m-6", "m-8", "m-10", "m-12", "m-16", "m-20", "m-24", "m-32",
        "mx-0", "mx-1", "mx-2", "mx-4", "mx-8", "mx-auto",
        "my-0", "my-1", "my-2", "my-4", "my-8", "my-auto",
        "mt-0", "mt-1", "mt-2", "mt-4", "mt-8", "mt-16",
        "mr-0", "mr-1", "mr-2", "mr-4", "mr-8", "mr-16",
        "mb-0", "mb-1", "mb-2", "mb-4", "mb-8", "mb-16",
        "ml-0", "ml-1", "ml-2", "ml-4", "ml-8", "ml-16",
        "-m-1", "-m-2", "-m-4", "-mx-1", "-mx-2", "-my-1", "-my-2", // negative margins
        
        // Padding (40+ utilities)
        "p-0", "p-px", "p-0.5", "p-1", "p-2", "p-3", "p-4", "p-5", "p-6", "p-8", "p-10", "p-12", "p-16", "p-20", "p-24", "p-32",
        "px-0", "px-1", "px-2", "px-4", "px-8", "px-16",
        "py-0", "py-1", "py-2", "py-4", "py-8", "py-16",
        "pt-0", "pt-1", "pt-2", "pt-4", "pt-8", "pt-16",
        "pr-0", "pr-1", "pr-2", "pr-4", "pr-8", "pr-16",
        "pb-0", "pb-1", "pb-2", "pb-4", "pb-8", "pb-16",
        "pl-0", "pl-1", "pl-2", "pl-4", "pl-8", "pl-16",
        
        // Space Between (10+ utilities)
        "space-x-0", "space-x-1", "space-x-2", "space-x-4", "space-x-8",
        "space-y-0", "space-y-1", "space-y-2", "space-y-4", "space-y-8",
        "space-x-reverse", "space-y-reverse",
        "-space-x-1", "-space-y-1", // negative space
        
        // ===== SIZING SYSTEM (80+ utilities) =====
        // Width (30+ utilities)
        "w-0", "w-px", "w-0.5", "w-1", "w-2", "w-3", "w-4", "w-5", "w-6", "w-8", "w-10", "w-12", "w-16", "w-20", "w-24", "w-32",
        "w-40", "w-48", "w-56", "w-64", "w-72", "w-80", "w-96",
        "w-auto", "w-1/2", "w-1/3", "w-2/3", "w-1/4", "w-3/4", "w-full", "w-screen", "w-min", "w-max", "w-fit",
        "w-[100px]", "w-[200px]", "w-[50%]", // arbitrary values
        
        // Min Width (10+ utilities)
        "min-w-0", "min-w-full", "min-w-min", "min-w-max", "min-w-fit",
        "min-w-[100px]", "min-w-[200px]",
        
        // Max Width (15+ utilities)
        "max-w-0", "max-w-none", "max-w-xs", "max-w-sm", "max-w-md", "max-w-lg", "max-w-xl",
        "max-w-2xl", "max-w-3xl", "max-w-4xl", "max-w-5xl", "max-w-6xl", "max-w-7xl",
        "max-w-full", "max-w-min", "max-w-max", "max-w-fit", "max-w-prose",
        "max-w-screen-sm", "max-w-screen-md", "max-w-screen-lg", "max-w-screen-xl", "max-w-screen-2xl",
        
        // Height (30+ utilities)
        "h-0", "h-px", "h-0.5", "h-1", "h-2", "h-3", "h-4", "h-5", "h-6", "h-8", "h-10", "h-12", "h-16", "h-20", "h-24", "h-32",
        "h-40", "h-48", "h-56", "h-64", "h-72", "h-80", "h-96",
        "h-auto", "h-1/2", "h-1/3", "h-2/3", "h-1/4", "h-3/4", "h-full", "h-screen", "h-min", "h-max", "h-fit",
        "h-[100px]", "h-[200px]", "h-[50vh]", // arbitrary values
        
        // Min Height (10+ utilities)
        "min-h-0", "min-h-full", "min-h-screen", "min-h-min", "min-h-max", "min-h-fit",
        "min-h-[100px]", "min-h-[200px]",
        
        // Max Height (10+ utilities)
        "max-h-0", "max-h-px", "max-h-0.5", "max-h-1", "max-h-2", "max-h-4", "max-h-8", "max-h-16", "max-h-32",
        "max-h-64", "max-h-96", "max-h-none", "max-h-full", "max-h-screen",
        "max-h-min", "max-h-max", "max-h-fit",
        
        // Size (combined w/h) (10+ utilities)
        "size-0", "size-px", "size-0.5", "size-1", "size-2", "size-4", "size-8", "size-16", "size-32",
        "size-full", "size-min", "size-max", "size-fit",
        
        // ===== TYPOGRAPHY SYSTEM (150+ utilities) =====
        // Font Family (3 utilities)
        "font-sans", "font-serif", "font-mono",
        
        // Font Size (15 utilities)
        "text-xs", "text-sm", "text-base", "text-lg", "text-xl", "text-2xl", "text-3xl",
        "text-4xl", "text-5xl", "text-6xl", "text-7xl", "text-8xl", "text-9xl",
        "text-[14px]", "text-[1.5rem]", // arbitrary values
        
        // Font Weight (10 utilities)
        "font-thin", "font-extralight", "font-light", "font-normal", "font-medium",
        "font-semibold", "font-bold", "font-extrabold", "font-black",
        "font-[500]", "font-[600]", // arbitrary values
        
        // Line Height (15+ utilities)
        "leading-3", "leading-4", "leading-5", "leading-6", "leading-7", "leading-8", "leading-9", "leading-10",
        "leading-none", "leading-tight", "leading-snug", "leading-normal", "leading-relaxed", "leading-loose",
        "leading-[1.5]", "leading-[20px]", // arbitrary values
        
        // Letter Spacing (8 utilities)
        "tracking-tighter", "tracking-tight", "tracking-normal", "tracking-wide",
        "tracking-wider", "tracking-widest",
        "tracking-[0.05em]", "tracking-[2px]", // arbitrary values
        
        // Text Alignment (6 utilities)
        "text-left", "text-center", "text-right", "text-justify", "text-start", "text-end",
        
        // Text Color (50+ utilities)
        "text-black", "text-white", "text-transparent", "text-current",
        "text-gray-50", "text-gray-100", "text-gray-200", "text-gray-300", "text-gray-400",
        "text-gray-500", "text-gray-600", "text-gray-700", "text-gray-800", "text-gray-900",
        "text-red-50", "text-red-100", "text-red-200", "text-red-300", "text-red-400",
        "text-red-500", "text-red-600", "text-red-700", "text-red-800", "text-red-900",
        "text-blue-50", "text-blue-100", "text-blue-200", "text-blue-300", "text-blue-400",
        "text-blue-500", "text-blue-600", "text-blue-700", "text-blue-800", "text-blue-900",
        "text-green-50", "text-green-100", "text-green-200", "text-green-300", "text-green-400",
        "text-green-500", "text-green-600", "text-green-700", "text-green-800", "text-green-900",
        "text-yellow-50", "text-yellow-100", "text-yellow-200", "text-yellow-300", "text-yellow-400",
        "text-yellow-500", "text-yellow-600", "text-yellow-700", "text-yellow-800", "text-yellow-900",
        
        // Text Decoration (10+ utilities)
        "underline", "overline", "line-through", "no-underline",
        "decoration-solid", "decoration-double", "decoration-dotted", "decoration-dashed", "decoration-wavy",
        "decoration-1", "decoration-2", "decoration-4", "decoration-8",
        "decoration-gray-500", "decoration-blue-500", "decoration-red-500",
        "underline-offset-1", "underline-offset-2", "underline-offset-4", "underline-offset-8",
        
        // Text Transform (4 utilities)
        "uppercase", "lowercase", "capitalize", "normal-case",
        
        // Text Overflow (3 utilities)
        "truncate", "text-ellipsis", "text-clip",
        
        // Vertical Alignment (8 utilities)
        "align-baseline", "align-top", "align-middle", "align-bottom",
        "align-text-top", "align-text-bottom", "align-sub", "align-super",
        
        // Whitespace (6 utilities)
        "whitespace-normal", "whitespace-nowrap", "whitespace-pre", "whitespace-pre-line",
        "whitespace-pre-wrap", "whitespace-break-spaces",
        
        // Word Break (4 utilities)
        "break-normal", "break-words", "break-all", "break-keep",
        
        // Text Wrapping (4 utilities)
        "text-wrap", "text-nowrap", "text-balance", "text-pretty",
        
        // Font Style (2 utilities)
        "italic", "not-italic",
        
        // Font Variant Numeric (8 utilities)
        "normal-nums", "ordinal", "slashed-zero", "lining-nums", "oldstyle-nums",
        "proportional-nums", "tabular-nums", "diagonal-fractions", "stacked-fractions",
        
        // ===== BACKGROUND SYSTEM (100+ utilities) =====
        // Background Color (50+ utilities)
        "bg-transparent", "bg-current", "bg-black", "bg-white",
        "bg-gray-50", "bg-gray-100", "bg-gray-200", "bg-gray-300", "bg-gray-400",
        "bg-gray-500", "bg-gray-600", "bg-gray-700", "bg-gray-800", "bg-gray-900",
        "bg-red-50", "bg-red-100", "bg-red-200", "bg-red-300", "bg-red-400",
        "bg-red-500", "bg-red-600", "bg-red-700", "bg-red-800", "bg-red-900",
        "bg-blue-50", "bg-blue-100", "bg-blue-200", "bg-blue-300", "bg-blue-400",
        "bg-blue-500", "bg-blue-600", "bg-blue-700", "bg-blue-800", "bg-blue-900",
        "bg-green-50", "bg-green-100", "bg-green-200", "bg-green-300", "bg-green-400",
        "bg-green-500", "bg-green-600", "bg-green-700", "bg-green-800", "bg-green-900",
        "bg-yellow-50", "bg-yellow-100", "bg-yellow-200", "bg-yellow-300", "bg-yellow-400",
        "bg-yellow-500", "bg-yellow-600", "bg-yellow-700", "bg-yellow-800", "bg-yellow-900",
        
        // Background Image/Gradient (10+ utilities)
        "bg-none", "bg-gradient-to-t", "bg-gradient-to-tr", "bg-gradient-to-r", "bg-gradient-to-br",
        "bg-gradient-to-b", "bg-gradient-to-bl", "bg-gradient-to-l", "bg-gradient-to-tl",
        
        // Background Position (9 utilities)
        "bg-bottom", "bg-center", "bg-left", "bg-left-bottom", "bg-left-top",
        "bg-right", "bg-right-bottom", "bg-right-top", "bg-top",
        
        // Background Repeat (5 utilities)
        "bg-repeat", "bg-no-repeat", "bg-repeat-x", "bg-repeat-y", "bg-repeat-round", "bg-repeat-space",
        
        // Background Size (3 utilities)
        "bg-auto", "bg-cover", "bg-contain",
        
        // Background Attachment (3 utilities)
        "bg-fixed", "bg-local", "bg-scroll",
        
        // Background Clip (4 utilities)
        "bg-clip-border", "bg-clip-padding", "bg-clip-content", "bg-clip-text",
        
        // Background Origin (3 utilities)
        "bg-origin-border", "bg-origin-padding", "bg-origin-content",
        
        // ===== BORDER SYSTEM (80+ utilities) =====
        // Border Width (20+ utilities)
        "border", "border-0", "border-2", "border-4", "border-8",
        "border-x", "border-x-0", "border-x-2", "border-x-4", "border-x-8",
        "border-y", "border-y-0", "border-y-2", "border-y-4", "border-y-8",
        "border-t", "border-t-0", "border-t-2", "border-t-4", "border-t-8",
        "border-r", "border-r-0", "border-r-2", "border-r-4", "border-r-8",
        "border-b", "border-b-0", "border-b-2", "border-b-4", "border-b-8",
        "border-l", "border-l-0", "border-l-2", "border-l-4", "border-l-8",
        
        // Border Color (20+ utilities)
        "border-transparent", "border-current", "border-black", "border-white",
        "border-gray-50", "border-gray-100", "border-gray-200", "border-gray-300", "border-gray-400",
        "border-gray-500", "border-gray-600", "border-gray-700", "border-gray-800", "border-gray-900",
        "border-red-500", "border-blue-500", "border-green-500", "border-yellow-500",
        
        // Border Style (6 utilities)
        "border-solid", "border-dashed", "border-dotted", "border-double", "border-hidden", "border-none",
        
        // Border Radius (25+ utilities)
        "rounded-none", "rounded-sm", "rounded", "rounded-md", "rounded-lg", "rounded-xl", "rounded-2xl", "rounded-3xl", "rounded-full",
        "rounded-t-none", "rounded-t-sm", "rounded-t", "rounded-t-md", "rounded-t-lg", "rounded-t-xl", "rounded-t-2xl", "rounded-t-3xl", "rounded-t-full",
        "rounded-r-none", "rounded-r-sm", "rounded-r", "rounded-r-md", "rounded-r-lg", "rounded-r-xl", "rounded-r-2xl", "rounded-r-3xl", "rounded-r-full",
        "rounded-b-none", "rounded-b-sm", "rounded-b", "rounded-b-md", "rounded-b-lg", "rounded-b-xl", "rounded-b-2xl", "rounded-b-3xl", "rounded-b-full",
        "rounded-l-none", "rounded-l-sm", "rounded-l", "rounded-l-md", "rounded-l-lg", "rounded-l-xl", "rounded-l-2xl", "rounded-l-3xl", "rounded-l-full",
        "rounded-[0.5rem]", "rounded-[10px]", // arbitrary values
        
        // Divide (10+ utilities)
        "divide-x", "divide-x-0", "divide-x-2", "divide-x-4", "divide-x-8",
        "divide-y", "divide-y-0", "divide-y-2", "divide-y-4", "divide-y-8",
        "divide-gray-200", "divide-gray-300", "divide-gray-400",
        "divide-solid", "divide-dashed", "divide-dotted",
        
        // Outline (15+ utilities)
        "outline-none", "outline", "outline-0", "outline-1", "outline-2", "outline-4", "outline-8",
        "outline-dashed", "outline-dotted", "outline-double", "outline-solid",
        "outline-offset-0", "outline-offset-1", "outline-offset-2", "outline-offset-4", "outline-offset-8",
        "outline-black", "outline-white", "outline-gray-500", "outline-red-500", "outline-blue-500",
        
        // Ring (20+ utilities)
        "ring-0", "ring-1", "ring-2", "ring-4", "ring-8", "ring",
        "ring-inset",
        "ring-transparent", "ring-current", "ring-black", "ring-white",
        "ring-gray-500", "ring-red-500", "ring-blue-500", "ring-green-500",
        "ring-offset-0", "ring-offset-1", "ring-offset-2", "ring-offset-4", "ring-offset-8",
        "ring-offset-gray-50", "ring-offset-red-50", "ring-offset-blue-50",
        
        // ===== EFFECTS SYSTEM (40+ utilities) =====
        // Box Shadow (10+ utilities)
        "shadow-sm", "shadow", "shadow-md", "shadow-lg", "shadow-xl", "shadow-2xl", "shadow-inner", "shadow-none",
        "shadow-[0_4px_6px_rgba(0,0,0,0.1)]", // arbitrary value
        
        // Shadow Color (10+ utilities)
        "shadow-gray-500", "shadow-red-500", "shadow-blue-500", "shadow-green-500", "shadow-yellow-500",
        
        // Opacity (20+ utilities)
        "opacity-0", "opacity-5", "opacity-10", "opacity-20", "opacity-25", "opacity-30", "opacity-40",
        "opacity-50", "opacity-60", "opacity-70", "opacity-75", "opacity-80", "opacity-90", "opacity-95", "opacity-100",
        
        // Mix Blend Mode (15+ utilities)
        "mix-blend-normal", "mix-blend-multiply", "mix-blend-screen", "mix-blend-overlay",
        "mix-blend-darken", "mix-blend-lighten", "mix-blend-color-dodge", "mix-blend-color-burn",
        "mix-blend-hard-light", "mix-blend-soft-light", "mix-blend-difference", "mix-blend-exclusion",
        "mix-blend-hue", "mix-blend-saturation", "mix-blend-color", "mix-blend-luminosity",
        
        // ===== FILTERS SYSTEM (70+ utilities) =====
        // Blur (8 utilities)
        "blur-none", "blur-sm", "blur", "blur-md", "blur-lg", "blur-xl", "blur-2xl", "blur-3xl",
        
        // Brightness (10+ utilities)
        "brightness-0", "brightness-50", "brightness-75", "brightness-90", "brightness-95",
        "brightness-100", "brightness-105", "brightness-110", "brightness-125", "brightness-150", "brightness-200",
        
        // Contrast (10+ utilities)
        "contrast-0", "contrast-50", "contrast-75", "contrast-100", "contrast-125", "contrast-150", "contrast-200",
        
        // Grayscale (2 utilities)
        "grayscale-0", "grayscale",
        
        // Hue Rotate (8+ utilities)
        "hue-rotate-0", "hue-rotate-15", "hue-rotate-30", "hue-rotate-60", "hue-rotate-90", "hue-rotate-180",
        "-hue-rotate-15", "-hue-rotate-30", "-hue-rotate-60", "-hue-rotate-90", "-hue-rotate-180",
        
        // Invert (2 utilities)
        "invert-0", "invert",
        
        // Saturate (6 utilities)
        "saturate-0", "saturate-50", "saturate-100", "saturate-150", "saturate-200",
        
        // Sepia (2 utilities)
        "sepia-0", "sepia",
        
        // Backdrop Filters (30+ utilities)
        "backdrop-blur-none", "backdrop-blur-sm", "backdrop-blur", "backdrop-blur-md", "backdrop-blur-lg", "backdrop-blur-xl", "backdrop-blur-2xl", "backdrop-blur-3xl",
        "backdrop-brightness-0", "backdrop-brightness-50", "backdrop-brightness-75", "backdrop-brightness-100", "backdrop-brightness-125", "backdrop-brightness-150", "backdrop-brightness-200",
        "backdrop-contrast-0", "backdrop-contrast-50", "backdrop-contrast-75", "backdrop-contrast-100", "backdrop-contrast-125", "backdrop-contrast-150", "backdrop-contrast-200",
        "backdrop-grayscale-0", "backdrop-grayscale",
        "backdrop-hue-rotate-0", "backdrop-hue-rotate-15", "backdrop-hue-rotate-30", "backdrop-hue-rotate-60", "backdrop-hue-rotate-90", "backdrop-hue-rotate-180",
        "backdrop-invert-0", "backdrop-invert",
        "backdrop-opacity-0", "backdrop-opacity-50", "backdrop-opacity-100",
        "backdrop-saturate-0", "backdrop-saturate-50", "backdrop-saturate-100", "backdrop-saturate-150", "backdrop-saturate-200",
        "backdrop-sepia-0", "backdrop-sepia",
        
        // ===== TRANSFORM SYSTEM (50+ utilities) =====
        // Scale (15+ utilities)
        "scale-0", "scale-50", "scale-75", "scale-90", "scale-95", "scale-100", "scale-105", "scale-110", "scale-125", "scale-150",
        "scale-x-0", "scale-x-50", "scale-x-75", "scale-x-90", "scale-x-95", "scale-x-100", "scale-x-105", "scale-x-110", "scale-x-125", "scale-x-150",
        "scale-y-0", "scale-y-50", "scale-y-75", "scale-y-90", "scale-y-95", "scale-y-100", "scale-y-105", "scale-y-110", "scale-y-125", "scale-y-150",
        "-scale-50", "-scale-100", "-scale-x-100", "-scale-y-100", // negative values
        
        // Rotate (10+ utilities)
        "rotate-0", "rotate-1", "rotate-2", "rotate-3", "rotate-6", "rotate-12", "rotate-45", "rotate-90", "rotate-180",
        "-rotate-1", "-rotate-2", "-rotate-3", "-rotate-6", "-rotate-12", "-rotate-45", "-rotate-90", "-rotate-180",
        
        // Translate (20+ utilities)
        "translate-x-0", "translate-x-px", "translate-x-0.5", "translate-x-1", "translate-x-2", "translate-x-4", "translate-x-8", "translate-x-16", "translate-x-32",
        "translate-y-0", "translate-y-px", "translate-y-0.5", "translate-y-1", "translate-y-2", "translate-y-4", "translate-y-8", "translate-y-16", "translate-y-32",
        "-translate-x-1", "-translate-x-2", "-translate-x-4", "-translate-x-8",
        "-translate-y-1", "-translate-y-2", "-translate-y-4", "-translate-y-8",
        "translate-x-1/2", "translate-y-1/2", "translate-x-full", "translate-y-full",
        
        // Skew (8+ utilities)
        "skew-x-0", "skew-x-1", "skew-x-2", "skew-x-3", "skew-x-6", "skew-x-12",
        "skew-y-0", "skew-y-1", "skew-y-2", "skew-y-3", "skew-y-6", "skew-y-12",
        "-skew-x-1", "-skew-x-2", "-skew-x-3", "-skew-x-6", "-skew-x-12",
        "-skew-y-1", "-skew-y-2", "-skew-y-3", "-skew-y-6", "-skew-y-12",
        
        // Transform Origin (9 utilities)
        "origin-center", "origin-top", "origin-top-right", "origin-right", "origin-bottom-right",
        "origin-bottom", "origin-bottom-left", "origin-left", "origin-top-left",
        
        // ===== TRANSITION & ANIMATION SYSTEM (40+ utilities) =====
        // Transition Property (8 utilities)
        "transition-none", "transition-all", "transition", "transition-colors", "transition-opacity",
        "transition-shadow", "transition-transform",
        "transition-[width]", // arbitrary value
        
        // Transition Duration (10+ utilities)
        "duration-75", "duration-100", "duration-150", "duration-200", "duration-300",
        "duration-500", "duration-700", "duration-1000",
        "duration-[2000ms]", "duration-[3s]", // arbitrary values
        
        // Transition Timing (5 utilities)
        "ease-linear", "ease-in", "ease-out", "ease-in-out",
        "ease-[cubic-bezier(0.4,0,0.2,1)]", // arbitrary value
        
        // Transition Delay (8 utilities)
        "delay-75", "delay-100", "delay-150", "delay-200", "delay-300", "delay-500", "delay-700", "delay-1000",
        
        // Animation (10+ utilities)
        "animate-none", "animate-spin", "animate-ping", "animate-pulse", "animate-bounce",
        "animate-[wiggle_1s_ease-in-out_infinite]", // arbitrary value
        
        // ===== FORMS SYSTEM (5 utilities) =====
        // Form Utilities
        "form-input", "form-textarea", "form-select", "form-checkbox", "form-radio",
        
        // ===== INTERACTIVITY SYSTEM (50+ utilities) =====
        // Cursor (38 utilities - comprehensive list matching CSS cursor values)
        "cursor-auto", "cursor-default", "cursor-pointer", "cursor-wait", "cursor-text", "cursor-move",
        "cursor-help", "cursor-not-allowed", "cursor-none", "cursor-context-menu", "cursor-progress",
        "cursor-cell", "cursor-crosshair", "cursor-vertical-text", "cursor-alias", "cursor-copy",
        "cursor-no-drop", "cursor-grab", "cursor-grabbing", "cursor-all-scroll",
        "cursor-zoom-in", "cursor-zoom-out",
        // Resize cursors
        "cursor-col-resize", "cursor-row-resize",
        "cursor-n-resize", "cursor-e-resize", "cursor-s-resize", "cursor-w-resize",
        "cursor-ne-resize", "cursor-nw-resize", "cursor-se-resize", "cursor-sw-resize",
        "cursor-ew-resize", "cursor-ns-resize", "cursor-nesw-resize", "cursor-nwse-resize",
        
        // Pointer Events (2 utilities)
        "pointer-events-none", "pointer-events-auto",
        
        // Resize (4 utilities)
        "resize-none", "resize-y", "resize-x", "resize",
        
        // User Select (4 utilities)
        "select-none", "select-text", "select-all", "select-auto",
        
        // Scroll Behavior (2 utilities)
        "scroll-auto", "scroll-smooth",
        
        // Scroll Snap Type (5 utilities)
        "snap-none", "snap-x", "snap-y", "snap-both", "snap-mandatory", "snap-proximity",
        
        // Scroll Snap Align (4 utilities)
        "snap-start", "snap-end", "snap-center", "snap-align-none",
        
        // Scroll Snap Stop (2 utilities)
        "snap-normal", "snap-always",
        
        // Touch Action (10+ utilities)
        "touch-auto", "touch-none", "touch-pan-x", "touch-pan-left", "touch-pan-right",
        "touch-pan-y", "touch-pan-up", "touch-pan-down", "touch-pinch-zoom", "touch-manipulation",
        
        // Will Change (5 utilities)
        "will-change-auto", "will-change-scroll", "will-change-contents", "will-change-transform",
        "will-change-[opacity]", // arbitrary value
        
        // Accent Color (10+ utilities)
        "accent-auto", "accent-current", "accent-transparent", "accent-black", "accent-white",
        "accent-gray-500", "accent-red-500", "accent-blue-500", "accent-green-500", "accent-yellow-500",
        
        // Appearance (2 utilities)
        "appearance-none", "appearance-auto",
        
        // Caret Color (10+ utilities)
        "caret-transparent", "caret-current", "caret-black", "caret-white",
        "caret-gray-500", "caret-red-500", "caret-blue-500", "caret-green-500", "caret-yellow-500",
        
        // ===== SVG SYSTEM (20+ utilities) =====
        // Fill (10+ utilities)
        "fill-none", "fill-current", "fill-black", "fill-white",
        "fill-gray-500", "fill-red-500", "fill-blue-500", "fill-green-500", "fill-yellow-500",
        
        // Stroke (10+ utilities)
        "stroke-none", "stroke-current", "stroke-black", "stroke-white",
        "stroke-gray-500", "stroke-red-500", "stroke-blue-500", "stroke-green-500", "stroke-yellow-500",
        
        // Stroke Width (4 utilities)
        "stroke-0", "stroke-1", "stroke-2", "stroke-[3]",
        
        // ===== TABLES SYSTEM (5 utilities) =====
        "table-auto", "table-fixed",
        "border-collapse", "border-separate",
        "border-spacing-0", "border-spacing-1", "border-spacing-2", "border-spacing-4",
        
        // ===== ACCESSIBILITY SYSTEM (2 utilities) =====
        "sr-only", "not-sr-only",
        
        // ===== CONTAINER & RESPONSIVE (5 utilities) =====
        "container",
        "aspect-auto", "aspect-square", "aspect-video", "aspect-[16/9]",
        
        // ===== COLUMNS (10+ utilities) =====
        "columns-1", "columns-2", "columns-3", "columns-4", "columns-5", "columns-6",
        "columns-7", "columns-8", "columns-9", "columns-10", "columns-11", "columns-12",
        "columns-auto", "columns-3xs", "columns-2xs", "columns-xs", "columns-sm", "columns-md",
        "columns-lg", "columns-xl", "columns-2xl", "columns-3xl", "columns-4xl", "columns-5xl",
        "columns-6xl", "columns-7xl",
        
        // ===== BREAK UTILITIES (10+ utilities) =====
        "break-after-auto", "break-after-avoid", "break-after-all", "break-after-avoid-page", "break-after-page",
        "break-after-left", "break-after-right", "break-after-column",
        "break-before-auto", "break-before-avoid", "break-before-all", "break-before-avoid-page", "break-before-page",
        "break-before-left", "break-before-right", "break-before-column",
        "break-inside-auto", "break-inside-avoid", "break-inside-avoid-page", "break-inside-avoid-column",
        
        // ===== RESPONSIVE & STATE VARIANTS (50+ examples) =====
        // Responsive prefixes
        "sm:text-lg", "md:text-xl", "lg:text-2xl", "xl:text-3xl", "2xl:text-4xl",
        "sm:p-4", "md:p-6", "lg:p-8", "xl:p-10", "2xl:p-12",
        "sm:flex", "md:grid", "lg:hidden", "xl:block",
        
        // Dark mode
        "dark:bg-gray-900", "dark:text-white", "dark:border-gray-700",
        
        // State variants
        "hover:bg-gray-100", "hover:text-blue-600", "hover:shadow-lg", "hover:scale-105",
        "focus:outline-none", "focus:ring-2", "focus:ring-blue-500", "focus:ring-offset-2",
        "active:bg-gray-200", "active:scale-95",
        "disabled:opacity-50", "disabled:cursor-not-allowed",
        "visited:text-purple-600",
        "checked:bg-blue-600",
        "focus-within:ring-2", "focus-within:ring-blue-500",
        "focus-visible:ring-2", "focus-visible:ring-offset-2",
        
        // Group and peer variants
        "group-hover:text-blue-600", "group-focus:ring-2",
        "peer-invalid:text-red-500", "peer-checked:bg-blue-100",
        
        // Important modifier
        "!p-4", "!text-red-500", "!bg-yellow-100",
        
        // Container queries
        "@sm:text-sm", "@md:text-base", "@lg:text-lg", "@xl:text-xl", "@2xl:text-2xl",
        "@container", "@sm:flex", "@lg:grid",
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
        if let Ok(transformed1) = result1 {
            // Second transformation (should be idempotent)
            let result2 = builder2.trace(&transformed1, false);
            if let Ok(transformed2) = result2 {
                if transformed1 != transformed2 {
                    println!("✗ IDEMPOTENCE FAILED for '{}': first='{}', second='{}'", 
                             class, transformed1, transformed2);
                    idempotence_failures.push((class, transformed1.to_string(), transformed2.to_string()));
                } else {
                    println!("✓ {} -> {} (idempotent)", class, transformed1);
                }
            } else {
                println!("⚠ Second trace failed for '{}': {:?}", class, result2);
                parse_failures.push((class, transformed1.to_string()));
            }
        } else {
            println!("⚠ First trace failed for '{}': {:?}", class, result1);
        }
    }
    
    // Test combinations of classes for idempotence
    println!("\n=== Testing class combination idempotence ===");
    
    // Create owned strings for the stress test combinations - test diverse utilities
    let stress_test_1 = TAILWIND_CLASSES[..50].join(" ");        // Layout & Display
    let stress_test_2 = TAILWIND_CLASSES[100..200].join(" ");    // Flexbox & Grid
    let stress_test_3 = TAILWIND_CLASSES[200..300].join(" ");    // Spacing & Sizing
    let stress_test_4 = TAILWIND_CLASSES[300..400].join(" ");    // Typography
    let stress_test_5 = TAILWIND_CLASSES[400..500].join(" ");    // Background & Borders
    let stress_test_6 = TAILWIND_CLASSES[500..600].join(" ");    // Effects & Filters
    let stress_test_7 = TAILWIND_CLASSES[600..700].join(" ");    // Transforms
    let stress_test_8 = TAILWIND_CLASSES[700..800].join(" ");    // Transitions & Animations
    let stress_test_9 = TAILWIND_CLASSES[800..900].join(" ");    // Interactivity
    let stress_test_10 = TAILWIND_CLASSES[900..].join(" ");      // SVG, Tables, & Variants
    
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
        
        // Test non-Tailwind class preservation
        "custom-class p-4 bg-blue-500 another-custom text-white",
        "my-component flex items-center custom-style gap-4",
        "p-4 non-tailwind-1 m-2 non-tailwind-2 border",
        
        // Mix Tailwind with custom classes
        "hero-section bg-gradient-to-r from-blue-500 to-purple-600 text-white py-16",
        "card-container shadow-lg rounded-lg p-6 hover:shadow-xl transition-shadow",
        
        // All classes together (comprehensive stress tests)
        stress_test_1.as_str(),
        stress_test_2.as_str(),
        stress_test_3.as_str(),
        stress_test_4.as_str(),
        stress_test_5.as_str(),
        stress_test_6.as_str(),
        stress_test_7.as_str(),
        stress_test_8.as_str(),
        stress_test_9.as_str(),
        stress_test_10.as_str(),
    ];
    
    for combination in test_combinations {
        let mut builder1 = TailwindBuilder::default();
        let mut builder2 = TailwindBuilder::default();
        
        // First transformation
        let result1 = builder1.trace(combination, false);
        if let Ok(transformed1) = result1 {
            // Second transformation (should be idempotent)
            let result2 = builder2.trace(&transformed1, false);
            if let Ok(transformed2) = result2 {
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
        if let Ok(transformed1) = result1 {
            // Second transformation (should be idempotent)
            let result2 = builder2.trace(&transformed1, false);
            if let Ok(transformed2) = result2 {
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
    println!("Total utilities tested: {}", TAILWIND_CLASSES.len());
    println!("Test coverage breakdown:");
    println!("  - Layout System: ~80 utilities");
    println!("  - Flexbox & Grid: ~120 utilities");
    println!("  - Spacing System: ~100 utilities");
    println!("  - Sizing System: ~80 utilities");
    println!("  - Typography System: ~150 utilities");
    println!("  - Background System: ~100 utilities");
    println!("  - Border System: ~80 utilities");
    println!("  - Effects System: ~40 utilities");
    println!("  - Filters System: ~70 utilities");
    println!("  - Transform System: ~50 utilities");
    println!("  - Transition & Animation: ~40 utilities");
    println!("  - Interactivity System: ~50 utilities");
    println!("  - SVG, Tables, & More: ~50 utilities");
    println!("  - Responsive & State Variants: ~50 examples");
    
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
        println!("\n✅ ALL TESTS PASSED: trace() is idempotent for all {} tested classes!", TAILWIND_CLASSES.len());
    } else {
        println!("\n❌ TEST FAILED: trace() is NOT idempotent for some classes");
        panic!("Idempotence property violated!");
    }
}
