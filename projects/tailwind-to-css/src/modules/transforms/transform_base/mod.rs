use crate::{css_attributes, CssAttributes, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::fmt::{Debug, Display, Formatter};

/// Transform base utility
/// 
/// Supports:
/// - transform (enables transforms with CSS variables)
/// - transform-none (disables transforms)
/// - transform-gpu (forces hardware acceleration)
/// - transform-cpu (forces CPU processing)
#[derive(Clone, Debug)]
pub struct TailwindTransformBase {
    kind: TransformBaseKind,
}

#[derive(Clone, Debug)]
enum TransformBaseKind {
    Transform,
    None,
    Gpu,
    Cpu,
}

impl Display for TailwindTransformBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TransformBaseKind::Transform => write!(f, "transform"),
            TransformBaseKind::None => write!(f, "transform-none"),
            TransformBaseKind::Gpu => write!(f, "transform-gpu"),
            TransformBaseKind::Cpu => write!(f, "transform-cpu"),
        }
    }
}

impl TailwindInstance for TailwindTransformBase {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match self.kind {
            TransformBaseKind::Transform => {
                // The transform base utility sets up the transform with CSS variables
                // that are used by translate, rotate, scale, and skew utilities
                css_attributes! {
                    "--tw-translate-x" => "0",
                    "--tw-translate-y" => "0",
                    "--tw-rotate" => "0",
                    "--tw-skew-x" => "0",
                    "--tw-skew-y" => "0",
                    "--tw-scale-x" => "1",
                    "--tw-scale-y" => "1",
                    "transform" => "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
                }
            }
            TransformBaseKind::None => {
                css_attributes! {
                    "transform" => "none",
                }
            }
            TransformBaseKind::Gpu => {
                css_attributes! {
                    "--tw-translate-x" => "0",
                    "--tw-translate-y" => "0",
                    "--tw-rotate" => "0",
                    "--tw-skew-x" => "0",
                    "--tw-skew-y" => "0",
                    "--tw-scale-x" => "1",
                    "--tw-scale-y" => "1",
                    "transform" => "translate3d(var(--tw-translate-x), var(--tw-translate-y), 0) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
                    "will-change" => "transform",
                }
            }
            TransformBaseKind::Cpu => {
                css_attributes! {
                    "--tw-translate-x" => "0",
                    "--tw-translate-y" => "0",
                    "--tw-rotate" => "0",
                    "--tw-skew-x" => "0",
                    "--tw-skew-y" => "0",
                    "--tw-scale-x" => "1",
                    "--tw-scale-y" => "1",
                    "transform" => "translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))",
                    "will-change" => "auto",
                }
            }
        }
    }
}

impl TailwindTransformBase {
    pub fn parse(input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match input {
            [] => TransformBaseKind::Transform,
            ["none"] => TransformBaseKind::None,
            ["gpu"] => TransformBaseKind::Gpu,
            ["cpu"] => TransformBaseKind::Cpu,
            _ => return Err(().into()),
        };
        Ok(Self { kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_base() {
        let arbitrary = TailwindArbitrary::from("dummy");
        let transform = TailwindTransformBase::parse(&[], &arbitrary).unwrap();
        assert_eq!(transform.to_string(), "transform");
        
        // The transform class should have the CSS variable setup
        assert!(matches!(transform.kind, TransformBaseKind::Transform));
    }

    #[test]
    fn test_transform_none() {
        let arbitrary = TailwindArbitrary::from("dummy");
        let transform = TailwindTransformBase::parse(&["none"], &arbitrary).unwrap();
        assert_eq!(transform.to_string(), "transform-none");
        assert!(matches!(transform.kind, TransformBaseKind::None));
    }

    #[test]
    fn test_transform_gpu() {
        let arbitrary = TailwindArbitrary::from("dummy");
        let transform = TailwindTransformBase::parse(&["gpu"], &arbitrary).unwrap();
        assert_eq!(transform.to_string(), "transform-gpu");
        assert!(matches!(transform.kind, TransformBaseKind::Gpu));
    }

    #[test]
    fn test_transform_cpu() {
        let arbitrary = TailwindArbitrary::from("dummy");
        let transform = TailwindTransformBase::parse(&["cpu"], &arbitrary).unwrap();
        assert_eq!(transform.to_string(), "transform-cpu");
        assert!(matches!(transform.kind, TransformBaseKind::Cpu));
    }

    #[test]
    fn test_invalid_transform() {
        let arbitrary = TailwindArbitrary::from("dummy");
        assert!(TailwindTransformBase::parse(&["invalid"], &arbitrary).is_err());
    }
}