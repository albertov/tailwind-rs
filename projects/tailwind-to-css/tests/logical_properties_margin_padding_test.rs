use tailwind_css::TailwindBuilder;

fn test_class(class: &str, expected_property: &str, expected_value: &str) {
    let mut builder = TailwindBuilder::default();
    let result = builder.inline(class);
    
    match result {
        Ok((_, css)) => {
            assert!(css.contains(expected_property), 
                    "Class '{}' should generate '{}', got: {}", class, expected_property, css);
            assert!(css.contains(expected_value), 
                    "Class '{}' should use value '{}', got: {}", class, expected_value, css);
        }
        Err(e) => {
            panic!("Failed to process '{}': {:?}", class, e);
        }
    }
}

#[test]
fn test_margin_inline_start() {
    // Test standard values
    test_class("ms-0", "margin-inline-start:", "0rem");
    test_class("ms-0.5", "margin-inline-start:", "0.125rem");
    test_class("ms-1", "margin-inline-start:", "0.25rem");
    test_class("ms-2", "margin-inline-start:", "0.5rem");
    test_class("ms-4", "margin-inline-start:", "1rem");
    test_class("ms-8", "margin-inline-start:", "2rem");
    test_class("ms-16", "margin-inline-start:", "4rem");
    test_class("ms-32", "margin-inline-start:", "8rem");
    test_class("ms-48", "margin-inline-start:", "12rem");
    test_class("ms-64", "margin-inline-start:", "16rem");
    test_class("ms-96", "margin-inline-start:", "24rem");
    
    // Test px value
    test_class("ms-px", "margin-inline-start:", "1px");
    
    // Test auto
    test_class("ms-auto", "margin-inline-start:", "auto");
}

#[test]
fn test_margin_inline_start_negative() {
    // Test negative values
    test_class("-ms-1", "margin-inline-start:", "-0.25rem");
    test_class("-ms-4", "margin-inline-start:", "-1rem");
    test_class("-ms-8", "margin-inline-start:", "-2rem");
}

#[test]
fn test_margin_inline_start_arbitrary() {
    // Test arbitrary values
    test_class("ms-[10px]", "margin-inline-start:", "10px");
    test_class("ms-[2.5rem]", "margin-inline-start:", "2.5rem");
    test_class("-ms-[10px]", "margin-inline-start:", "-10px");
}

#[test]
fn test_margin_inline_end() {
    // Test standard values
    test_class("me-0", "margin-inline-end:", "0rem");
    test_class("me-0.5", "margin-inline-end:", "0.125rem");
    test_class("me-1", "margin-inline-end:", "0.25rem");
    test_class("me-2", "margin-inline-end:", "0.5rem");
    test_class("me-4", "margin-inline-end:", "1rem");
    test_class("me-8", "margin-inline-end:", "2rem");
    test_class("me-16", "margin-inline-end:", "4rem");
    test_class("me-32", "margin-inline-end:", "8rem");
    test_class("me-48", "margin-inline-end:", "12rem");
    test_class("me-64", "margin-inline-end:", "16rem");
    test_class("me-96", "margin-inline-end:", "24rem");
    
    // Test px value
    test_class("me-px", "margin-inline-end:", "1px");
    
    // Test auto
    test_class("me-auto", "margin-inline-end:", "auto");
}

#[test]
fn test_margin_inline_end_negative() {
    // Test negative values
    test_class("-me-1", "margin-inline-end:", "-0.25rem");
    test_class("-me-4", "margin-inline-end:", "-1rem");
    test_class("-me-8", "margin-inline-end:", "-2rem");
}

#[test]
fn test_margin_inline_end_arbitrary() {
    // Test arbitrary values
    test_class("me-[10px]", "margin-inline-end:", "10px");
    test_class("me-[2.5rem]", "margin-inline-end:", "2.5rem");
    test_class("-me-[10px]", "margin-inline-end:", "-10px");
}

#[test]
fn test_padding_inline_start() {
    // Test standard values
    test_class("ps-0", "padding-inline-start:", "0rem");
    test_class("ps-0.5", "padding-inline-start:", "0.125rem");
    test_class("ps-1", "padding-inline-start:", "0.25rem");
    test_class("ps-2", "padding-inline-start:", "0.5rem");
    test_class("ps-4", "padding-inline-start:", "1rem");
    test_class("ps-8", "padding-inline-start:", "2rem");
    test_class("ps-16", "padding-inline-start:", "4rem");
    test_class("ps-32", "padding-inline-start:", "8rem");
    test_class("ps-48", "padding-inline-start:", "12rem");
    test_class("ps-64", "padding-inline-start:", "16rem");
    test_class("ps-96", "padding-inline-start:", "24rem");
    
    // Test px value
    test_class("ps-px", "padding-inline-start:", "1px");
}

