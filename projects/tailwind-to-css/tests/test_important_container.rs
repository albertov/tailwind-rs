use tailwind_css::TailwindBuilder;

#[test]
fn test_important_with_container() {
    let input = "@lg:!flex";
    println!("\n=== Testing: {} ===", input);
    
    // First parse with tailwind_ast to see what's happening
    match tailwind_ast::parse_tailwind(input) {
        Ok(parsed) => {
            for style in parsed {
                println!("Parsed AST: {:?}", style);
                println!("  Important: {}", style.important);
                println!("  Variants: {:?}", style.variants);
                println!("  Elements: {:?}", style.elements);
            }
        }
        Err(e) => println!("Parse error: {:?}", e),
    }
    
    // Now test with TailwindBuilder
    let mut b = TailwindBuilder::default();
    match b.trace(input, false) {
        Ok(classes) => {
            println!("Classes: {}", classes);
            match b.bundle() {
                Ok(css) => {
                    println!("Generated CSS:");
                    for line in css.lines() {
                        if line.contains("@container") || line.contains("flex") || line.contains("important") {
                            println!("  {}", line);
                        }
                    }
                }
                Err(e) => println!("Bundle error: {}", e),
            }
        }
        Err(e) => println!("Trace error: {}", e),
    }
}