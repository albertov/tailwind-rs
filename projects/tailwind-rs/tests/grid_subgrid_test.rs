use tailwind_rs::TailwindBuilder;

#[test]
fn test_grid_rows_subgrid() {
    let mut tw = TailwindBuilder::default();
    tw.trace("grid-rows-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the grid-template-rows property set to subgrid
    assert!(css.contains("grid-template-rows:subgrid"));
}

#[test]
fn test_grid_cols_subgrid() {
    let mut tw = TailwindBuilder::default();
    tw.trace("grid-cols-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the grid-template-columns property set to subgrid
    assert!(css.contains("grid-template-columns:subgrid"));
}

#[test]
fn test_grid_subgrid_with_hover_modifier() {
    let mut tw = TailwindBuilder::default();
    tw.trace("hover:grid-rows-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain hover pseudo-class with grid-template-rows
    assert!(css.contains(":hover"));
    assert!(css.contains("grid-template-rows:subgrid"));
}

#[test]
fn test_grid_subgrid_with_responsive_modifier() {
    let mut tw = TailwindBuilder::default();
    tw.trace("sm:grid-cols-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain media query with grid-template-columns
    assert!(css.contains("@media"));
    assert!(css.contains("grid-template-columns:subgrid"));
}

#[test]
fn test_grid_subgrid_with_dark_mode() {
    let mut tw = TailwindBuilder::default();
    tw.trace("dark:grid-rows-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain dark mode selector with grid-template-rows
    assert!(css.contains("grid-template-rows:subgrid"));
}

#[test]
fn test_multiple_grid_utilities() {
    let mut tw = TailwindBuilder::default();
    tw.trace("grid grid-cols-subgrid gap-4", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain grid display and subgrid columns
    assert!(css.contains("display:grid"));
    assert!(css.contains("grid-template-columns:subgrid"));
    assert!(css.contains("gap:1rem")); // gap-4 is 1rem
}

#[test]
fn test_grid_cols_subgrid_with_important() {
    let mut tw = TailwindBuilder::default();
    tw.trace("!grid-cols-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain important flag
    assert!(css.contains("grid-template-columns:subgrid!important") || 
            css.contains("grid-template-columns:subgrid !important"));
}

#[test]
fn test_grid_rows_subgrid_with_important() {
    let mut tw = TailwindBuilder::default();
    tw.trace("!grid-rows-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain important flag
    assert!(css.contains("grid-template-rows:subgrid!important") || 
            css.contains("grid-template-rows:subgrid !important"));
}

#[test]
fn test_grid_subgrid_with_focus_modifier() {
    let mut tw = TailwindBuilder::default();
    tw.trace("focus:grid-cols-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain focus pseudo-class
    assert!(css.contains(":focus"));
    assert!(css.contains("grid-template-columns:subgrid"));
}

#[test]
fn test_grid_subgrid_with_lg_breakpoint() {
    let mut tw = TailwindBuilder::default();
    tw.trace("lg:grid-rows-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain media query for large screens
    assert!(css.contains("@media"));
    assert!(css.contains("grid-template-rows:subgrid"));
}

#[test]
fn test_combined_grid_and_subgrid() {
    let mut tw = TailwindBuilder::default();
    tw.trace("grid-cols-3 md:grid-cols-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain both regular grid and subgrid with media query
    assert!(css.contains("grid-template-columns:repeat(3,minmax(0,1fr))"));
    assert!(css.contains("@media"));
    assert!(css.contains("grid-template-columns:subgrid"));
}

#[test]
fn test_grid_rows_switching_to_subgrid() {
    let mut tw = TailwindBuilder::default();
    tw.trace("grid-rows-6 xl:grid-rows-subgrid", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain both regular grid rows and subgrid at xl breakpoint
    assert!(css.contains("grid-template-rows:repeat(6,minmax(0,1fr))"));
    assert!(css.contains("@media"));
    assert!(css.contains("grid-template-rows:subgrid"));
}