use tailwind_css::{TailwindBuilder, TailwindInstance, TailwindSelect};

#[test]
fn test_select_none_direct() {
    let css = TailwindSelect::from("none")
        .attributes(&TailwindBuilder::default())
        .to_string();
    assert_eq!(css, "user-select:none;");
}

#[test]
fn test_select_text_direct() {
    let css = TailwindSelect::from("text")
        .attributes(&TailwindBuilder::default())
        .to_string();
    assert_eq!(css, "user-select:text;");
}

#[test]
fn test_select_all_direct() {
    let css = TailwindSelect::from("all")
        .attributes(&TailwindBuilder::default())
        .to_string();
    assert_eq!(css, "user-select:all;");
}

#[test]
fn test_select_auto_direct() {
    let css = TailwindSelect::from("auto")
        .attributes(&TailwindBuilder::default())
        .to_string();
    assert_eq!(css, "user-select:auto;");
}

#[test]
fn test_select_parsing_with_builder() {
    let mut builder = TailwindBuilder::default();
    
    // Test parsing the select-none class
    let result = builder.inline("select-none").expect("Failed to parse select-none");
    assert!(result.1.contains("user-select:none"));
    
    // Test parsing the select-text class
    builder = TailwindBuilder::default();
    let result = builder.inline("select-text").expect("Failed to parse select-text");
    assert!(result.1.contains("user-select:text"));
    
    // Test parsing the select-all class
    builder = TailwindBuilder::default();
    let result = builder.inline("select-all").expect("Failed to parse select-all");
    assert!(result.1.contains("user-select:all"));
    
    // Test parsing the select-auto class
    builder = TailwindBuilder::default();
    let result = builder.inline("select-auto").expect("Failed to parse select-auto");
    assert!(result.1.contains("user-select:auto"));
}

#[test]
fn test_select_with_modifiers() {
    let mut builder = TailwindBuilder::default();
    
    // Test with hover modifier
    let result = builder.inline("hover:select-none").expect("Failed to parse hover:select-none");
    // When using inline(), modifiers are processed but the CSS output contains only the properties
    // The modifier information is typically handled by the rendering context
    assert!(result.1.contains("user-select:none"));
    
    // Test with focus modifier
    builder = TailwindBuilder::default();
    let result = builder.inline("focus:select-all").expect("Failed to parse focus:select-all");
    assert!(result.1.contains("user-select:all"));
    
    // Test with disabled modifier  
    builder = TailwindBuilder::default();
    let result = builder.inline("disabled:select-auto").expect("Failed to parse disabled:select-auto");
    assert!(result.1.contains("user-select:auto"));
}

#[test]
fn test_select_arbitrary_values() {
    let mut builder = TailwindBuilder::default();
    
    // Test arbitrary value support for contain
    let result = builder.inline("select-[contain]").expect("Failed to parse select-[contain]");
    assert!(result.1.contains("user-select:contain"));
    
    // Test arbitrary value support for inherit
    builder = TailwindBuilder::default();
    let result = builder.inline("select-[inherit]").expect("Failed to parse select-[inherit]");
    assert!(result.1.contains("user-select:inherit"));
}

#[test]
fn test_all_tailwind_select_values() {
    let select_values = vec![
        ("select-none", "user-select:none"),
        ("select-text", "user-select:text"),
        ("select-all", "user-select:all"),
        ("select-auto", "user-select:auto"),
    ];
    
    for (class, expected_css) in select_values {
        let mut builder = TailwindBuilder::default();
        let result = builder.inline(class)
            .unwrap_or_else(|_| panic!("Failed to parse {}", class));
        assert!(
            result.1.contains(expected_css),
            "Class '{}' should generate '{}', but got: {}",
            class,
            expected_css,
            result.1
        );
    }
}