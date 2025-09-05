use tailwind_css::TailwindBuilder;

/// Test comprehensive container query functionality
/// This test file covers all aspects of container queries in Tailwind CSS v3/v4

#[test]
fn test_basic_container_queries_all_breakpoints() {
    // Test all standard breakpoints
    let breakpoints = vec![
        ("@sm", "16rem"),
        ("@md", "28rem"),
        ("@lg", "48rem"),
        ("@xl", "64rem"),
        ("@2xl", "80rem"),
    ];
    
    for (bp, expected_width) in breakpoints {
        let mut builder = TailwindBuilder::default();
        builder.trace(&format!("{}:flex", bp), false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains(&format!("@container (min-width: {})", expected_width)),
            "Missing container query for breakpoint {}: expected min-width: {}",
            bp, expected_width
        );
        assert!(
            css.contains("display:flex") || css.contains("display: flex"),
            "Missing display:flex for breakpoint {}",
            bp
        );
    }
}

#[test]
fn test_max_width_container_queries() {
    // Test max-width queries for all breakpoints
    let breakpoints = vec![
        ("@max-sm", "16rem"),
        ("@max-md", "28rem"),
        ("@max-lg", "48rem"),
        ("@max-xl", "64rem"),
        ("@max-2xl", "80rem"),
    ];
    
    for (bp, expected_width) in breakpoints {
        let mut builder = TailwindBuilder::default();
        builder.trace(&format!("{}:hidden", bp), false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains(&format!("@container (max-width: {})", expected_width)),
            "Missing max-width container query for breakpoint {}: expected max-width: {}",
            bp, expected_width
        );
        assert!(
            css.contains("display:none") || css.contains("display: none"),
            "Missing display:none for breakpoint {}",
            bp
        );
    }
}

#[test]
fn test_arbitrary_container_queries_various_units() {
    // Test various arbitrary value formats
    let test_cases = vec![
        ("@[100px]:block", "100px"),
        ("@[10rem]:inline", "10rem"),
        ("@[50vw]:flex", "50vw"),
        ("@[800px]:grid", "800px"),
        ("@[1200px]:hidden", "1200px"),
    ];
    
    for (class, expected_width) in test_cases {
        let mut builder = TailwindBuilder::default();
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains(&format!("@container (min-width: {})", expected_width)),
            "Missing arbitrary container query for {}: expected min-width: {}",
            class, expected_width
        );
    }
}

#[test]
fn test_named_container_queries() {
    // Test named containers with various breakpoints
    let test_cases = vec![
        ("@lg/sidebar:flex", "sidebar", "48rem"),
        ("@md/header:block", "header", "28rem"),
        ("@xl/main:grid", "main", "64rem"),
        ("@sm/footer:hidden", "footer", "16rem"),
    ];
    
    for (class, container_name, expected_width) in test_cases {
        let mut builder = TailwindBuilder::default();
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains(&format!("@container {} (min-width: {})", container_name, expected_width)),
            "Missing named container query for {}: expected @container {} (min-width: {})",
            class, container_name, expected_width
        );
    }
}

#[test]
fn test_container_queries_with_various_utilities() {
    // Test container queries with different utility classes
    let test_cases = vec![
        // Layout utilities
        ("@lg:flex", "display:flex"),
        ("@lg:block", "display:block"),
        ("@lg:inline", "display:inline"),
        ("@lg:grid", "display:grid"),
        ("@lg:hidden", "display:none"),
        
        // Spacing utilities
        ("@lg:p-4", "padding:1rem"),
        ("@lg:m-2", "margin:0.5rem"),
        ("@lg:px-6", "padding-left:1.5rem"),
        
        // Sizing utilities
        ("@lg:w-full", "width:100%"),
        ("@lg:h-screen", "height:100vh"),
        
        // Typography utilities
        ("@lg:text-lg", "font-size:1.125rem"),
        ("@lg:font-bold", "font-weight:700"),
        
        // Background utilities
        ("@lg:bg-blue-500", "background-color:"),
        
        // Border utilities
        ("@lg:border-2", "border-width:2px"),
        ("@lg:rounded-lg", "border-radius:0.5rem"),
    ];
    
    for (class, expected_css) in test_cases {
        let mut builder = TailwindBuilder::default();
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains("@container (min-width: 48rem)"),
            "Missing container query for {}",
            class
        );
        assert!(
            css.contains(expected_css),
            "Missing expected CSS '{}' for class {}",
            expected_css, class
        );
    }
}

#[test]
fn test_container_queries_with_pseudo_classes() {
    // Test combining container queries with pseudo-classes
    let test_cases = vec![
        ("@lg:hover:bg-blue-500", ":hover"),
        ("@lg:focus:outline-none", ":focus"),
        ("@lg:active:scale-95", ":active"),
        ("@lg:disabled:opacity-50", ":disabled"),
        ("@lg:first:mt-0", ":first-child"),
        ("@lg:last:mb-0", ":last-child"),
    ];
    
    for (class, expected_pseudo) in test_cases {
        let mut builder = TailwindBuilder::default();
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains("@container (min-width: 48rem)"),
            "Missing container query for {}",
            class
        );
        assert!(
            css.contains(expected_pseudo),
            "Missing pseudo-class '{}' for class {}",
            expected_pseudo, class
        );
    }
}

