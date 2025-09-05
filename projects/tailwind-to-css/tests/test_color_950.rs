use tailwind_css::TailwindBuilder;

#[test]
fn test_color_950_shades() {
    // Test that 950 shades are recognized and generate CSS
    let test_classes = vec![
        "bg-slate-950",
        "bg-gray-950",
        "bg-zinc-950",
        "bg-neutral-950",
        "bg-stone-950",
        "bg-red-950",
        "bg-orange-950",
        "bg-amber-950",
        "bg-yellow-950",
        "bg-lime-950",
        "bg-green-950",
        "bg-emerald-950",
        "bg-teal-950",
        "bg-cyan-950",
        "bg-sky-950",
        "bg-blue-950",
        "bg-indigo-950",
        "bg-violet-950",
        "bg-purple-950",
        "bg-fuchsia-950",
        "bg-pink-950",
        "bg-rose-950",
        "text-slate-950",
        "text-red-950",
        "border-blue-950",
    ];

    println!("\n=== Testing color-950 shade implementation ===\n");
    
    let mut successes = 0;
    let mut failures = Vec::new();
    
    for class in test_classes {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(transformed) => {
                // The trace should return the same class if valid
                println!("✓ {} is recognized (transformed to: {})", class, transformed);
                successes += 1;
                
                // Also check that it generates CSS
                if let Ok(css) = builder.bundle() {
                    if !css.is_empty() {
                        println!("  Generated CSS length: {} bytes", css.len());
                    }
                }
            }
            Err(e) => {
                println!("✗ {} failed: {:?}", class, e);
                failures.push((class, e));
            }
        }
    }
    
    println!("\n=== Summary ===");
    println!("Successes: {}", successes);
    println!("Failures: {}", failures.len());
    
    if !failures.is_empty() {
        println!("\nFailed classes:");
        for (class, err) in &failures {
            println!("  - {}: {:?}", class, err);
        }
        panic!("Some color-950 classes failed to parse!");
    }
}

#[test]
fn test_950_shade_css_generation() {
    // Test that 950 shades generate correct CSS properties
    let test_cases = vec![
        ("bg-slate-950", "background-color"),
        ("text-red-950", "color"),
        ("border-blue-950", "border-color"),
    ];
    
    println!("\n=== Testing CSS generation for 950 shades ===\n");
    
    for (class, expected_property) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                match builder.bundle() {
                    Ok(css) => {
                        if css.contains(expected_property) {
                            println!("✓ {} generates CSS with {}", class, expected_property);
                            println!("  CSS snippet: {}", 
                                css.lines().find(|l| l.contains(expected_property))
                                    .unwrap_or("(property found)"));
                        } else {
                            panic!("{} should generate CSS with {}, but got:\n{}", 
                                class, expected_property, css);
                        }
                    }
                    Err(e) => {
                        panic!("Failed to build CSS for {}: {:?}", class, e);
                    }
                }
            }
            Err(e) => {
                panic!("Failed to parse {}: {:?}", class, e);
            }
        }
    }
}