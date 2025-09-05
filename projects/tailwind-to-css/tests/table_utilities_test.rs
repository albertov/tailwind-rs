use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    builder.trace(class, false).unwrap();
    let css = builder.bundle().unwrap_or_default();
    
    // Handle special cases where the CSS class name differs from input
    // For arbitrary values, remove hyphen before bracket: "border-spacing-[10px]" => "border-spacing[10px]"
    let css_class = if class.contains("[") {
        class
            .replace("-[", "[")  // Remove hyphen before bracket for arbitrary values
            .replace(":", "\\:")
            .replace("[", "\\[")
            .replace("]", "\\]")
            .replace(".", "\\.")
    } else if class.ends_with("-px") && !class.contains("[") {
        class.replace("-px", "\\[1px\\]")
    } else {
        class
            .replace(":", "\\:")
            .replace("[", "\\[")
            .replace("]", "\\]")
            .replace(".", "\\.")
    };
    
    // Extract the CSS rules for the class
    for line in css.lines() {
        if line.contains(&format!(".{}", css_class)) {
            // Extract just the CSS properties without selector
            if let Some(start) = line.find('{') {
                if let Some(end) = line.rfind('}') {
                    return line[start+1..end].trim().to_string();
                }
            }
        }
    }
    
    // If not found, maybe it's in a media query - return empty for now
    String::new()
}

#[test]
fn test_border_spacing_all() {
    // Standard spacing values
    assert_eq!(
        generate_css("border-spacing-0"),
        "border-spacing:0px;"
    );
    assert_eq!(
        generate_css("border-spacing-px"),
        "border-spacing:1px;"
    );
    assert_eq!(
        generate_css("border-spacing-0.5"),
        "border-spacing:0.125rem;"
    );
    assert_eq!(
        generate_css("border-spacing-1"),
        "border-spacing:0.25rem;"
    );
    assert_eq!(
        generate_css("border-spacing-2"),
        "border-spacing:0.5rem;"
    );
    assert_eq!(
        generate_css("border-spacing-4"),
        "border-spacing:1rem;"
    );
    assert_eq!(
        generate_css("border-spacing-8"),
        "border-spacing:2rem;"
    );
    assert_eq!(
        generate_css("border-spacing-16"),
        "border-spacing:4rem;"
    );
    assert_eq!(
        generate_css("border-spacing-32"),
        "border-spacing:8rem;"
    );
    assert_eq!(
        generate_css("border-spacing-64"),
        "border-spacing:16rem;"
    );
    assert_eq!(
        generate_css("border-spacing-96"),
        "border-spacing:24rem;"
    );
}

#[test]
fn test_border_spacing_x_axis() {
    assert_eq!(
        generate_css("border-spacing-x-0"),
        "--tw-border-spacing-x:0px;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-x-px"),
        "--tw-border-spacing-x:1px;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-x-1"),
        "--tw-border-spacing-x:0.25rem;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-x-4"),
        "--tw-border-spacing-x:1rem;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-x-8"),
        "--tw-border-spacing-x:2rem;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
}

#[test]
fn test_border_spacing_y_axis() {
    assert_eq!(
        generate_css("border-spacing-y-0"),
        "--tw-border-spacing-y:0px;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-y-px"),
        "--tw-border-spacing-y:1px;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-y-1"),
        "--tw-border-spacing-y:0.25rem;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-y-4"),
        "--tw-border-spacing-y:1rem;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-y-8"),
        "--tw-border-spacing-y:2rem;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
}

#[test]
fn test_border_spacing_arbitrary() {
    assert_eq!(
        generate_css("border-spacing-[10px]"),
        "border-spacing:10px;"
    );
    assert_eq!(
        generate_css("border-spacing-[2em]"),
        "border-spacing:2em;"
    );
    assert_eq!(
        generate_css("border-spacing-[1.5rem]"),
        "border-spacing:1.5rem;"
    );
    assert_eq!(
        generate_css("border-spacing-x-[20px]"),
        "--tw-border-spacing-x:20px;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
    assert_eq!(
        generate_css("border-spacing-y-[30px]"),
        "--tw-border-spacing-y:30px;border-spacing:var(--tw-border-spacing-x) var(--tw-border-spacing-y);"
    );
}

#[test]
fn test_border_spacing_with_modifiers() {
    // Just test that these classes are recognized
    assert!(!generate_css("hover:border-spacing-2").is_empty());
    assert!(!generate_css("focus:border-spacing-4").is_empty());
    assert!(!generate_css("sm:border-spacing-x-2").is_empty());
    assert!(!generate_css("md:border-spacing-y-4").is_empty());
}

#[test]
fn test_caption_side() {
    assert_eq!(
        generate_css("caption-top"),
        "caption-side:top;"
    );
    assert_eq!(
        generate_css("caption-bottom"),
        "caption-side:bottom;"
    );
}

#[test]
fn test_caption_side_arbitrary() {
    assert_eq!(
        generate_css("caption-[block-start]"),
        "caption-side:block-start;"
    );
    assert_eq!(
        generate_css("caption-[block-end]"),
        "caption-side:block-end;"
    );
    assert_eq!(
        generate_css("caption-[inline-start]"),
        "caption-side:inline-start;"
    );
    assert_eq!(
        generate_css("caption-[inline-end]"),
        "caption-side:inline-end;"
    );
}

#[test]
fn test_caption_side_with_modifiers() {
    // Just test that these classes are recognized
    assert!(!generate_css("hover:caption-top").is_empty());
    assert!(!generate_css("focus:caption-bottom").is_empty());
    assert!(!generate_css("sm:caption-top").is_empty());
    assert!(!generate_css("lg:caption-bottom").is_empty());
}

#[test]
fn test_existing_table_utilities() {
    // Ensure existing utilities still work
    assert_eq!(
        generate_css("border-collapse"),
        "border-collapse:collapse;"
    );
    assert_eq!(
        generate_css("border-separate"),
        "border-collapse:separate;"
    );
    assert_eq!(
        generate_css("table-auto"),
        "table-layout:auto;"
    );
    assert_eq!(
        generate_css("table-fixed"),
        "table-layout:fixed;"
    );
}

#[test]
fn test_table_display_utilities() {
    // Ensure table display utilities aren't confused with caption-side
    assert_eq!(
        generate_css("table"),
        "display:table;"
    );
    assert_eq!(
        generate_css("table-caption"),
        "display:table-caption;"
    );
    assert_eq!(
        generate_css("table-cell"),
        "display:table-cell;"
    );
    assert_eq!(
        generate_css("table-row"),
        "display:table-row;"
    );
}

#[test]
fn test_combined_table_utilities() {
    // Test that multiple table utilities can be used together
    let _combined = "table border-separate border-spacing-2 caption-bottom";
    // Note:convert_classname expects a single class, so we test individually
    assert!(generate_css("table").contains("display:table"));
    assert!(generate_css("border-separate").contains("border-collapse:separate"));
    assert!(generate_css("border-spacing-2").contains("border-spacing:"));
    assert!(generate_css("caption-bottom").contains("caption-side:bottom"));
}