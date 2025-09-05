use tailwind_rs::TailwindBuilder;

#[test]
fn test_space_x_generates_css() {
    let mut tw = TailwindBuilder::default();
    
    // Test space-x-4
    let result = tw.trace("space-x-4", false).unwrap();
    assert_eq!(result, "space-x-4", "trace should return class name");
    
    let bundle = tw.bundle().unwrap();
    
    // Check if CSS is generated
    assert!(bundle.contains("space-x-4"), "Bundle should contain the class name");
    assert!(bundle.contains("margin-left"), "Bundle should contain margin-left property");
    assert!(bundle.contains("> * + *"), "Bundle should contain child combinator selector");
    
    println!("Generated CSS for space-x-4:\n{}", bundle);
}

#[test]
fn test_space_y_generates_css() {
    let mut tw = TailwindBuilder::default();
    
    // Test space-y-2
    let result = tw.trace("space-y-2", false).unwrap();
    assert_eq!(result, "space-y-2", "trace should return class name");
    
    let bundle = tw.bundle().unwrap();
    
    // Check if CSS is generated
    assert!(bundle.contains("space-y-2"), "Bundle should contain the class name");
    assert!(bundle.contains("margin-top"), "Bundle should contain margin-top property");
    assert!(bundle.contains("> * + *"), "Bundle should contain child combinator selector");
    
    println!("Generated CSS for space-y-2:\n{}", bundle);
}
