use super::*;

#[doc=include_str!("readme.md")]
#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindContainer {}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "container",)
    }
}
/// .container {
//     width: 100%
// }
//

impl TailwindInstance for TailwindContainer {
    fn inlineable(&self) -> bool {
        false
    }
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "width" => "100%"
        }
    }
    fn additional(&self, _: &TailwindBuilder) -> String {
        r#"
@media (min-width: 640px){.container{max-width:640px}}
@media (min-width: 720px){.container{max-width:720px}}
@media (min-width: 768px){.container{max-width:768px}}
@media (min-width: 1024px){.container{max-width:1024px}}
@media (min-width: 1280px){.container{max-width:1280px}}
@media (min-width: 1536px){.container{max-width:1536px}}
"#
        .to_string()
    }
}

/// Container query setup utilities for Tailwind CSS v3/v4.
/// 
/// These utilities set up containers for use with container queries.
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerQuerySetup {
    /// Sets container-type: inline-size (default @container)
    InlineSize { name: Option<String> },
    
    /// Sets container-type: normal (@container-normal)
    Normal { name: Option<String> },
    
    /// Sets container-type: size (@container-size)
    Size { name: Option<String> },
}

impl ContainerQuerySetup {
    /// Creates an inline-size container (the default @container).
    pub fn inline_size(name: Option<String>) -> Self {
        Self::InlineSize { name }
    }
    
    /// Creates a normal container.
    pub fn normal(name: Option<String>) -> Self {
        Self::Normal { name }
    }
    
    /// Creates a size container (both width and height).
    pub fn size(name: Option<String>) -> Self {
        Self::Size { name }
    }
}

impl Display for ContainerQuerySetup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InlineSize { name: None } => write!(f, "@container"),
            Self::InlineSize { name: Some(n) } => write!(f, "@container/{}", n),
            Self::Normal { name: None } => write!(f, "@container-normal"),
            Self::Normal { name: Some(n) } => write!(f, "@container-normal/{}", n),
            Self::Size { name: None } => write!(f, "@container-size"),
            Self::Size { name: Some(n) } => write!(f, "@container-size/{}", n),
        }
    }
}

impl TailwindInstance for ContainerQuerySetup {
    fn inlineable(&self) -> bool {
        true
    }
    
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match self {
            Self::InlineSize { name } => {
                match name {
                    None => css_attributes! {
                        "container-type" => "inline-size"
                    },
                    Some(n) => css_attributes! {
                        "container-type" => "inline-size",
                        "container-name" => n.clone()
                    },
                }
            },
            Self::Normal { name } => {
                match name {
                    None => css_attributes! {
                        "container-type" => "normal"
                    },
                    Some(n) => css_attributes! {
                        "container-type" => "normal",
                        "container-name" => n.clone()
                    },
                }
            },
            Self::Size { name } => {
                match name {
                    None => css_attributes! {
                        "container-type" => "size"
                    },
                    Some(n) => css_attributes! {
                        "container-type" => "size",
                        "container-name" => n.clone()
                    },
                }
            },
        }
    }
    
    fn additional(&self, _: &TailwindBuilder) -> String {
        String::new()
    }
}