#[test]
fn test_padding_inline_start_arbitrary() {
    // Test arbitrary values
    test_class("ps-[10px]", "padding-inline-start:", "10px");
    test_class("ps-[2.5rem]", "padding-inline-start:", "2.5rem");
}

#[test]
fn test_padding_inline_end() {
    // Test standard values
    test_class("pe-0", "padding-inline-end:", "0rem");
    test_class("pe-0.5", "padding-inline-end:", "0.125rem");
    test_class("pe-1", "padding-inline-end:", "0.25rem");
    test_class("pe-2", "padding-inline-end:", "0.5rem");
    test_class("pe-4", "padding-inline-end:", "1rem");
    test_class("pe-8", "padding-inline-end:", "2rem");
    test_class("pe-16", "padding-inline-end:", "4rem");
    test_class("pe-32", "padding-inline-end:", "8rem");
    test_class("pe-48", "padding-inline-end:", "12rem");
    test_class("pe-64", "padding-inline-end:", "16rem");
    test_class("pe-96", "padding-inline-end:", "24rem");
    
    // Test px value
    test_class("pe-px", "padding-inline-end:", "1px");
}

#[test]
fn test_padding_inline_end_arbitrary() {
    // Test arbitrary values
    test_class("pe-[10px]", "padding-inline-end:", "10px");
    test_class("pe-[2.5rem]", "padding-inline-end:", "2.5rem");
}

#[test]
fn test_logical_properties_with_modifiers() {
    let mut builder = TailwindBuilder::default();
    
    // Test with hover modifier
    let result = builder.trace("hover:ms-4", false);
    assert!(result.is_ok(), "hover:ms-4 should be recognized");
    let css = builder.bundle().unwrap();
    assert!(css.contains("margin-inline-start"), "hover:ms-4 should generate margin-inline-start");
    assert!(css.contains(":hover"), "hover:ms-4 should have hover pseudo-class");
    
    // Test with responsive modifiers
    let mut builder2 = TailwindBuilder::default();
    let result = builder2.trace("sm:ps-8", false);
    assert!(result.is_ok(), "sm:ps-8 should be recognized");
    let css = builder2.bundle().unwrap();
    assert!(css.contains("padding-inline-start"), "sm:ps-8 should generate padding-inline-start");
    assert!(css.contains("@media"), "sm:ps-8 should have media query");
}

#[test]
fn test_logical_properties_edge_cases() {
    // Test with fractional values
    test_class("ms-1.5", "margin-inline-start:", "0.375rem");
    test_class("me-2.5", "margin-inline-end:", "0.625rem");
    test_class("ps-3.5", "padding-inline-start:", "0.875rem");
    test_class("pe-5.5", "padding-inline-end:", "1.375rem");
    
    // Test larger values
    test_class("ms-80", "margin-inline-start:", "20rem");
    test_class("me-80", "margin-inline-end:", "20rem");
    test_class("ps-80", "padding-inline-start:", "20rem");
    test_class("pe-80", "padding-inline-end:", "20rem");
}

#[test]
fn test_logical_properties_important() {
    let mut builder = TailwindBuilder::default();
    
    // Test with important modifier
    let result = builder.inline("!ms-4");
    match result {
        Ok((_, css)) => {
            assert!(css.contains("margin-inline-start"), "!ms-4 should generate margin-inline-start");
            assert!(css.contains("!important"), "!ms-4 should have !important flag");
        }
        Err(e) => {
            panic!("Failed to process !ms-4: {:?}", e);
        }
    }
}

#[test]
fn test_logical_properties_negative_padding_not_allowed() {
    let mut builder = TailwindBuilder::default();
    
    // Negative padding should not be allowed
    // The inline method is permissive and won't error, but it should return empty CSS
    let result1 = builder.inline("-ps-4");
    match result1 {
        Ok((class, css)) => {
            assert_eq!(css, "", "-ps-4 should not generate CSS (negative padding not allowed)");
            assert_eq!(class, "-ps-4", "Class should be preserved but no CSS generated");
        },
        Err(_) => {
            // Also acceptable if it errors
        }
    }
    
    let result2 = builder.inline("-pe-4");
    match result2 {
        Ok((class, css)) => {
            assert_eq!(css, "", "-pe-4 should not generate CSS (negative padding not allowed)");
            assert_eq!(class, "-pe-4", "Class should be preserved but no CSS generated");
        },
        Err(_) => {
            // Also acceptable if it errors  
        }
    }
}