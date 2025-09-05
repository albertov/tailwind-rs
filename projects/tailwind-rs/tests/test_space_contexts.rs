use tailwind_rs::TailwindBuilder;

#[test]
fn test_space_in_different_contexts() {
    let mut tw = TailwindBuilder::default();
    
    println!("\n=== TEST 1: trace() ===");
    let trace_result = tw.trace("space-x-4", false).unwrap();
    println!("trace() returns: '{}'", trace_result);
    
    println!("\n=== TEST 2: inline() ===");
    match tw.inline("space-x-4") {
        Ok((classes, styles)) => {
            println!("inline() returns classes: '{}'", classes);
            println!("inline() returns styles: '{}'", styles);
        },
        Err(e) => println!("inline() error: {}", e),
    }
    
    println!("\n=== TEST 3: bundle() after trace ===");
    let bundle = tw.bundle().unwrap();
    if bundle.contains(".space-x-4 > * + *") {
        println!("✅ Bundle contains space CSS");
    } else {
        println!("❌ Bundle missing space CSS");
    }
}
