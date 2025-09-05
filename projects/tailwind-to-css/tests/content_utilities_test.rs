use tailwind_css::TailwindBuilder;

#[test]
fn test_content_none() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-none", false).expect("Failed to parse content-none");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:none"));
    assert!(css.contains("content:none"));
}

#[test]
fn test_content_empty_string() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['']", false).expect("Failed to parse content-['']");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:''"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_simple_text() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['Hello']", false).expect("Failed to parse content-['Hello']");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:'Hello'"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_with_spaces() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['Hello_World']", false).expect("Failed to parse content with spaces");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:'Hello World'"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_with_escaped_underscore() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['file\\_name.txt']", false).expect("Failed to parse content with escaped underscore");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:'file_name.txt'"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_attr_function() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-[attr(data-label)]", false).expect("Failed to parse content-[attr(data-label)]");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:attr(data-label)"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_url_function() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-[url('/icon.svg')]", false).expect("Failed to parse content-[url('/icon.svg')]");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:url('/icon.svg')"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_counter_function() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-[counter(chapter)]", false).expect("Failed to parse content-[counter(chapter)]");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:counter(chapter)"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_var_function() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-[var(--my-content)]", false).expect("Failed to parse content-[var(--my-content)]");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:var(--my-content)"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_css_keywords() {
    for keyword in ["normal", "inherit", "initial", "unset", "revert"] {
        let mut builder = TailwindBuilder::default();
        let class_str = format!("content-[{}]", keyword);
        builder.trace(&class_str, false).expect(&format!("Failed to parse content-[{}]", keyword));
        
        let css = builder.bundle().unwrap_or_default();
        
        assert!(css.contains(&format!("--tw-content:{}", keyword)));
        assert!(css.contains("content:var(--tw-content)"));
    }
}

#[test]
fn test_content_with_pseudo_elements() {
    // Test before:content-['→']
    let mut builder = TailwindBuilder::default();
    builder.trace("before:content-['→']", false).expect("Failed to parse before:content");
    let css = builder.bundle().unwrap_or_default();
    assert!(css.contains("::before"));
    assert!(css.contains("--tw-content:'→'"));
    assert!(css.contains("content:var(--tw-content)"));
    
    // Test after:content-['']
    let mut builder = TailwindBuilder::default();
    builder.trace("after:content-['']", false).expect("Failed to parse after:content");
    let css = builder.bundle().unwrap_or_default();
    assert!(css.contains("::after"));
    assert!(css.contains("--tw-content:''"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_with_hover_variant() {
    // Test hover:before:content-['hover']
    let mut builder = TailwindBuilder::default();
    builder.trace("hover:before:content-['hover']", false).expect("Failed to parse hover:before:content");
    let css = builder.bundle().unwrap_or_default();
    assert!(css.contains(":hover::before"));
    assert!(css.contains("--tw-content:'hover'"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_multiple_underscores() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['__foo__bar__']", false).expect("Failed to parse content with multiple underscores");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:'  foo  bar  '"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_mixed_underscores() {
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['hello_world\\_file']", false).expect("Failed to parse content with mixed underscores");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:'hello world_file'"));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_with_quotes() {
    // Double quotes already present
    let mut builder = TailwindBuilder::default();
    builder.trace("content-[\"quoted\"]", false).expect("Failed to parse content with quotes");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:\"quoted\""));
    assert!(css.contains("content:var(--tw-content)"));
}

#[test]
fn test_content_complex_combination() {
    // Test a complex content string
    let mut builder = TailwindBuilder::default();
    builder.trace("content-['Step_\\_1:_Complete']", false).expect("Failed to parse complex content");
    
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("--tw-content:'Step _1: Complete'"));
    assert!(css.contains("content:var(--tw-content)"));
}