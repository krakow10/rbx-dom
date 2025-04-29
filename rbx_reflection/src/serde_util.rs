use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    hash::Hash,
};

use hash_str::HashStrMap;
use serde::{Serialize, Serializer};

pub(crate) fn ordered_map<S, V>(value: &HashStrMap<'_, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

pub(crate) fn ordered_set<S, V>(value: &HashSet<V>, serializer: S) -> Result<S::Ok, S::Error>
where
    V: Hash + Ord + Serialize,
    S: Serializer,
{
    let ordered: BTreeSet<_> = value.iter().collect();
    ordered.serialize(serializer)
}
