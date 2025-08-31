use super::*;

pub use self::{
    justify_content::TailwindJustifyContent, justify_item::TailwindJustifyItems, justify_self::TailwindJustifySelf,
};

mod justify_content;
mod justify_item;
mod justify_self;

pub(crate) fn justify_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match str {
        // https://tailwindcss.com/docs/justify-items
        ["items", rest @ ..] => TailwindJustifyItems::parse(rest, arbitrary)?.boxed(),
        // https://tailwindcss.com/docs/justify-self
        ["self", rest @ ..] => TailwindJustifySelf::parse(rest, arbitrary)?.boxed(),
        // Handle already-transformed justify-content-* classes
        // When "justify-content-between" is parsed, it becomes ["justify", "content", "between"]
        // and this function receives ["content", "between"]
        ["content", rest @ ..] => {
            // This is already a transformed class, return an error to keep it as-is
            return Err(crate::TailwindError::syntax_error(format!("Already transformed: justify-content-{}", rest.join("-"))));
        },
        // https://tailwindcss.com/docs/justify-content
        _ => TailwindJustifyContent::parse(str, arbitrary)?.boxed(),
    };
    Ok(out)
}
