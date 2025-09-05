use super::*;

#[test]
fn test_gradient_trace() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    let (html, css) = config.compile_html(include_str!("gradient.html"), &mut builder).unwrap();
    std::fs::write("tests/html/gradient/gradient.traced.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/gradient/gradient.traced.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("gradient.traced.html"));
    assert_eq!(css, include_str!("gradient.traced.css"));
}

#[test]
fn test_gradient_inline() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::Inline;
    let (html, css) = config.compile_html(include_str!("gradient.html"), &mut builder).unwrap();
    std::fs::write("tests/html/gradient/gradient.inline.html", html.as_bytes()).unwrap();
    std::fs::write("tests/html/gradient/gradient.inline.css", css.as_bytes()).unwrap();
    assert_eq!(html, include_str!("gradient.inline.html"));
    assert_eq!(css, include_str!("gradient.inline.css"));
}