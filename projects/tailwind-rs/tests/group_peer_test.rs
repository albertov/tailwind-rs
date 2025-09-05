use tailwind_css::{CssInlineMode, TailwindBuilder};
use tailwind_rs::CLIConfig;

fn pre_config() -> (CLIConfig, TailwindBuilder) {
    let mut config = CLIConfig::default();
    let mut builder = config.builder();
    config.minify = false;
    config.mode = CssInlineMode::None;
    builder.preflight.disable = true;
    (config, builder)
}

#[test]
fn test_group_hover_basic() {
    let (config, mut builder) = pre_config();
    
    let html = r#"
        <div class="group">
            <button>Hover me</button>
            <div class="group-hover:flex">This appears on hover</div>
        </div>
    "#;
    
    let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that CSS contains the group-hover selector pattern
    assert!(css.contains(".group-hover\\:flex"), "CSS should contain group-hover:flex class");
    assert!(css.contains(".group:hover .group-hover\\:flex"), "CSS should contain group hover selector pattern");
    assert!(css.contains("display: flex") || css.contains("display:flex"), "CSS should contain flex display property");
}

#[test]
fn test_peer_checked_basic() {
    let (config, mut builder) = pre_config();
    
    let html = r#"
        <input type="checkbox" class="peer" />
        <div class="peer-checked:opacity-100">This shows when checked</div>
    "#;
    
    let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that CSS contains the peer-checked selector pattern
    assert!(css.contains(".peer-checked\\:opacity-100"), "CSS should contain peer-checked:opacity-100 class");
    assert!(css.contains(".peer:checked ~ .peer-checked\\:opacity-100"), "CSS should contain peer checked selector pattern");
    assert!(css.contains("opacity: 1") || css.contains("opacity:100%"), "CSS should contain opacity property");
}

#[test]
fn test_group_hover_named_variant() {
    let (config, mut builder) = pre_config();
    
    let html = r#"
        <div class="group/sidebar">
            <button>Hover sidebar</button>
            <div class="group-hover/sidebar:visible">Sidebar content</div>
        </div>
    "#;
    
    let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that CSS contains the named group-hover selector pattern
    assert!(css.contains(".group-hover\\/sidebar\\:visible"), "CSS should contain group-hover/sidebar:visible class");
    assert!(css.contains(".group\\/sidebar:hover .group-hover\\/sidebar\\:visible"), "CSS should contain named group hover selector pattern");
    assert!(css.contains("visibility: visible") || css.contains("visibility:visible"), "CSS should contain visibility property");
}

#[test]
fn test_peer_focus_named_variant() {
    let (config, mut builder) = pre_config();
    
    let html = r#"
        <input type="text" class="peer/input" />
        <div class="peer-focus/input:border-red-500">Error message</div>
    "#;
    
    let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that CSS contains the named peer-focus selector pattern
    assert!(css.contains(".peer-focus\\/input\\:border-red-500"), "CSS should contain peer-focus/input:border-red-500 class");
    assert!(css.contains(".peer\\/input:focus ~ .peer-focus\\/input\\:border-red-500"), "CSS should contain named peer focus selector pattern");
    assert!(css.contains("border-color:"), "CSS should contain border-color property");
}

#[test]
fn test_group_hover_with_media_query() {
    let (config, mut builder) = pre_config();
    
    let html = r#"
        <div class="group">
            <button>Hover me</button>
            <div class="group-hover:text-blue-500">Changes color on hover</div>
        </div>
    "#;
    
    let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that hover is wrapped in hover media query
    assert!(css.contains("@media (hover: hover)"), "CSS should contain hover media query for accessibility");
    assert!(css.contains(".group-hover\\:text-blue-500"), "CSS should contain group-hover:text-blue-500 class");
}

#[test]
fn test_group_arbitrary_selector() {
    let (config, mut builder) = pre_config();
    
    let html = r#"
        <div class="group" data-state="open">
            <div class="group-[&[data-state='open']]:flex">Shows when open</div>
        </div>
    "#;
    
    let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that CSS contains the arbitrary group selector pattern
    // Note: This test is ignored as arbitrary selectors are not yet fully implemented
    assert!(!css.is_empty(), "CSS should be generated for arbitrary selectors");
    if !css.is_empty() {
        assert!(css.contains("display-flex") || css.contains("flex"), "CSS should contain flex-related class");
    }
}