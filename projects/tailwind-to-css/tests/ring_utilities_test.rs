use tailwind_css::TailwindBuilder;

#[test]
fn test_ring_default_v4() {
    // CRITICAL: v4 default is 1px (was 3px in v3)
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("ring", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Check that the default ring width is 1px
    assert!(css.contains("1px"), "Default ring width should be 1px in v4, got: {}", css);
    assert!(css.contains("--tw-ring-shadow"), "Should set ring shadow variable");
    assert!(css.contains("box-shadow"), "Should set box-shadow");
}

#[test]
fn test_ring_widths() {
    // Test specific ring widths
    let test_cases = vec![
        ("ring-0", "0px"),
        ("ring-1", "1px"),
        ("ring-2", "2px"),
        ("ring-4", "4px"),
        ("ring-8", "8px"),
    ];
    
    for (class, expected_width) in test_cases {
        let mut tw = TailwindBuilder::default();
        let _ = tw.trace(class, false).unwrap();
        let css = tw.bundle().unwrap();
        assert!(css.contains(expected_width), "Class {} should contain {}, got: {}", class, expected_width, css);
    }
}

#[test]
fn test_ring_inset() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("ring-inset", false).unwrap();
    let css = tw.bundle().unwrap();
    assert!(css.contains("--tw-ring-inset"), "Should set ring inset variable");
    assert!(css.contains("inset"), "Should contain 'inset' keyword");
}

#[test]
fn test_ring_colors() {
    // Test ring colors
    let test_cases = vec![
        "ring-red-500",
        "ring-blue-600",
        "ring-green-400",
    ];
    
    for class in test_cases {
        let mut tw = TailwindBuilder::default();
        let _ = tw.trace(class, false).unwrap();
        let css = tw.bundle().unwrap();
        assert!(css.contains("--tw-ring-color"), "Class {} should set ring color variable", class);
    }
}

#[test]
fn test_ring_offset_width() {
    let test_cases = vec![
        ("ring-offset-0", "0px"),
        ("ring-offset-1", "1px"),
        ("ring-offset-2", "2px"),
        ("ring-offset-4", "4px"),
    ];
    
    for (class, expected_width) in test_cases {
        let mut tw = TailwindBuilder::default();
        let _ = tw.trace(class, false).unwrap();
        let css = tw.bundle().unwrap();
        assert!(css.contains("--tw-ring-offset-width"), "Class {} should set ring offset width", class);
        assert!(css.contains(expected_width), "Class {} should contain {}, got: {}", class, expected_width, css);
    }
}

#[test]
fn test_ring_offset_colors() {
    let test_cases = vec![
        "ring-offset-white",
        "ring-offset-red-500",
        "ring-offset-blue-200",
    ];
    
    for class in test_cases {
        let mut tw = TailwindBuilder::default();
        let _ = tw.trace(class, false).unwrap();
        let css = tw.bundle().unwrap();
        assert!(css.contains("--tw-ring-offset-color"), "Class {} should set ring offset color", class);
    }
}

#[test]
fn test_inset_ring_widths() {
    // Test inset ring widths
    let test_cases = vec![
        ("inset-ring", "1px"), // v4 default
        ("inset-ring-0", "0px"),
        ("inset-ring-1", "1px"),
        ("inset-ring-2", "2px"),
        ("inset-ring-4", "4px"),
    ];
    
    for (class, expected_width) in test_cases {
        let mut tw = TailwindBuilder::default();
        let _ = tw.trace(class, false).unwrap();
        let css = tw.bundle().unwrap();
        assert!(css.contains("--tw-inset-ring-shadow"), "Class {} should set inset ring shadow", class);
        assert!(css.contains("inset"), "Class {} should contain 'inset' keyword", class);
        assert!(css.contains(expected_width), "Class {} should contain {}, got: {}", class, expected_width, css);
    }
}

#[test]
fn test_inset_ring_colors() {
    let test_cases = vec![
        "inset-ring-red-500",
        "inset-ring-blue-600",
        "inset-ring-purple-300",
    ];
    
    for class in test_cases {
        let mut tw = TailwindBuilder::default();
        let _ = tw.trace(class, false).unwrap();
        let css = tw.bundle().unwrap();
        assert!(css.contains("--tw-inset-ring-color"), "Class {} should set inset ring color", class);
        assert!(css.contains("inset"), "Class {} should contain 'inset' keyword", class);
    }
}

#[test]
fn test_ring_arbitrary_values() {
    // Test arbitrary ring width
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("ring-[3px]", false).unwrap();
    let css = tw.bundle().unwrap();
    assert!(css.contains("3px"), "Arbitrary ring width should work");
    
    // Test arbitrary ring color
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("ring-[#0088cc]", false).unwrap();
    let css = tw.bundle().unwrap();
    assert!(css.contains("#0088cc") || css.contains("0088cc"), "Arbitrary ring color should work");
}

#[test]
fn test_ring_box_shadow_layering() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("ring-2", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Check that box-shadow includes all the necessary shadow layers
    assert!(css.contains("var(--tw-inset-shadow"), "Should include inset shadow layer");
    assert!(css.contains("var(--tw-inset-ring-shadow"), "Should include inset ring shadow layer");
    assert!(css.contains("var(--tw-ring-offset-shadow"), "Should include ring offset shadow layer");
    assert!(css.contains("var(--tw-ring-shadow"), "Should include ring shadow layer");
    assert!(css.contains("var(--tw-shadow"), "Should include regular shadow layer");
}