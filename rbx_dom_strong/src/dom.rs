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
    pub fn get_by_ref(&self, referent: Ref) -> Option<&StrongInstance> {
        self.instances.get(&referent)
    }

    /// Returns a _mutable_ reference to an instance by referent, or `None` if
    /// it is not found.
    pub fn get_by_ref_mut(&mut self, referent: Ref) -> Option<&mut StrongInstance> {
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
