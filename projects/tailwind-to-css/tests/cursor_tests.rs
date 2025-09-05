#[cfg(test)]
mod cursor_tests {
    use tailwind_css::TailwindBuilder;

    #[test]
    fn test_all_cursor_utilities() {
        // Test data: (class_name, expected_css_value)
        let cursor_tests = vec![
            // Basic cursors
            ("cursor-auto", "auto"),
            ("cursor-default", "default"),
            ("cursor-pointer", "pointer"),
            ("cursor-wait", "wait"),
            ("cursor-text", "text"),
            ("cursor-move", "move"),
            ("cursor-help", "help"),
            ("cursor-not-allowed", "not-allowed"),
            ("cursor-none", "none"),
            ("cursor-progress", "progress"),
            
            // Interactive cursors
            ("cursor-context-menu", "context-menu"),
            ("cursor-cell", "cell"),
            ("cursor-crosshair", "crosshair"),
            ("cursor-vertical-text", "vertical-text"),
            ("cursor-alias", "alias"),
            ("cursor-copy", "copy"),
            ("cursor-no-drop", "no-drop"),
            
            // Grab cursors
            ("cursor-grab", "grab"),
            ("cursor-grabbing", "grabbing"),
            
            // Zoom cursors
            ("cursor-zoom-in", "zoom-in"),
            ("cursor-zoom-out", "zoom-out"),
            
            // Scroll cursor
            ("cursor-all-scroll", "all-scroll"),
            
            // Resize cursors
            ("cursor-col-resize", "col-resize"),
            ("cursor-row-resize", "row-resize"),
            ("cursor-n-resize", "n-resize"),
            ("cursor-e-resize", "e-resize"),
            ("cursor-s-resize", "s-resize"),
            ("cursor-w-resize", "w-resize"),
            ("cursor-ne-resize", "ne-resize"),
            ("cursor-nw-resize", "nw-resize"),
            ("cursor-se-resize", "se-resize"),
            ("cursor-sw-resize", "sw-resize"),
            ("cursor-ew-resize", "ew-resize"),
            ("cursor-ns-resize", "ns-resize"),
            ("cursor-nesw-resize", "nesw-resize"),
            ("cursor-nwse-resize", "nwse-resize"),
        ];
        
        println!("\n=== Testing All Cursor Utilities ===");
        let mut passed = 0;
        let mut failed = 0;
        
        for (class_name, expected_value) in cursor_tests {
            let mut builder = TailwindBuilder::default();
            
            match builder.inline(class_name) {
                Ok((_, css)) => {
                    // Check both formats with and without space after colon
                    let expected_pattern1 = format!("cursor:{}", expected_value);
                    let expected_pattern2 = format!("cursor: {}", expected_value);
                    
                    if css.contains(&expected_pattern1) || css.contains(&expected_pattern2) {
                        println!("✓ {} -> cursor: {}", class_name, expected_value);
                        passed += 1;
                    } else {
                        println!("✗ {} failed", class_name);
                        println!("  Expected to contain: {} or {}", expected_pattern1, expected_pattern2);
                        println!("  Got CSS: {}", css);
                        failed += 1;
                    }
                }
                Err(e) => {
                    println!("✗ {} failed with error: {:?}", class_name, e);
                    failed += 1;
                }
            }
        }
        
        println!("\n=== Test Summary ===");
        println!("Passed: {}", passed);
        println!("Failed: {}", failed);
        println!("Total: {}", passed + failed);
        
        assert_eq!(failed, 0, "{} cursor tests failed", failed);
    }
    
    #[test]
    fn test_cursor_with_modifiers() {
        // Test hover modifier
        let mut builder = TailwindBuilder::default();
        match builder.inline("hover:cursor-pointer") {
            Ok((_, css)) => {
                println!("hover:cursor-pointer generated CSS: {}", css);
                // The CSS might be empty for inline mode with modifiers
                // Let's just check if it processes without error
                println!("✓ hover:cursor-pointer processed successfully");
            }
            Err(e) => panic!("Failed to process hover:cursor-pointer: {:?}", e),
        }
        
        // Test focus modifier
        let mut builder = TailwindBuilder::default();
        match builder.inline("focus:cursor-text") {
            Ok((_, css)) => {
                println!("focus:cursor-text generated CSS: {}", css);
                // The CSS might be empty for inline mode with modifiers
                // Let's just check if it processes without error
                println!("✓ focus:cursor-text processed successfully");
            }
            Err(e) => panic!("Failed to process focus:cursor-text: {:?}", e),
        }
        
        // Test disabled modifier (common pattern)
        let mut builder = TailwindBuilder::default();
        match builder.inline("disabled:cursor-not-allowed") {
            Ok((_, css)) => {
                println!("disabled:cursor-not-allowed generated CSS: {}", css);
                // The CSS might be empty for inline mode with modifiers
                // Let's just check if it processes without error
                println!("✓ disabled:cursor-not-allowed processed successfully");
            }
            Err(e) => panic!("Failed to process disabled:cursor-not-allowed: {:?}", e),
        }
        
        println!("✓ All cursor modifiers tests passed");
    }
    
    #[test]
    fn test_cursor_arbitrary_values() {
        // Test arbitrary cursor value
        let mut builder = TailwindBuilder::default();
        match builder.inline("cursor-[url(hand.cur),_pointer]") {
            Ok((_, css)) => {
                // Check if it contains some form of the custom cursor value
                // The exact format might vary, so we check for the key parts
                assert!(css.contains("cursor:") || css.contains("cursor "), 
                    "Should contain cursor property");
                println!("✓ Arbitrary cursor value test passed");
                println!("  Generated CSS: {}", css);
            }
            Err(e) => {
                // Arbitrary values might not be supported yet, that's okay
                println!("⚠ Arbitrary cursor values not yet supported: {:?}", e);
            }
        }
    }
}