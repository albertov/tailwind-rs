use tailwind_css::TailwindBuilder;

#[test]
fn check_css_formatting() {
    let mut builder = TailwindBuilder::default();
    
    // Test a padding utility
    builder.trace("p-4", false).unwrap();
    let css = builder.bundle().unwrap_or_default();
    
    if let Some(line) = css.lines().find(|l| l.contains(".p-4")) {
        eprintln!("Padding CSS: {}", line);
    }
    
    // Test a width utility
    let mut builder2 = TailwindBuilder::default();
    builder2.trace("w-4", false).unwrap();
    let css2 = builder2.bundle().unwrap_or_default();
    
    if let Some(line) = css2.lines().find(|l| l.contains(".w-4")) {
        eprintln!("Width CSS: {}", line);
    }
}
