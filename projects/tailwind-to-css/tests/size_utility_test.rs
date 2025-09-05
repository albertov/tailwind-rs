use tailwind_css::TailwindBuilder;

#[test]
fn test_size_numeric_values() {
    // Test spacing scale values
    let test_cases = vec![
        ("size-0", "width:0px", "height:0px"),
        ("size-px", "width:1px", "height:1px"),
        ("size-0.5", "width:0.125rem", "height:0.125rem"),
        ("size-1", "width:0.25rem", "height:0.25rem"),
        ("size-2", "width:0.5rem", "height:0.5rem"),
        ("size-4", "width:1rem", "height:1rem"),
        ("size-8", "width:2rem", "height:2rem"),
        ("size-10", "width:2.5rem", "height:2.5rem"),
        ("size-16", "width:4rem", "height:4rem"),
        ("size-32", "width:8rem", "height:8rem"),
        ("size-64", "width:16rem", "height:16rem"),
        ("size-96", "width:24rem", "height:24rem"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_fractional_values() {
    let test_cases = vec![
        ("size-1/2", "width:50%", "height:50%"),
        ("size-1/3", "width:33.333", "height:33.333"),
        ("size-2/3", "width:66.666", "height:66.666"),
        ("size-1/4", "width:25%", "height:25%"),
        ("size-3/4", "width:75%", "height:75%"),
        ("size-1/5", "width:20%", "height:20%"),
        ("size-2/5", "width:40%", "height:40%"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_special_values() {
    let test_cases = vec![
        ("size-auto", "width:auto", "height:auto"),
        ("size-full", "width:100%", "height:100%"),
        ("size-min", "width:min-content", "height:min-content"),
        ("size-max", "width:max-content", "height:max-content"),
        ("size-fit", "width:fit-content", "height:fit-content"),
        ("size-screen", "width:100vw", "height:100vh"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_arbitrary_values() {
    // Test arbitrary values that are currently supported
    // Note: viewport units (vh, vw) and calc() are not yet supported in the parser
    let test_cases = vec![
        ("size-[100px]", "width:100px", "height:100px"),
        ("size-[10rem]", "width:10rem", "height:10rem"),
        ("size-[50%]", "width:50%", "height:50%"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_viewport_units() {
    let test_cases = vec![
        ("size-[50vh]", "width:50vh", "height:50vh"),
        ("size-[100vw]", "width:100vw", "height:100vw"),
        ("size-[25vmin]", "width:25vmin", "height:25vmin"),
        ("size-[75vmax]", "width:75vmax", "height:75vmax"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_calc_expressions() {
    let test_cases = vec![
        ("size-[calc(100%-2rem)]", "width:calc(100%-2rem)", "height:calc(100%-2rem)"),
        ("size-[var(--custom-size)]", "width:var(--custom-size)", "height:var(--custom-size)"),
        ("size-[clamp(1rem,2vw,3rem)]", "width:clamp(1rem,2vw,3rem)", "height:clamp(1rem,2vw,3rem)"),
        ("size-[min(100%,500px)]", "width:min(100%,500px)", "height:min(100%,500px)"),
        ("size-[max(50%,200px)]", "width:max(50%,200px)", "height:max(50%,200px)"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_preset_values() {
    // Test preset sizes (xs, sm, md, lg, xl, etc.)
    let test_cases = vec![
        ("size-xs", "width:20rem", "height:20rem"),
        ("size-sm", "width:24rem", "height:24rem"),
        ("size-md", "width:28rem", "height:28rem"),
        ("size-lg", "width:32rem", "height:32rem"),
        ("size-xl", "width:36rem", "height:36rem"),
        ("size-2xl", "width:42rem", "height:42rem"),
    ];

    for (class, expected_width, expected_height) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected_width),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_width,
                    css
                );
                assert!(
                    css.contains(expected_height),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected_height,
                    css
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_size_multiple_classes() {
    // Test that multiple size utilities work together
    let mut builder = TailwindBuilder::default();
    
    let classes = vec!["size-4", "size-8", "size-full"];
    
    for class in &classes {
        builder.trace(class, false).expect(&format!("Failed to trace {}", class));
    }
    
    let css = builder.bundle().unwrap_or_default();
    
    // Check that all values are present
    assert!(css.contains("width:1rem"));
    assert!(css.contains("height:1rem"));
    assert!(css.contains("width:2rem"));
    assert!(css.contains("height:2rem"));
    assert!(css.contains("width:100%"));
    assert!(css.contains("height:100%"));
}

#[test]
fn test_size_with_other_utilities() {
    // Test that size works alongside other utilities
    let mut builder = TailwindBuilder::default();
    
    let classes = vec!["size-10", "bg-red-500", "p-4"];
    
    for class in &classes {
        builder.trace(class, false).expect(&format!("Failed to trace {}", class));
    }
    
    let css = builder.bundle().unwrap_or_default();
    
    // Check that size and other utilities are all present
    assert!(css.contains("width:2.5rem"));
    assert!(css.contains("height:2.5rem"));
    assert!(css.contains("background-color"));
    assert!(css.contains("padding"));
}
