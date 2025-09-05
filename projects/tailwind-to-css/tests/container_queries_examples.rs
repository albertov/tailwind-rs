use tailwind_css::TailwindBuilder;

/// Container Queries Example Usage
/// 
/// This file demonstrates real-world usage patterns for Tailwind CSS v3/v4 container queries.
/// Container queries allow you to apply styles based on the size of a container element
/// rather than the viewport, enabling truly component-based responsive design.

#[test]
fn example_responsive_card_component() {
    // A card component that adjusts its layout based on container width
    let mut builder = TailwindBuilder::default();
    
    // Card that switches from vertical to horizontal layout based on container size
    let card_classes = [
        // Base styles
        "bg-white rounded-lg shadow-md overflow-hidden",
        
        // Small container: Vertical layout
        "@container", // Make this element a container
        "flex flex-col",
        
        // Medium container: Horizontal layout with small image
        "@md:flex-row",
        "@md:items-center",
        
        // Large container: Horizontal layout with larger spacing
        "@lg:p-6",
        "@lg:gap-6",
    ].join(" ");
    
    builder.trace(&card_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify container queries are generated
    assert!(css.contains("container-type"));
    assert!(css.contains("@container (min-width: 28rem)")); // @md
    assert!(css.contains("@container (min-width: 48rem)")); // @lg
    
    println!("Responsive Card Component CSS generated successfully");
}

#[test]
fn example_sidebar_navigation() {
    // A sidebar that adjusts based on its container, not the viewport
    let mut builder = TailwindBuilder::default();
    
    // Sidebar navigation that adapts to container width
    let sidebar_classes = [
        // Container setup
        "@container/sidebar",
        
        // Narrow container: Icon-only navigation
        "w-16",
        "@sm/sidebar:w-64", // Expand when container is wide enough
        
        // Navigation items
        "flex flex-col gap-2 p-2",
        "@sm/sidebar:p-4",
    ].join(" ");
    
    let nav_item_classes = [
        // Base item styles
        "flex items-center gap-3 p-2 rounded hover:bg-gray-100",
        
        // Hide text labels in narrow containers
        "@container/sidebar",
        "[&_span]:hidden",
        "@sm/sidebar:[&_span]:block",
        
        // Center icons when text is hidden
        "justify-center",
        "@sm/sidebar:justify-start",
    ].join(" ");
    
    builder.trace(&sidebar_classes, false).unwrap();
    builder.trace(&nav_item_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify named container queries
    assert!(css.contains("container-name"));
    assert!(css.contains("@container sidebar"));
    
    println!("Sidebar Navigation CSS generated successfully");
}

#[test]
fn example_article_typography() {
    // Article content that adjusts typography based on available width
    let mut builder = TailwindBuilder::default();
    
    let article_classes = [
        // Make article a container
        "@container",
        
        // Base typography
        "prose prose-gray",
        
        // Adjust typography scale based on container width
        "text-sm",           // Small text by default
        "@md:text-base",     // Normal text at medium width
        "@lg:text-lg",       // Larger text at large width
        "@xl:text-xl",       // Extra large at xl width
        
        // Adjust line height and spacing
        "leading-relaxed",
        "@lg:leading-loose",
        
        // Adjust margins
        "space-y-2",
        "@md:space-y-3",
        "@lg:space-y-4",
    ].join(" ");
    
    builder.trace(&article_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify responsive typography
    assert!(css.contains("@container (min-width: 28rem)")); // @md
    assert!(css.contains("@container (min-width: 48rem)")); // @lg
    assert!(css.contains("@container (min-width: 64rem)")); // @xl
    
    println!("Article Typography CSS generated successfully");
}

#[test]
fn example_product_grid() {
    // Product grid that adjusts columns based on container width
    let mut builder = TailwindBuilder::default();
    
    let grid_classes = [
        // Container setup
        "@container",
        
        // Responsive grid columns
        "grid gap-4",
        "grid-cols-1",           // 1 column on small containers
        "@sm:grid-cols-2",       // 2 columns at sm
        "@md:grid-cols-3",       // 3 columns at md
        "@lg:grid-cols-4",       // 4 columns at lg
        "@xl:grid-cols-5",       // 5 columns at xl
        
        // Adjust gap based on container size
        "@lg:gap-6",
        "@xl:gap-8",
    ].join(" ");
    
    let product_card_classes = [
        // Product card that adjusts based on grid container
        "bg-white rounded-lg shadow-sm p-3",
        "@md:p-4",
        "@lg:p-5",
        
        // Image aspect ratio changes
        "[&_img]:aspect-square",
        "@lg:[&_img]:aspect-[4/3]",
        
        // Text sizing
        "text-sm",
        "@md:text-base",
    ].join(" ");
    
    builder.trace(&grid_classes, false).unwrap();
    builder.trace(&product_card_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify grid responsive behavior
    assert!(css.contains("grid-cols-1"));
    assert!(css.contains("@container (min-width: 16rem)")); // @sm
    assert!(css.contains("grid-cols-4"));
    
    println!("Product Grid CSS generated successfully");
}

#[test]
fn example_dashboard_widget() {
    // Dashboard widget that can be placed in different sized containers
    let mut builder = TailwindBuilder::default();
    
    let widget_classes = [
        // Container setup
        "@container/widget",
        "bg-white rounded-xl shadow p-4",
        
        // Compact mode (default)
        "space-y-2",
        
        // Normal mode (medium containers)
        "@md/widget:space-y-4",
        "@md/widget:p-6",
        
        // Expanded mode (large containers)
        "@lg/widget:space-y-6",
        "@lg/widget:p-8",
    ].join(" ");
    
    let widget_header_classes = [
        // Header adjusts to widget container
        "flex items-center justify-between",
        
        // Title size
        "text-lg font-semibold",
        "@md/widget:text-xl",
        "@lg/widget:text-2xl",
        
        // Show/hide elements based on space
        "[&_.subtitle]:hidden",
        "@md/widget:[&_.subtitle]:block",
        
        "[&_.actions]:hidden",
        "@lg/widget:[&_.actions]:flex",
    ].join(" ");
    
    builder.trace(&widget_classes, false).unwrap();
    builder.trace(&widget_header_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify named widget container
    assert!(css.contains("@container widget"));
    assert!(css.contains("container-name"));
    
    println!("Dashboard Widget CSS generated successfully");
}

#[test]
fn example_comparison_table() {
    // A comparison table that switches layout based on container width
    let mut builder = TailwindBuilder::default();
    
    let table_classes = [
        // Container setup
        "@container",
        
        // Mobile: Card layout
        "space-y-4",
        
        // Desktop: Table layout
        "@md:block",
        "@md:overflow-x-auto",
    ].join(" ");
    
    let row_classes = [
        // Mobile: Vertical card
        "bg-white rounded-lg p-4 shadow",
        "flex flex-col gap-2",
        
        // Desktop: Table row
        "@md:flex-row",
        "@md:gap-0",
        "@md:rounded-none",
        "@md:shadow-none",
        "@md:border-b",
    ].join(" ");
    
    let cell_classes = [
        // Mobile: Stack vertically with labels
        "flex justify-between",
        "[&>span:first-child]:font-semibold",
        "[&>span:first-child]:text-gray-600",
        
        // Desktop: Equal width cells
        "@md:flex-1",
        "@md:p-4",
        "@md:[&>span:first-child]:hidden", // Hide labels in table view
    ].join(" ");
    
    builder.trace(&table_classes, false).unwrap();
    builder.trace(&row_classes, false).unwrap();
    builder.trace(&cell_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify responsive table behavior
    assert!(css.contains("@container (min-width: 28rem)")); // @md
    assert!(css.contains("flex-col"));
    assert!(css.contains("@md:flex-row") || css.contains("flex-row"));
    
    println!("Comparison Table CSS generated successfully");
}

#[test]
fn example_modal_dialog() {
    // Modal that adjusts based on its container (useful for nested modals)
    let mut builder = TailwindBuilder::default();
    
    let modal_classes = [
        // Container setup for nested content
        "@container/modal",
        
        // Base modal styles
        "bg-white rounded-lg shadow-xl",
        
        // Size based on container
        "w-full max-w-md",
        "@md/modal:max-w-lg",
        "@lg/modal:max-w-2xl",
        
        // Padding adjustments
        "p-4",
        "@md/modal:p-6",
        "@lg/modal:p-8",
    ].join(" ");
    
    let modal_content_classes = [
        // Content adapts to modal container
        "space-y-4",
        
        // Typography scaling
        "text-sm",
        "@md/modal:text-base",
        "@lg/modal:text-lg",
        
        // Button layout
        "[&_.actions]:flex",
        "[&_.actions]:flex-col",
        "[&_.actions]:gap-2",
        "@md/modal:[&_.actions]:flex-row",
        "@md/modal:[&_.actions]:justify-end",
        "@md/modal:[&_.actions]:gap-4",
    ].join(" ");
    
    builder.trace(&modal_classes, false).unwrap();
    builder.trace(&modal_content_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify modal container queries
    assert!(css.contains("@container modal"));
    assert!(css.contains("max-w-2xl"));
    
    println!("Modal Dialog CSS generated successfully");
}

#[test]
fn example_media_player() {
    // Media player that adjusts controls based on container size
    let mut builder = TailwindBuilder::default();
    
    let player_classes = [
        // Container setup
        "@container/player",
        "bg-black rounded-lg overflow-hidden",
        
        // Aspect ratio changes
        "aspect-video",
        "@lg/player:aspect-[21/9]", // Ultrawide in large containers
    ].join(" ");
    
    let controls_classes = [
        // Control bar
        "absolute bottom-0 left-0 right-0",
        "bg-gradient-to-t from-black/80 to-transparent",
        "p-2",
        "@md/player:p-4",
        
        // Controls layout
        "flex items-center gap-2",
        "@md/player:gap-4",
        
        // Hide secondary controls in small containers
        "[&_.volume]:hidden",
        "@sm/player:[&_.volume]:flex",
        
        "[&_.settings]:hidden",
        "@md/player:[&_.settings]:flex",
        
        "[&_.fullscreen]:hidden",
        "@lg/player:[&_.fullscreen]:flex",
    ].join(" ");
    
    builder.trace(&player_classes, false).unwrap();
    builder.trace(&controls_classes, false).unwrap();
    let css = builder.bundle().unwrap();
    
    // Verify player container behavior
    assert!(css.contains("@container player"));
    assert!(css.contains("aspect-"));
    
    println!("Media Player CSS generated successfully");
}

// Summary of Container Query Patterns
// 
// 1. **Component-Based Responsive Design**: Components adapt to their container, not viewport
// 2. **Named Containers**: Use named containers to scope queries to specific parents
// 3. **Progressive Enhancement**: Start with mobile layout, enhance for larger containers
// 4. **Content Adaptation**: Hide/show elements based on available space
// 5. **Typography Scaling**: Adjust text size based on reading area width
// 6. **Layout Switching**: Change from stack to grid/flex based on container
// 7. **Nested Containers**: Containers can be nested for complex layouts
// 8. **Arbitrary Values**: Use specific pixel values for precise breakpoints
// 
// Container queries enable truly reusable components that work correctly
// regardless of where they're placed in the layout.