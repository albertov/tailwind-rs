use tailwind_css::TailwindBuilder;

#[test]
fn test_shadow_2xs() {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace("shadow-2xs", false);
    
    // Get the generated CSS
    let css = builder.bundle().unwrap_or_default();
    
    // Check that shadow-2xs generates expected CSS
    assert!(css.contains(".shadow-2xs") || css.contains("shadow-2xs"), 
            "CSS should contain shadow-2xs class definition, but got: {}", css);
}

#[test]
fn test_shadow_xs() {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace("shadow-xs", false);
    
    // Get the generated CSS
    let css = builder.bundle().unwrap_or_default();
    
    // Check that shadow-xs generates expected CSS
    assert!(css.contains(".shadow-xs") || css.contains("shadow-xs"),
            "CSS should contain shadow-xs class definition, but got: {}", css);
}

#[test]
fn test_drop_shadow_2xs() {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace("drop-shadow-2xs", false);
    
    // Get the generated CSS
    let css = builder.bundle().unwrap_or_default();
    
    // Check that drop-shadow-2xs generates expected CSS
    assert!(css.contains(".drop-shadow-2xs") || css.contains("drop-shadow-2xs"),
            "CSS should contain drop-shadow-2xs class definition, but got: {}", css);
}

#[test]
fn test_drop_shadow_xs() {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace("drop-shadow-xs", false);
    
    // Get the generated CSS
    let css = builder.bundle().unwrap_or_default();
    
    // Check that drop-shadow-xs generates expected CSS
    assert!(css.contains(".drop-shadow-xs") || css.contains("drop-shadow-xs"),
            "CSS should contain drop-shadow-xs class definition, but got: {}", css);
}

#[test]
fn test_shadow_scale_classes_recognized() {
    // Just verify that all shadow scale values are recognized without errors
    let classes = vec![
        "shadow-2xs",
        "shadow-xs",
        "shadow-sm",
        "shadow",
        "shadow-md",
        "shadow-lg",
        "shadow-xl",
        "shadow-2xl",
    ];
    
    for class in &classes {
        let mut builder = TailwindBuilder::default();
        let result = builder.trace(class, false);
        
        // Should not error on valid shadow classes
        assert!(result.is_ok(), 
                "Failed to process valid shadow class '{}': {:?}", 
                class, result);
    }
    
    println!("All shadow scale classes recognized successfully");
}
