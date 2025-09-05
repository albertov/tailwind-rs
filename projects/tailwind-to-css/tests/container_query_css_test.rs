use tailwind_css::TailwindBuilder;

/// Test basic container query CSS generation
#[test]
fn test_container_query_min_width() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@lg:flex", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Debug output
    println!("Generated CSS: {}", css);
    
    // Should generate @container rule with min-width
    assert!(css.contains("@container (min-width: 48rem)"), "CSS doesn't contain expected container query. Got: {}", css);
    assert!(css.contains("@lg\\:flex") || css.contains("\\@lg\\:flex"), "CSS doesn't contain expected class selector. Got: {}", css);
    assert!(css.contains("display: flex") || css.contains("display:flex"), "CSS doesn't contain expected display property. Got: {}", css);
}

/// Test max-width container query
#[test]
fn test_container_query_max_width() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@max-lg:hidden", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should generate @container rule with max-width
    assert!(css.contains("@container (max-width: 48rem)"));
    assert!(css.contains("@max-lg\\:hidden") || css.contains("\\@max-lg\\:hidden"));
    assert!(css.contains("display: none") || css.contains("display:none"));
}

/// Test arbitrary value container query
#[test]
fn test_container_query_arbitrary() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@[300px]:grid", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Debug output
    println!("Generated CSS for @[300px]:grid:");
    for line in css.lines() {
        if line.contains("300px") || line.contains("grid") || line.contains("@container") {
            println!("  {}", line);
        }
    }
    
    // Should generate @container rule with arbitrary value
    assert!(css.contains("@container (min-width: 300px)"));
    // Note: Arbitrary values in container queries have brackets stripped in class names
    assert!(css.contains("@300px\\:grid") || css.contains("\\@300px\\:grid") || css.contains("\\@\\[300px\\]\\:grid"));
    assert!(css.contains("display: grid") || css.contains("display:grid"));
}

/// Test named container query
#[test]
fn test_container_query_named() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@lg/sidebar:flex", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should generate @container rule with container name
    assert!(css.contains("@container sidebar (min-width: 48rem)"));
    assert!(css.contains("@lg\\/sidebar\\:flex") || css.contains("\\@lg\\/sidebar\\:flex"));
    assert!(css.contains("display: flex") || css.contains("display:flex"));
}

/// Test container query with different breakpoints
#[test]
fn test_container_query_breakpoints() {
    let mut builder = TailwindBuilder::default();
    
    // Test sm breakpoint
    builder.trace("@sm:block", false).unwrap();
    let css_sm = builder.bundle().unwrap();
    assert!(css_sm.contains("@container (min-width: 16rem)"));
    
    // Clear and test md breakpoint
    builder = TailwindBuilder::default();
    builder.trace("@md:inline-block", false).unwrap();
    let css_md = builder.bundle().unwrap();
    assert!(css_md.contains("@container (min-width: 28rem)"));
    
    // Clear and test xl breakpoint
    builder = TailwindBuilder::default();
    builder.trace("@xl:inline", false).unwrap();
    let css_xl = builder.bundle().unwrap();
    assert!(css_xl.contains("@container (min-width: 64rem)"));
    
    // Clear and test 2xl breakpoint
    builder = TailwindBuilder::default();
    builder.trace("@2xl:flex", false).unwrap();
    let css_2xl = builder.bundle().unwrap();
    assert!(css_2xl.contains("@container (min-width: 80rem)"));
}

/// Test container query with multiple classes
#[test]
fn test_container_query_multiple_classes() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@lg:flex @lg:p-4 @lg:bg-blue-500", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should generate multiple container query rules
    assert!(css.contains("@container (min-width: 48rem)"));
    assert!(css.contains("display: flex") || css.contains("display:flex"));
    assert!(css.contains("padding: 1rem") || css.contains("padding:1rem"));
}

/// Test mixing container queries with media queries
#[test]
fn test_container_query_with_media_query() {
    let mut builder = TailwindBuilder::default();
    builder.trace("sm:@lg:flex", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should nest container query inside media query
    assert!(css.contains("@media (min-width: 640px)"));
    assert!(css.contains("@container (min-width: 48rem)"));
    assert!(css.contains("display: flex") || css.contains("display:flex"));
}

/// Test container query with pseudo-class
#[test]
fn test_container_query_with_pseudo_class() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@lg:hover:bg-red-500", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should combine container query with hover pseudo-class
    assert!(css.contains("@container (min-width: 48rem)"));
    assert!(css.contains(":hover"));
}

/// Test container query with arbitrary rem value
#[test]
fn test_container_query_arbitrary_rem() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@[20rem]:flex", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should handle rem values in arbitrary container queries
    assert!(css.contains("@container (min-width: 20rem)"));
    assert!(css.contains("display: flex") || css.contains("display:flex"));
}

/// Test max container query with named container
#[test]
fn test_container_query_max_named() {
    let mut builder = TailwindBuilder::default();
    builder.trace("@max-md/content:hidden", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Should generate max-width query with container name
    assert!(css.contains("@container content (max-width: 28rem)"));
    assert!(css.contains("display: none") || css.contains("display:none"));
}