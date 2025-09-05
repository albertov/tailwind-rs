use crate::*;
use std::fmt::{Display, Formatter};

/// Marker utility for group-based variant targeting.
/// 
/// The `group` class is a marker that enables descendant elements to apply styles
/// based on the parent's state. It generates an empty CSS rule `.group {}` which
/// serves as an anchor for group-based pseudo-class selectors.
#[derive(Debug, Clone)]
pub struct TailwindGroupMarker;

impl Display for TailwindGroupMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "group")
    }
}

impl TailwindInstance for TailwindGroupMarker {
    fn attributes(&self, _ctx: &TailwindBuilder) -> CssAttributes {
        // Group markers don't have any CSS properties - they're just selector anchors
        css_attributes! {}
    }
    
    fn inlineable(&self) -> bool {
        // Markers cannot be inlined - they need to be in the CSS
        false
    }
}

/// Marker utility for peer-based variant targeting.
/// 
/// The `peer` class is a marker that enables sibling elements to apply styles
/// based on a peer element's state. It generates an empty CSS rule `.peer {}`
/// which serves as an anchor for peer-based pseudo-class selectors.
#[derive(Debug, Clone)]
pub struct TailwindPeerMarker;

impl Display for TailwindPeerMarker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "peer")
    }
}

impl TailwindInstance for TailwindPeerMarker {
    fn attributes(&self, _ctx: &TailwindBuilder) -> CssAttributes {
        // Peer markers don't have any CSS properties - they're just selector anchors
        css_attributes! {}
    }
    
    fn inlineable(&self) -> bool {
        // Markers cannot be inlined - they need to be in the CSS
        false
    }
}

/// Parser for marker utilities
#[allow(dead_code)]
pub fn parse_marker(class: &str) -> Option<Box<dyn TailwindInstance>> {
    match class {
        "group" => Some(TailwindGroupMarker.boxed()),
        "peer" => Some(TailwindPeerMarker.boxed()),
        _ => None,
    }
}