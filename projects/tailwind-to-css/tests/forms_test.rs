use tailwind_css::TailwindBuilder;

#[test]
fn test_form_input() {
    let mut builder = TailwindBuilder::default();
    builder.trace("form-input", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(css.contains("appearance:none"));
    assert!(css.contains("background-color:#ffffff"));
    assert!(css.contains("border-color:#d1d5db"));
    assert!(css.contains("border-width:1px"));
    assert!(css.contains("border-radius:0.375rem"));
    assert!(css.contains("padding-top:0.5rem"));
    assert!(css.contains("padding-right:0.75rem"));
    assert!(css.contains("padding-bottom:0.5rem"));
    assert!(css.contains("padding-left:0.75rem"));
    assert!(css.contains("font-size:1rem"));
    assert!(css.contains("line-height:1.5rem"));
}

#[test]
fn test_form_textarea() {
    let mut builder = TailwindBuilder::default();
    builder.trace("form-textarea", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(css.contains("appearance:none"));
    assert!(css.contains("resize:vertical"));
}

#[test]
fn test_form_select() {
    let mut builder = TailwindBuilder::default();
    builder.trace("form-select", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(css.contains("appearance:none"));
    assert!(css.contains("padding-right:2.5rem"));
    assert!(css.contains("background-image:url"));
    assert!(css.contains("background-position:right 0.5rem center"));
}

#[test]
fn test_form_checkbox() {
    let mut builder = TailwindBuilder::default();
    builder.trace("form-checkbox", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(css.contains("appearance:none"));
    assert!(css.contains("height:1rem"));
    assert!(css.contains("width:1rem"));
    assert!(css.contains("border-radius:0.25rem"));
}

#[test]
fn test_form_radio() {
    let mut builder = TailwindBuilder::default();
    builder.trace("form-radio", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(css.contains("appearance:none"));
    assert!(css.contains("height:1rem"));
    assert!(css.contains("width:1rem"));
    assert!(css.contains("border-radius:100%"));
}

#[test]
fn test_all_form_utilities() {
    let utilities = vec![
        "form-input",
        "form-textarea",
        "form-select",
        "form-checkbox",
        "form-radio",
    ];
    
    for utility in utilities {
        let mut builder = TailwindBuilder::default();
        builder.trace(utility, false).unwrap_or_else(|_| panic!("Failed to trace {}", utility));
        let css = builder.bundle().unwrap();
        
        assert!(!css.is_empty(), "{} should generate CSS", utility);
        assert!(css.contains(&format!(".{}", utility)), "{} should include the class name", utility);
    }
}