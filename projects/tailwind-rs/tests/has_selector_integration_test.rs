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

/// Helper function to generate CSS from a single Tailwind class
fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    builder.preflight.disable = true;
    builder.trace(class, false).unwrap();
    builder.bundle().unwrap()
}

#[cfg(test)]
mod basic_has_variants {
    use super::*;

    #[test]
    fn test_has_checked_bg_green() {
        let css = generate_css("has-checked:bg-green-500");
        println!("Generated CSS for has-checked:bg-green-500:\n{}", css);
        
        assert!(css.contains(":has(:checked)"), 
            "CSS should contain :has(:checked) selector. Got: {}", css);
        assert!(css.contains("background-color") || css.contains("background:"), 
            "CSS should contain background property");
    }

    #[test]
    fn test_has_focus_ring() {
        let css = generate_css("has-focus:ring-4");
        println!("Generated CSS for has-focus:ring-4:\n{}", css);
        
        assert!(css.contains(":has(:focus)"), 
            "CSS should contain :has(:focus) selector");
        assert!(css.contains("box-shadow") || css.contains("--tw-ring-offset-shadow"), 
            "CSS should contain ring properties");
    }

    #[test]
    fn test_has_hover_shadow() {
        let css = generate_css("has-hover:shadow-lg");
        println!("Generated CSS for has-hover:shadow-lg:\n{}", css);
        
        assert!(css.contains(":has(:hover)"), 
            "CSS should contain :has(:hover) selector");
        assert!(css.contains("box-shadow") || css.contains("--tw-shadow"), 
            "CSS should contain shadow properties");
    }

    #[test]
    fn test_has_disabled_opacity() {
        let css = generate_css("has-disabled:opacity-50");
        println!("Generated CSS for has-disabled:opacity-50:\n{}", css);
        
        assert!(css.contains(":has(:disabled)"), 
            "CSS should contain :has(:disabled) selector");
        assert!(css.contains("opacity"), 
            "CSS should contain opacity property");
    }

    #[test]
    fn test_has_empty_hidden() {
        let css = generate_css("has-empty:hidden");
        println!("Generated CSS for has-empty:hidden:\n{}", css);
        
        assert!(css.contains(":has(:empty)"), 
            "CSS should contain :has(:empty) selector");
        assert!(css.contains("display: none") || css.contains("display:none"), 
            "CSS should contain display: none");
    }

    #[test]
    fn test_has_required_border() {
        let css = generate_css("has-required:border-red-500");
        println!("Generated CSS for has-required:border-red-500:\n{}", css);
        
        assert!(css.contains(":has(:required)"), 
            "CSS should contain :has(:required) selector");
        assert!(css.contains("border-color") || css.contains("border:"), 
            "CSS should contain border properties");
    }

    #[test]
    fn test_has_invalid_text() {
        let css = generate_css("has-invalid:text-red-600");
        println!("Generated CSS for has-invalid:text-red-600:\n{}", css);
        
        assert!(css.contains(":has(:invalid)"), 
            "CSS should contain :has(:invalid) selector");
        assert!(css.contains("color"), 
            "CSS should contain color property");
    }
}

#[cfg(test)]
mod arbitrary_selectors {
    use super::*;

    #[test]
    fn test_has_arbitrary_child_img() {
        let css = generate_css("has-[>img]:flex");
        println!("Generated CSS for has-[>img]:flex:\n{}", css);
        
        // Child combinators need a space before them in :has()
        assert!(css.contains(":has( >img)") || css.contains(":has(>img)"), 
            "CSS should contain :has(>img) selector. Got: {}", css);
        assert!(css.contains("display: flex") || css.contains("display:flex"), 
            "CSS should contain display: flex");
    }

    #[test]
    fn test_has_arbitrary_class_error() {
        let css = generate_css("has-[.error]:bg-red-500");
        println!("Generated CSS for has-[.error]:bg-red-500:\n{}", css);
        
        assert!(css.contains(":has(.error)"), 
            "CSS should contain :has(.error) selector");
        assert!(css.contains("background-color") || css.contains("background:"), 
            "CSS should contain background property");
    }

