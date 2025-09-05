use super::*;

pub(crate) mod text_align;
pub(crate) mod text_color;
pub(crate) mod text_overflow;
pub(crate) mod text_transform;
pub(crate) mod text_wrap;

pub fn text_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/text-align
        [s @ ("left" | "center" | "right" | "justify" | "start" | "end")] => TailwindTextAlignment::from(*s).boxed(),
        ["align", rest @ ..] => TailwindTextAlignment::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/text-overflow
        [s @ ("ellipsis" | "clip")] => TailwindTextAlignment::from(*s).boxed(),
        ["overflow", rest @ ..] => TailwindTextAlignment::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/text-transform
        ["transform", rest @ ..] => TailwindTextTransform::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/text-wrap
        ["wrap"] => text_wrap::TailwindTextWrap::from("wrap").boxed(),
        ["nowrap"] => text_wrap::TailwindTextWrap::from("nowrap").boxed(),
        ["balance"] => text_wrap::TailwindTextWrap::from("balance").boxed(),
        ["pretty"] => text_wrap::TailwindTextWrap::from("pretty").boxed(),
        // https://tailwindcss.com/docs/font-size  Built-in with potential line-height
        [size @ ("xs" | "sm" | "base" | "md" | "lg" | "xl" | "2xl" | "3xl" | "4xl" | "5xl" | "6xl" | "7xl" | "8xl" | "9xl")] => {
            // Check if opacity is actually a line-height value (combined font syntax)
            if let Some(line_height) = opacity {
                TailwindFontSizeWithLineHeight::parse(size, line_height, arbitrary)?.boxed()
            } else {
                TailwindFontSize::parse(pattern, arbitrary)?.boxed()
            }
        },
        // https://tailwindcss.com/docs/font-size  Arbitrary with potential line-height
        [] => {
            // Check if this looks like a color value first
            let value = arbitrary.as_str();
            if value.starts_with("oklch(") || value.starts_with("lch(") || 
               value.starts_with("lab(") || value.starts_with("oklab(") ||
               value.starts_with("hwb(") || value.starts_with("color(") ||
               value.starts_with("#") || value.starts_with("rgb(") || 
               value.starts_with("rgba(") || value.starts_with("hsl(") || 
               value.starts_with("hsla(") {
                // This is a color, not a font size
                let color = TailwindColor::parse_with_opacity(pattern, arbitrary, opacity)?;
                TailwindTextColor::from(color).boxed()
            } else if let Some(line_height) = opacity {
                // For arbitrary font size with line height
                TailwindFontSizeWithLineHeight::parse_arbitrary(arbitrary, line_height)?.boxed()
            } else {
                // Try as font size first, fall back to color if it fails
                match TailwindFontSize::parse(pattern, arbitrary) {
                    Ok(font_size) => font_size.boxed(),
                    Err(_) => {
                        // Fall back to color
                        let color = TailwindColor::parse_with_opacity(pattern, arbitrary, opacity)?;
                        TailwindTextColor::from(color).boxed()
                    }
                }
            }
        },
        // https://tailwindcss.com/docs/text-color
        _ => {
            let color = TailwindColor::parse_with_opacity(pattern, arbitrary, opacity)?;
            TailwindTextColor::from(color).boxed()
        },
    };
    Ok(out)
}
