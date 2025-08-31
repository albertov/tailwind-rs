use super::*;
use indexmap::IndexSet;

mod methods;
mod traits;

/// The `css-global-attribute` system.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ImportantSet {
    important: bool,
    set: IndexSet<String>,
}

/// The `css-global-attribute` system.
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImportantMap {
    map: BTreeMap<String, (bool, String)>,
}
