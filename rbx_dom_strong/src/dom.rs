use ahash::{AHashMap, AHashSet};
use rbx_types::{Ref, UniqueId};

use rbx_types_strong::instances::StrongInstance;

/// Represents a DOM containing one or more Roblox instances.
#[derive(Debug)]
pub struct StrongDom {
    instances: AHashMap<Ref, StrongInstance>,
    root_ref: Ref,
    unique_ids: AHashSet<UniqueId>,
}
