use super::*;

pub(crate) mod inset_ring_color;
pub(crate) mod inset_ring_width;
pub(crate) mod ring_color;
pub(crate) mod ring_inset;
pub(crate) mod ring_offset_color;
pub(crate) mod ring_offset_width;
pub(crate) mod ring_width;

#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindRing {}

impl TailwindRing {
    pub fn adapt(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        Self::adapt_with_opacity(str, arbitrary, None)
    }
    
    pub fn adapt_with_opacity(str: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // ring-width utilities (ring, ring-0, ring-1, ring-2, ring-4, ring-8)
            [] => TailwindRingWidth::parse(&[], arbitrary)?.boxed(),
            ["0"] | ["1"] | ["2"] | ["4"] | ["8"] => TailwindRingWidth::parse(str, arbitrary)?.boxed(),
            
            // ring-inset utility
            ["inset"] => TailwindRingInset::default().boxed(),
            
            // ring-offset utilities
            ["offset", rest @ ..] => {
                match rest {
                    // ring-offset-width (ring-offset-0, ring-offset-1, ring-offset-2, ring-offset-4, ring-offset-8)
                    ["0"] | ["1"] | ["2"] | ["4"] | ["8"] => TailwindRingOffsetWidth::parse(rest, arbitrary)?.boxed(),
                    // ring-offset-color (ring-offset-red-500, etc.)
                    _ => TailwindRingOffsetColor::parse(rest, arbitrary)?.boxed(),
                }
            },
            
            // Arbitrary values for ring width are handled with the empty pattern above when arbitrary.is_some()
            
            // ring-color utilities (ring-red-500, ring-blue-600, etc.)
            _ => TailwindRingColor::parse_with_opacity(str, arbitrary, opacity)?.boxed(),
        };
        Ok(out)
    }
}