    #[test]
    fn test_has_arbitrary_input_checked() {
        let css = generate_css("has-[input:checked]:bg-green-100");
        println!("Generated CSS for has-[input:checked]:bg-green-100:\n{}", css);
        
        assert!(css.contains(":has(input:checked)"), 
            "CSS should contain :has(input:checked) selector");
        assert!(css.contains("background-color") || css.contains("background:"), 
            "CSS should contain background property");
    }

    #[test]
    fn test_has_arbitrary_data_attribute() {
        let css = generate_css("has-[[data-active]]:ring-2");
        println!("Generated CSS for has-[[data-active]]:ring-2:\n{}", css);
        
        assert!(css.contains(":has([data-active])"), 
            "CSS should contain :has([data-active]) selector");
        assert!(css.contains("box-shadow") || css.contains("--tw-ring"), 
            "CSS should contain ring properties");
    }

    #[test]
    fn test_has_arbitrary_complex_selector() {
        let css = generate_css("has-[>div:first-child]:p-4");
        println!("Generated CSS for has-[>div:first-child]:p-4:\n{}", css);
        
        assert!(css.contains(":has( >div:first-child)") || css.contains(":has(>div:first-child)"), 
            "CSS should contain :has(>div:first-child) selector");
        assert!(css.contains("padding"), 
            "CSS should contain padding property");
    }

    #[test]
    fn test_has_arbitrary_adjacent_sibling() {
        let css = generate_css("has-[+p]:mt-4");
        println!("Generated CSS for has-[+p]:mt-4:\n{}", css);
        
        assert!(css.contains(":has( +p)") || css.contains(":has(+p)"), 
            "CSS should contain :has(+p) selector");
        assert!(css.contains("margin-top"), 
            "CSS should contain margin-top property");
    }

    #[test]
    fn test_has_arbitrary_general_sibling() {
        let css = generate_css("has-[~span]:text-gray-500");
        println!("Generated CSS for has-[~span]:text-gray-500:\n{}", css);
        
        assert!(css.contains(":has( ~span)") || css.contains(":has(~span)"), 
            "CSS should contain :has(~span) selector");
        assert!(css.contains("color"), 
            "CSS should contain color property");
    }
}

#[cfg(test)]
mod group_peer_combinations {
    use super::*;

    #[test]
    fn test_group_has_checked() {
        let (config, mut builder) = pre_config();
        
        let html = r#"
            <div class="group">
                <input type="checkbox" />
                <div class="group-has-checked:text-green-600">Checked!</div>
            </div>
        "#;
        
        let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
        println!("Generated CSS for group-has-checked:\n{}", css);
        
        assert!(css.contains(".group-has-checked\\:text-green-600"), 
            "CSS should contain the escaped class name");
        assert!(css.contains(":is(:where(.group):has(:checked)"), 
            "CSS should contain group has checked pattern");
        assert!(css.contains("color"), 
            "CSS should contain color property");
    }

    #[test]
    fn test_peer_has_focus() {
        let (config, mut builder) = pre_config();
        
        let html = r#"
            <input class="peer" type="text" />
            <div class="peer-has-focus:ring-4">Focus indicator</div>
        "#;
        
        let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
        println!("Generated CSS for peer-has-focus:\n{}", css);
        
        assert!(css.contains(".peer-has-focus\\:ring-4"), 
            "CSS should contain the escaped class name");
        assert!(css.contains(":is(:where(.peer):has(:focus)"), 
            "CSS should contain peer has focus pattern");
    }

    #[test]
    // #[ignore = "HTML processing of arbitrary group-has selectors not yet implemented"]
    fn test_group_has_arbitrary() {
        let (config, mut builder) = pre_config();
        
        let html = r#"
            <div class="group">
                <div class="error">Error message</div>
                <button class="group-has-[.error]:text-red-600">Submit</button>
            </div>
        "#;
        
        let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
        println!("Generated CSS for group-has-[.error]:\n{}", css);
        
        assert!(css.contains(".group-has-\\[\\.error\\]\\:text-red-600"), 
            "CSS should contain the escaped class name");
        assert!(css.contains(":is(:where(.group):has(.error)"), 
            "CSS should contain group has error pattern");
    }

