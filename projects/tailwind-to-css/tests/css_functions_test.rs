use tailwind_css::TailwindBuilder;

#[test]
fn test_width_calc_expressions() {
    let test_cases = vec![
        ("w-[calc(100%-2rem)]", "width:calc(100%-2rem)"),
        ("w-[var(--width)]", "width:var(--width)"),
        ("w-[clamp(10px,5vw,100px)]", "width:clamp(10px,5vw,100px)"),
        ("w-[min(100%,1200px)]", "width:min(100%,1200px)"),
        ("w-[max(300px,50%)]", "width:max(300px,50%)"),
    ];

    for (class, expected) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected,
                    css.lines().find(|l| l.contains(class)).unwrap_or("not found")
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_height_calc_expressions() {
    let test_cases = vec![
        ("h-[calc(100vh-4rem)]", "height:calc(100vh-4rem)"),
        ("h-[var(--height)]", "height:var(--height)"),
        ("h-[clamp(200px,50vh,800px)]", "height:clamp(200px,50vh,800px)"),
        ("h-[min(100vh,600px)]", "height:min(100vh,600px)"),
        ("h-[max(400px,80vh)]", "height:max(400px,80vh)"),
    ];

    for (class, expected) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected,
                    css.lines().find(|l| l.contains("height")).unwrap_or("not found")
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_nested_calc_expressions() {
    let test_cases = vec![
        ("w-[calc(var(--base-width,100%)-2rem)]", "width:calc(var(--base-width,100%)-2rem)"),
        ("h-[calc(100vh-calc(2rem+10px))]", "height:calc(100vh-calc(2rem+10px))"),
    ];

    for (class, expected) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected,
                    css.lines().find(|l| l.contains(if class.starts_with("w-") { "width" } else { "height" })).unwrap_or("not found")
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}

#[test]
fn test_min_max_width_calc() {
    let test_cases = vec![
        ("min-w-[calc(100%-2rem)]", "min-width:calc(100%-2rem)"),
        ("max-w-[calc(100%+1rem)]", "max-width:calc(100%+1rem)"),
        ("min-h-[calc(50vh-20px)]", "min-height:calc(50vh-20px)"),
        ("max-h-[calc(100vh-80px)]", "max-height:calc(100vh-80px)"),
    ];

    for (class, expected) in test_cases {
        let mut builder = TailwindBuilder::default();
        match builder.trace(class, false) {
            Ok(_) => {
                let css = builder.bundle().unwrap_or_default();
                assert!(
                    css.contains(expected),
                    "CSS for {} should contain '{}', but got: {}",
                    class,
                    expected,
                    css.lines().find(|l| l.contains(class)).unwrap_or("not found")
                );
            }
            Err(e) => panic!("{} failed to parse: {:?}", class, e),
        }
    }
}