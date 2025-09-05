use tailwind_rs::TailwindBuilder;

#[test]
fn test_animations_work() {
    let mut tw = TailwindBuilder::default();
    
    // Test all animation utilities
    tw.trace("animate-none", false).unwrap();
    tw.trace("animate-spin", false).unwrap();
    tw.trace("animate-ping", false).unwrap();
    tw.trace("animate-pulse", false).unwrap();
    tw.trace("animate-bounce", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify animations are present
    assert!(css.contains("animation:none"));
    assert!(css.contains("animation:spin"));
    assert!(css.contains("animation:ping"));
    assert!(css.contains("animation:pulse"));
    assert!(css.contains("animation:bounce"));
    
    // Verify keyframes are present
    assert!(css.contains("@keyframes spin"));
    assert!(css.contains("@keyframes ping"));
    assert!(css.contains("@keyframes pulse"));
    assert!(css.contains("@keyframes bounce"));
    
    println!("All animations work correctly!");
}