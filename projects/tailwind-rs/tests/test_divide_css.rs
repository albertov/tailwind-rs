use tailwind_rs::TailwindBuilder;

#[test]
fn test_divide_css() {
    let mut tw = TailwindBuilder::default();
    
    // Test divide-x-2
    let _ = tw.trace("divide-x-2", false).unwrap();
    let bundle = tw.bundle().unwrap();
    
    // Find the divide CSS
    let lines: Vec<&str> = bundle.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("divide-x-2") {
            println!("Found divide-x-2 at line {}:", i);
            // Print context lines
            for j in (i.saturating_sub(1))..=(i+2).min(lines.len()-1) {
                println!("{}: {}", j, lines[j]);
            }
        }
    }
    
    assert!(bundle.contains("divide-x-2"), "Should have divide-x-2");
    assert!(bundle.contains("> * + *"), "Should have child combinator");
    assert!(bundle.contains("border-left-width"), "Should have border-left-width");
}
