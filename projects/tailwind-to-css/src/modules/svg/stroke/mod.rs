use crate::syntax_error;

use super::*;

pub(crate) mod stroke_color;
pub(crate) mod stroke_width;
pub(crate) mod stroke_dasharray;
pub(crate) mod stroke_dashoffset;
pub(crate) mod stroke_opacity;
pub(crate) mod stroke_linecap;
pub(crate) mod stroke_linejoin;

#[derive(Clone, Debug)]
pub struct TailwindStroke {}

impl TailwindStroke {
    pub fn parse(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let color = |color| TailwindStrokeColor::from(color).boxed();
        let out = match str {
            // Stroke dasharray: stroke-dash-{n}
            ["dash", rest @ ..] => TailwindStrokeDasharray::parse(rest, arbitrary)?.boxed(),
            // Stroke dashoffset: stroke-offset-{n}
            ["offset", rest @ ..] => TailwindStrokeDashoffset::parse(rest, arbitrary)?.boxed(),
            // Stroke opacity: stroke-opacity-{n}
            ["opacity", rest @ ..] => TailwindStrokeOpacity::parse(rest, arbitrary)?.boxed(),
            // Stroke linecap: stroke-cap-{value}
            ["cap", rest @ ..] => TailwindStrokeLinecap::parse(rest, arbitrary)?.boxed(),
            // Stroke linejoin: stroke-join-{value}
            ["join", rest @ ..] => TailwindStrokeLinejoin::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            ["color"] => color(TailwindColor::parse_arbitrary(arbitrary)?),
            ["color", rest] => {
                let a = TailwindArbitrary::from(*rest);
                color(TailwindColor::parse_arbitrary(&a)?)
            },
            // https://tailwindcss.com/docs/text-decoration-color
            [theme, weight] => color(TailwindColor::parse_themed(theme, weight)?),
            // https://tailwindcss.com/docs/text-decoration-thickness
            [n] => maybe_width(n)?,
            _ => return syntax_error!("Unknown decoration instructions: {}", str.join("-")),
        };
        Ok(out)
    }
}

fn maybe_width(s: &str) -> Result<Box<dyn TailwindInstance>> {
    Ok(TailwindStrokeWidth::try_new(s)?.boxed())
}
