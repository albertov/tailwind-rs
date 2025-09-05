use tailwind_css::TailwindBuilder;

#[test]
fn test_container_setup_utilities_generate_correct_css() {
    let mut builder = TailwindBuilder::default();
    
    // Test @container (default inline-size)
    builder.trace("@container", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("container-type:inline-size"), "@container should generate container-type:inline-size");
    
    // Test @container/sidebar (named container)
    let mut builder = TailwindBuilder::default();
    builder.trace("@container/sidebar", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("container-type:inline-size"), "@container/sidebar should generate container-type:inline-size");
    assert!(css.contains("container-name:sidebar"), "@container/sidebar should generate container-name:sidebar");
    
    // Test @container-normal
    let mut builder = TailwindBuilder::default();
    builder.trace("@container-normal", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("container-type:normal"), "@container-normal should generate container-type:normal");
    
    // Test @container-size
    let mut builder = TailwindBuilder::default();
    builder.trace("@container-size", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("container-type:size"), "@container-size should generate container-type:size");
    
    // Test @container-normal/main (named normal container)
    let mut builder = TailwindBuilder::default();
    builder.trace("@container-normal/main", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("container-type:normal"), "@container-normal/main should generate container-type:normal");
    assert!(css.contains("container-name:main"), "@container-normal/main should generate container-name:main");
}

#[test]
fn test_multiple_container_setup_utilities() {
    let mut builder = TailwindBuilder::default();
    
    let classes = "@container @container/sidebar @container-normal @container-size";
    builder.trace(classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify all container types are present
    assert!(css.contains("container-type:inline-size"));
    assert!(css.contains("container-type:normal"));
    assert!(css.contains("container-type:size"));
    
    // Verify named container
    assert!(css.contains("container-name:sidebar"));
    
    println!("All container setup utilities generated correct CSS!");
}