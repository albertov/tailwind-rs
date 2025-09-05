use tailwind_rs::TailwindBuilder;

#[test]
fn test_space_utilities_idempotent() {
    let mut tw = TailwindBuilder::default();
    
    // Trace the same class multiple times
    println!("\n=== Testing Idempotency ===");
    
    // First trace
    tw.trace("space-x-4", false).unwrap();
    let bundle1 = tw.bundle().unwrap();
    let count1 = bundle1.matches(".space-x-4").count();
    println!("After 1st trace: {} occurrences of .space-x-4", count1);
    
    // Second trace (same class)
    tw.trace("space-x-4", false).unwrap();
    let bundle2 = tw.bundle().unwrap();
    let count2 = bundle2.matches(".space-x-4").count();
    println!("After 2nd trace: {} occurrences of .space-x-4", count2);
    
    // Third trace (same class)
    tw.trace("space-x-4", false).unwrap();
    let bundle3 = tw.bundle().unwrap();
    let count3 = bundle3.matches(".space-x-4").count();
    println!("After 3rd trace: {} occurrences of .space-x-4", count3);
    
    // Multiple classes including duplicates
    tw.trace("space-x-4 space-y-2 space-x-4", false).unwrap();
    let bundle4 = tw.bundle().unwrap();
    let count4 = bundle4.matches(".space-x-4").count();
    println!("After mixed trace: {} occurrences of .space-x-4", count4);
    
    // Verify idempotency
    assert_eq!(count1, count2, "Should be idempotent after 2nd trace");
    assert_eq!(count2, count3, "Should be idempotent after 3rd trace");
    assert_eq!(count3, count4, "Should be idempotent after mixed trace");
    
    println!("\n✅ Idempotent property verified!");
}