    #[test]
    fn test_group_has_with_modifier() {
        let (config, mut builder) = pre_config();
        
        let html = r#"
            <div class="group/sidebar">
                <input type="checkbox" />
                <div class="group-has-checked/sidebar:bg-green-100">Sidebar checked</div>
            </div>
        "#;
        
        let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
        println!("Generated CSS for group-has-checked/sidebar:\n{}", css);
        
        assert!(css.contains(".group-has-checked\\/sidebar\\:bg-green-100"), 
            "CSS should contain the escaped class name with modifier");
        assert!(css.contains(":is(:where(.group\\/sidebar):has(:checked)"), 
            "CSS should contain group has with modifier pattern");
    }

    #[test]
    // #[ignore = "HTML processing of peer-has with modifiers not yet implemented"]
    fn test_peer_has_with_modifier() {
        let (config, mut builder) = pre_config();
        
        let html = r#"
            <input class="peer/input" type="text" />
            <div class="peer-has-[:invalid]/input:text-red-500">Invalid input</div>
        "#;
        
        let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
        println!("Generated CSS for peer-has-[:invalid]/input:\n{}", css);
        
        assert!(css.contains(".peer-has-\\[\\:invalid\\]\\/input\\:text-red-500"), 
            "CSS should contain the escaped class name with modifier");
        assert!(css.contains(":is(:where(.peer\\/input):has(:invalid)"), 
            "CSS should contain peer has with modifier pattern");
    }
}

#[cfg(test)]
mod complex_scenarios {
    use super::*;

    #[test]
    fn test_multiple_has_selectors() {
        let css = generate_css("has-checked:has-focus:bg-green-500");
        println!("Generated CSS for has-checked:has-focus:bg-green-500:\n{}", css);
        
        // Multiple has selectors should chain
        assert!(css.contains(":has(:checked)") && css.contains(":has(:focus)"), 
            "CSS should contain both has selectors");
        assert!(css.contains("background-color") || css.contains("background:"), 
            "CSS should contain background property");
    }

    #[test]
    fn test_has_with_responsive_breakpoint() {
        let css = generate_css("sm:has-checked:bg-green-500");
        println!("Generated CSS for sm:has-checked:bg-green-500:\n{}", css);
        
        assert!(css.contains("@media") && css.contains("min-width"), 
            "CSS should contain responsive media query");
        assert!(css.contains(":has(:checked)"), 
            "CSS should contain has selector");
        assert!(css.contains("background-color") || css.contains("background:"), 
            "CSS should contain background property");
    }

    #[test]
    fn test_has_with_dark_mode() {
        let css = generate_css("dark:has-focus:ring-white");
        println!("Generated CSS for dark:has-focus:ring-white:\n{}", css);
        
        assert!(css.contains("@media") && css.contains("prefers-color-scheme"), 
            "CSS should contain dark mode media query");
        assert!(css.contains(":has(:focus)"), 
            "CSS should contain has selector");
    }

    #[test]
    fn test_has_not_checked() {
        let css = generate_css("has-not-checked:opacity-50");
        println!("Generated CSS for has-not-checked:opacity-50:\n{}", css);
        
        // This should generate :has(:not(:checked)) or similar
        assert!(css.contains(":has(:not(:checked))") || css.contains(":not(:has(:checked))"), 
            "CSS should contain negated has selector. Got: {}", css);
        assert!(css.contains("opacity"), 
            "CSS should contain opacity property");
    }

