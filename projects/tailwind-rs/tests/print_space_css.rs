use tailwind_rs::TailwindBuilder;

#[test]
fn print_space_css() {
    let mut tw = TailwindBuilder::default();
    
    // Test space-x-4
    let _ = tw.trace("space-x-4", false).unwrap();
    let bundle = tw.bundle().unwrap();
    
    println!("=== CSS for space-x-4 ===");
    println!("{}", bundle);
    println!("=== END ===");
}
