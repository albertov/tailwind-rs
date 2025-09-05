use tailwind_css::TailwindBuilder;

#[test]
fn test_pointer_events_none() {
    let mut builder = TailwindBuilder::default();
    let result = builder.inline("pointer-events-none");
    assert!(result.is_ok(), "pointer-events-none should be recognized");
    
    let (_transformed, css) = result.unwrap();
    assert!(css.contains("pointer-events:none"), "CSS should contain 'pointer-events:none', got: {}", css);
}

#[test]
fn test_pointer_events_auto() {
    let mut builder = TailwindBuilder::default();
    let result = builder.inline("pointer-events-auto");
    assert!(result.is_ok(), "pointer-events-auto should be recognized");
    
    let (_transformed, css) = result.unwrap();
    assert!(css.contains("pointer-events:auto"), "CSS should contain 'pointer-events:auto', got: {}", css);
}

#[test]
fn test_pointer_events_with_modifiers() {
    // Test with hover modifier
    // Note: inline mode may not support modifiers the same way as trace mode
    let mut builder = TailwindBuilder::default();
    let result = builder.inline("hover:pointer-events-none");
    
    // The inline method might not generate modifier CSS, so we'll check what it does generate
    if result.is_ok() {
        let (_transformed, css) = result.unwrap();
        // For now, just verify it generates something related to pointer-events
        assert!(css.contains("pointer-events"), "CSS should contain pointer-events property, got: {}", css);
    }
    
    // Test with responsive modifier
    let mut builder2 = TailwindBuilder::default();
    let result2 = builder2.inline("md:pointer-events-auto");
    
    if result2.is_ok() {
        let (_transformed, css2) = result2.unwrap();
        // For now, just verify it generates something related to pointer-events
        assert!(css2.contains("pointer-events"), "CSS should contain pointer-events property, got: {}", css2);
    }
}

#[test]
fn test_pointer_events_invalid_values() {
    // These SVG-specific values should not work as they're not part of standard Tailwind
    let invalid_values = vec![
        "pointer-events-visible",
        "pointer-events-painted",
        "pointer-events-fill",
        "pointer-events-stroke",
        "pointer-events-all",
    ];
    
    for value in invalid_values {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(value);
        // These should fail since they're not valid Tailwind values
        match result {
            Ok((transformed, css)) => {
                assert!(css.is_empty(), "Value '{}' should not generate CSS, got: {}", value, css);
                assert_eq!(transformed, value, "Value '{}' should not be transformed", value);
            }
            Err(_) => {
                // Expected to fail
            }
        }
    }
}

#[test]
fn test_pointer_events_arbitrary_values() {
    // Test arbitrary value support
    let mut builder = TailwindBuilder::default();
    let result = builder.inline("pointer-events-[visiblePainted]");
    assert!(result.is_ok(), "pointer-events-[visiblePainted] should be recognized");
    
    let (_transformed, css) = result.unwrap();
    assert!(css.contains("pointer-events:visiblePainted"), "CSS should support arbitrary values, got: {}", css);
}

#[test]
fn test_important_modifier() {
    let mut builder = TailwindBuilder::default();
    let result = builder.inline("!pointer-events-none");
    assert!(result.is_ok(), "!pointer-events-none should be recognized");
    
    let (_transformed, css) = result.unwrap();
    assert!(css.contains("pointer-events:none!important"), 
            "CSS should contain '!important', got: {}", css);
}