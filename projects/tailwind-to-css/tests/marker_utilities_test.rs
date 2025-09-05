use tailwind_css::TailwindBuilder;

#[test]
fn test_group_marker() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("group", false);
    let css = tw.bundle().unwrap();
    assert!(css.contains(".group { }"), "CSS should contain '.group {{ }}', got: {}", css);
}

#[test]
fn test_peer_marker() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("peer", false);
    let css = tw.bundle().unwrap();
    assert!(css.contains(".peer { }"), "CSS should contain '.peer {{ }}', got: {}", css);
}

#[test]
fn test_group_hover_with_marker() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("group", false);
    let _ = tw.trace("group-hover:text-blue-500", false);
    let css = tw.bundle().unwrap();
    
    // Should contain both the marker and the group-hover rule
    assert!(css.contains(".group { }"), "CSS should contain '.group {{ }}', got: {}", css);
    assert!(css.contains(".group:hover .group-hover\\:text-blue-500"), "CSS should contain group hover rule");
}

#[test]
fn test_peer_checked_with_marker() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("peer", false);
    let _ = tw.trace("peer-checked:bg-green-500", false);
    let css = tw.bundle().unwrap();
    
    // Should contain both the marker and the peer-checked rule
    assert!(css.contains(".peer { }"), "CSS should contain '.peer {{ }}', got: {}", css);
    assert!(css.contains(".peer:checked ~ .peer-checked\\:bg-green-500"), "CSS should contain peer checked rule");
}

#[test]
fn test_group_and_peer_together() {
    let mut tw = TailwindBuilder::default();
    let _ = tw.trace("group", false);
    let _ = tw.trace("peer", false);
    let css = tw.bundle().unwrap();
    
    // Both markers should be present
    assert!(css.contains(".group { }"), "CSS should contain '.group {{ }}', got: {}", css);
    assert!(css.contains(".peer { }"), "CSS should contain '.peer {{ }}', got: {}", css);
}

#[test]
fn test_inline_group() {
    let mut tw = TailwindBuilder::default();
    let result = tw.inline("group");
    
    assert!(result.is_ok(), "Should successfully parse 'group' class");
    let (class, style) = result.unwrap();
    assert_eq!(class, "group", "Class should be 'group'");
    assert_eq!(style, "", "Group marker should produce empty inline styles");
}