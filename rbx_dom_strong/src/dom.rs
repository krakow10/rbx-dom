use ahash::{AHashMap, AHashSet};
use rbx_types::{Ref, UniqueId};

use crate::instance::StrongInstance;

/// Represents a DOM containing one or more Roblox instances.
#[derive(Debug)]
pub struct StrongDom {
    instances: AHashMap<Ref, StrongInstance>,
    root_ref: Ref,
    unique_ids: AHashSet<UniqueId>,
}

impl StrongDom {
    /// Returns a reference to an instance by referent, or `None` if it is not
    /// found.
    pub fn get_by_ref<'a, IntoClass>(&'a self, referent: Ref) -> Option<IntoClass>
    where
        &'a StrongInstance: Into<IntoClass>,
    {
        self.instances.get(&referent).map(Into::into)
    }

    /// Returns a _mutable_ reference to an instance by referent, or `None` if
    /// it is not found.
    pub fn get_by_ref_mut<'a, IntoClass>(&'a mut self, referent: Ref) -> Option<IntoClass>
    where
        &'a mut StrongInstance: Into<IntoClass>,
    {
        self.instances.get_mut(&referent).map(Into::into)
    }
}

impl Default for StrongDom {
    fn default() -> StrongDom {
        StrongDom {
            instances: AHashMap::new(),
            root_ref: Ref::none(),
            unique_ids: AHashSet::new(),
        }
    }
}
