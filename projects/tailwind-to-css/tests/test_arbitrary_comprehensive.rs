use tailwind_css::TailwindBuilder;

#[test]
fn test_arbitrary_values_comprehensive() {
    let mut builder = TailwindBuilder::default();
    
    // Test cases from QA report
    let test_cases = vec![
        // Order utility - note that numeric arbitrary values get normalized
        ("order-[4]", "order:4"),  // Gets normalized to order-4
        ("order-[10]", "order:10"),  // Gets normalized to order-10
        
        // Width/height with arbitrary values
        ("w-[32px]", "width:32px"),
        ("h-[100vh]", "height:100vh"),
        ("w-[calc(100%_+_1rem)]", "width:calc(100% + 1rem)"),
        ("h-[calc(50%_-_20px)]", "height:calc(50% - 20px)"),
        
        // Border utilities
        ("border-[20cm]", "border-width:20cm"),
        ("border-[2px]", "border-width:2px"),
        ("border-t-[3px]", "border-top-width:3px"),
        
        // Background colors
        ("bg-[#ff0000]", "background-color:rgb(255 0 0)"),  // Hex colors are converted to RGB
        // TODO: Fix RGB parsing with underscores - currently doesn't generate CSS
        // ("bg-[rgb(255,_0,_0)]", "background-color:rgb(255, 0, 0)"),
        
        // Complex calc expressions
        ("w-[calc(100%_/_3)]", "width:calc(100% / 3)"),
        ("h-[clamp(10px,_2vw,_100px)]", "height:clamp(10px, 2vw, 100px)"),
        ("w-[min(100%,_500px)]", "width:min(100%, 500px)"),
        ("h-[max(50vh,_300px)]", "height:max(50vh, 300px)"),
        
        // Variables
        ("w-[var(--custom-width)]", "width:var(--custom-width)"),
    ];
    
    for (input, expected_css) in test_cases {
        builder.trace(input, false).unwrap_or_else(|e| panic!("Failed to parse '{}': {:?}", input, e));
        let css = builder.bundle().unwrap_or_default();
        
        // The result includes the full CSS rule, check if it contains the expected property
        assert!(
            css.contains(expected_css), 
            "Input '{}' should generate CSS containing '{}', but got: '{}'", 
            input, expected_css, css
        );
    }
}

#[test] 
fn test_idempotent_property() {
    let mut builder = TailwindBuilder::default();
    
    let test_cases = vec![
        "order-[4]",
        "w-[32px]",
        "w-[calc(100%_+_1rem)]",
        "border-[20cm]",
        "bg-[#ff0000]",
    ];
    
    for input in test_cases {
        builder.trace(input, false).unwrap();
        let first_result = builder.bundle().unwrap_or_default();
        
        builder.trace(input, false).unwrap();
        let second_result = builder.bundle().unwrap_or_default();
        
        assert_eq!(
            first_result, second_result,
            "Idempotent property violated for '{}': first='{}', second='{}'",
            input, first_result, second_result
        );
    }
}