    #[test]
    // #[ignore = "Complex HTML integration with has-selector not yet fully implemented"]
    fn test_complex_html_integration() {
        let (config, mut builder) = pre_config();
        
        let html = r#"
            <div class="has-[>img]:flex has-[.error]:bg-red-100">
                <img src="photo.jpg" />
                <span class="error">Error message</span>
            </div>
            <form class="group has-[:invalid]:border-red-500">
                <input type="email" required />
                <button class="group-has-[:invalid]:opacity-50">Submit</button>
            </form>
        "#;
        
        let (_processed_html, css) = config.compile_html(html, &mut builder).unwrap();
        println!("Generated CSS for complex HTML:\n{}", css);
        
        // Check multiple has selectors are present
        assert!(css.contains(":has( >img)") || css.contains(":has(>img)") || css.contains(":has( > img)"), 
            "CSS should contain has img selector");
        assert!(css.contains(":has(.error)"), 
            "CSS should contain has error selector");
        assert!(css.contains(":has(:invalid)"), 
            "CSS should contain has invalid selector");
        assert!(css.contains("display") && css.contains("background-color") && css.contains("border-color"), 
            "CSS should contain all relevant properties");
    }

    #[test]
    fn test_has_with_hover_state() {
        let css = generate_css("hover:has-checked:bg-green-600");
        println!("Generated CSS for hover:has-checked:bg-green-600:\n{}", css);
        
        assert!(css.contains("hover: hover") && css.contains(":has(:checked)"), 
            "CSS should contain both hover and has selectors");
        assert!(css.contains("background-color") || css.contains("background:"), 
            "CSS should contain background property");
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_has_with_important() {
        let css = generate_css("!has-checked:bg-green-500");
        println!("Generated CSS for !has-checked:bg-green-500:\n{}", css);
        
        assert!(css.contains(":has(:checked)"), 
            "CSS should contain has selector");
        assert!(css.contains("!important"), 
            "CSS should contain !important flag");
    }

    #[test]
    fn test_has_with_opacity_modifier() {
        let css = generate_css("has-checked:bg-green-500/50");
        println!("Generated CSS for has-checked:bg-green-500/50:\n{}", css);
        
        assert!(css.contains(":has(:checked)"), 
            "CSS should contain has selector");
        // Check for opacity in the color or as rgb/rgba
        assert!(css.contains("0.5") || css.contains("50%") || css.contains("rgba"), 
            "CSS should contain opacity modifier in color");
    }

    #[test]
    fn test_has_with_negative_value() {
        let css = generate_css("has-checked:-mt-2");
        println!("Generated CSS for has-checked:-mt-2:\n{}", css);
        
        assert!(css.contains(":has(:checked)"), 
            "CSS should contain has selector");
        assert!(css.contains("margin-top") && css.contains("-"), 
            "CSS should contain negative margin-top");
    }

    #[test]
    fn test_empty_arbitrary_selector() {
        // This should probably be handled gracefully
        let css = generate_css("has-[]:bg-red-500");
        println!("Generated CSS for has-[]:bg-red-500:\n{}", css);
        
        // Should either generate nothing or handle gracefully
        // Just check it doesn't panic
        assert!(css.is_empty() || css.contains("has-"), 
            "Should handle empty arbitrary selector gracefully");
    }
}

#[cfg(test)]
mod class_name_escaping {
    use super::*;

    #[test]
    fn test_has_arbitrary_bracket_escaping() {
        let css = generate_css("has-[>img]:flex");
        
        // Check that the class name is properly escaped
        assert!(css.contains("has-\\[\\>img\\]\\:flex") || 
                css.contains("has-\\[>img\\]\\:flex"), 
            "Should escape brackets in class name. Got: {}", css);
    }

    #[test]
    fn test_group_has_slash_escaping() {
        let css = generate_css("group-has-focus/sidebar:bg-blue-100");
        
        // Check that the slash is properly escaped
        assert!(css.contains("group-has-focus\\/sidebar\\:bg-blue-100"), 
            "Should escape slash in modifier. Got: {}", css);
    }

    #[test]
    fn test_has_colon_escaping() {
        let css = generate_css("has-[:invalid]:text-red-500");
        
        // Check that colons in arbitrary selectors are handled
        assert!(css.contains("has-\\[\\:invalid\\]\\:text-red-500"), 
            "Should escape colons in arbitrary selector. Got: {}", css);
    }
}
