use ahash::{AHashMap, AHashSet};
use rbx_types::{Ref, UniqueId};

use crate::instance::InstanceBuilderInner;
use crate::instance::InstanceInner;
use crate::instance::StrongInstance;

/// Represents a DOM containing one or more Roblox instances.
#[derive(Debug)]
pub struct StrongDom {
    instances: AHashMap<Ref, StrongInstance<InstanceInner>>,
    root_ref: Ref,
    unique_ids: AHashSet<UniqueId>,
}

impl StrongDom {
    /// Construct a new `WeakDom` described by the given [`InstanceBuilder`].
    pub fn new<B>(builder: B) -> Self
    where
        B: Into<StrongInstance<InstanceBuilderInner>>,
    {
        let builder = builder.into();
        let mut dom = Self {
            instances: AHashMap::new(),
            root_ref: builder.referent(),
            unique_ids: AHashSet::new(),
        };

        dom.insert(Ref::none(), builder);
        dom
    }

    /// Returns a reference to an instance by referent, or `None` if it is not
    /// found.
    pub fn get_by_ref(&self, referent: Ref) -> Option<&StrongInstance<InstanceInner>> {
        self.instances.get(&referent)
    }

    /// Returns a _mutable_ reference to an instance by referent, or `None` if
    /// it is not found.
    pub fn get_by_ref_mut(&mut self, referent: Ref) -> Option<&mut StrongInstance<InstanceInner>> {
        self.instances.get_mut(&referent)
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