#[test]
fn test_container_queries_with_media_queries() {
    // Test combining media queries and container queries
    let test_cases = vec![
        ("sm:@lg:flex", "@media (min-width: 640px)", "@container (min-width: 48rem)"),
        ("md:@xl:grid", "@media (min-width: 768px)", "@container (min-width: 64rem)"),
        ("lg:@sm:block", "@media (min-width: 1024px)", "@container (min-width: 16rem)"),
        ("dark:@lg:bg-gray-800", "@media (prefers-color-scheme: dark)", "@container (min-width: 48rem)"),
    ];
    
    for (class, expected_media, expected_container) in test_cases {
        let mut builder = TailwindBuilder::default();
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        assert!(
            css.contains(expected_media),
            "Missing media query '{}' for class {}",
            expected_media, class
        );
        assert!(
            css.contains(expected_container),
            "Missing container query '{}' for class {}",
            expected_container, class
        );
    }
}

#[test]
fn test_container_queries_with_group_and_peer() {
    let mut builder = TailwindBuilder::default();
    
    // Test combining container queries with group and peer selectors
    builder.trace("@lg:group-hover:text-blue-500", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(
        css.contains("@container (min-width: 48rem)"),
        "Missing container query for group-hover variant"
    );
    assert!(
        css.contains("group") && css.contains(":hover"),
        "Missing group-hover selector"
    );
    
    builder = TailwindBuilder::default();
    builder.trace("@lg:peer-checked:opacity-100", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(
        css.contains("@container (min-width: 48rem)"),
        "Missing container query for peer-checked variant"
    );
    assert!(
        css.contains("peer") && css.contains(":checked"),
        "Missing peer-checked selector"
    );
}

#[test]
fn test_multiple_container_queries_in_single_trace() {
    let mut builder = TailwindBuilder::default();
    
    // Test multiple container query classes in a single trace call
    builder.trace("@sm:p-2 @md:p-4 @lg:p-6 @xl:p-8", false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Check that all container queries are present
    assert!(css.contains("@container (min-width: 16rem)"), "Missing @sm container query");
    assert!(css.contains("@container (min-width: 28rem)"), "Missing @md container query");
    assert!(css.contains("@container (min-width: 48rem)"), "Missing @lg container query");
    assert!(css.contains("@container (min-width: 64rem)"), "Missing @xl container query");
    
    // Check that all padding values are present
    assert!(css.contains("padding:0.5rem"), "Missing padding for @sm:p-2");
    assert!(css.contains("padding:1rem"), "Missing padding for @md:p-4");
    assert!(css.contains("padding:1.5rem"), "Missing padding for @lg:p-6");
    assert!(css.contains("padding:2rem"), "Missing padding for @xl:p-8");
}

#[test]
fn test_container_query_with_important() {
    
    let mut builder = TailwindBuilder::default();
    
    // Test container queries with important modifier
    builder.trace("@lg:!flex", false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(
        css.contains("@container (min-width: 48rem)"),
        "Missing container query for important variant"
    );
    assert!(
        css.contains("!important") || css.contains("display:flex!important"),
        "Missing !important modifier"
    );
}

#[test]
fn test_container_query_edge_cases() {
    let mut builder = TailwindBuilder::default();
    
    // Test edge cases
    
    // 1. Very small arbitrary value
    builder.trace("@[1px]:block", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("@container (min-width: 1px)"), "Failed to handle 1px arbitrary value");
    
    // 2. Very large arbitrary value
    builder = TailwindBuilder::default();
    builder.trace("@[9999px]:hidden", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("@container (min-width: 9999px)"), "Failed to handle large arbitrary value");
    
    // 3. Multiple named containers
    builder = TailwindBuilder::default();
    builder.trace("@lg/sidebar:flex @lg/main:grid @lg/header:block", false).unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("@container sidebar"), "Missing sidebar container");
    assert!(css.contains("@container main"), "Missing main container");
    assert!(css.contains("@container header"), "Missing header container");
}

#[test]
fn test_container_setup_utilities() {
    // Note: Container setup utilities like @container, @container-normal, @container-size
    // are not yet fully integrated into the instruction resolver.
    // This test is a placeholder for future implementation.
    
    // Expected behavior:
    // @container → container-type: inline-size
    // @container-normal → container-type: normal
    // @container-size → container-type: size
    // @container/name → container-type: inline-size; container-name: name
    
    // TODO: Implement container setup utilities in instruction resolver
}

#[test]
fn test_container_queries_performance() {
    let mut builder = TailwindBuilder::default();
    
    // Test that processing many container queries doesn't cause issues
    let mut classes = Vec::new();
    for i in 0..50 {
        classes.push(format!("@lg:p-{}", i % 10));
    }
    
    let start = std::time::Instant::now();
    builder.trace(&classes.join(" "), false).unwrap();
    let _css = builder.bundle().unwrap();
    let duration = start.elapsed();
    
    // Ensure processing completes in reasonable time (< 1 second)
    assert!(
        duration.as_secs() < 1,
        "Container query processing took too long: {:?}",
        duration
    );
}