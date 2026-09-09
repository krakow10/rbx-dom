use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Serialize, Serializer};
use vecmap::{VecMap, VecSet};

pub(crate) fn ordered_map<S, K, V>(value: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Hash + Ord + Serialize,
    V: Serialize,
    S: Serializer,
{
    let values: Vec<_> = value.iter().collect();
    // SAFETY: HashMap has no duplicates
    let mut ordered = unsafe { VecMap::from_vec_unchecked(values) };
    ordered.sort_unstable_keys();
    ordered.serialize(serializer)
}

pub(crate) fn ordered_set<S, V>(value: &HashSet<V>, serializer: S) -> Result<S::Ok, S::Error>
where
    V: Hash + Ord + Serialize,
    S: Serializer,
{
    let values: Vec<_> = value.iter().collect();
    // SAFETY: HashSet has no duplicates
    let mut ordered = unsafe { VecSet::from_vec_unchecked(values) };
    ordered.sort_unstable();
    ordered.serialize(serializer)
}
