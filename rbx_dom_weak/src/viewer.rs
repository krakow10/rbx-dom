use std::{collections::HashMap, fmt::Write};

use crate::{
    types::{Ref, Variant},
    WeakDom,
};
use serde::{Deserialize, Serialize};
use ustr::Ustr;

/// Contains state for viewing and redacting nondeterministic portions of
/// WeakDom objects, making them suitable for usage in snapshot tests.
///
/// `DomViewer` can be held onto and used with a DOM multiple times. IDs will
/// persist when viewing the same instance multiple times, and should stay the
/// same across multiple runs of a test.
pub struct DomViewer {
    referent_to_id: HashMap<Ref, String>,
    next_id: usize,
}

impl DomViewer {
    /// Construct a new `DomViewer` with no interned referents.
    pub fn new() -> Self {
        Self {
            referent_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    /// View the given `WeakDom`, creating a `ViewedInstance` object that can be
    /// used in a snapshot test.
    pub fn view(&mut self, dom: &WeakDom) -> ViewedInstance {
        let root_referent = dom.root_ref();
        self.populate_referent_map(dom, root_referent);
        self.view_instance(dom, root_referent)
    }

    /// View the children of the root instance of the given `WeakDom`, returning
    /// them as a `Vec<ViewedInstance>`.
    pub fn view_children(&mut self, dom: &WeakDom) -> Vec<ViewedInstance> {
        let root_instance = dom.root();
        let children = root_instance.children();

        for &referent in children {
            self.populate_referent_map(dom, referent);
        }

        children
            .iter()
            .map(|&referent| self.view_instance(dom, referent))
            .collect()
    }

    fn populate_referent_map(&mut self, dom: &WeakDom, referent: Ref) {
        let next_id = &mut self.next_id;
        self.referent_to_id.entry(referent).or_insert_with(|| {
            let name = format!("referent-{next_id}");
            *next_id += 1;
            name
        });

        let instance = dom.get_by_ref(referent).unwrap();
        for referent in instance.children() {
            self.populate_referent_map(dom, *referent);
        }
    }

    fn view_instance(&self, dom: &WeakDom, referent: Ref) -> ViewedInstance {
        let instance = dom.get_by_ref(referent).unwrap();

        let children = instance
            .children()
            .iter()
            .copied()
            .map(|referent| self.view_instance(dom, referent))
            .collect();

        let properties = instance
            .properties
            .iter()
            .map(|(key, value)| {
                let new_value = match value {
                    Variant::Ref(referent) => {
                        if referent.is_some() {
                            let referent_str = self
                                .referent_to_id
                                .get(referent)
                                .cloned()
                                .unwrap_or_else(|| "[unknown ID]".to_owned());

                            ViewedValue::Ref(referent_str)
                        } else {
                            ViewedValue::Ref("null".to_owned())
                        }
                    }
                    Variant::SharedString(shared_string) => {
                        let hash = shared_string.hash();
                        let mut hash_hex = String::with_capacity(hash.as_bytes().len() * 2);

                        for byte in hash.as_bytes() {
                            write!(hash_hex, "{byte:02x}").unwrap();
                        }
                        ViewedValue::SharedString {
                            len: shared_string.data().len(),
                            hash: hash_hex,
                        }
                    }
                    Variant::NetAssetRef(net) => {
                        let hash = net.hash();
                        let mut hash_hex = String::with_capacity(hash.as_bytes().len() * 2);

                        for byte in hash.as_bytes() {
                            write!(hash_hex, "{byte:02x}").unwrap();
                        }
                        ViewedValue::NetAssetRef {
                            len: net.data().len(),
                            hash: hash_hex,
                        }
                    }
                    other => ViewedValue::Other(other.clone()),
                };

                (*key, new_value)
            })
            .collect();

        let properties = SortedVecMap::new(properties);

        ViewedInstance {
            referent: self.referent_to_id.get(&referent).unwrap().clone(),
            name: instance.name.clone(),
            class: instance.class,
            properties,
            children,
        }
    }
}

impl Default for DomViewer {
    fn default() -> Self {
        Self::new()
    }
}

/// A pre-sorted list of key-value pairs which
/// serializes as a map rather than a list of tuples.
#[derive(Debug, Clone)]
struct SortedVecMap<K, V> {
    inner: Vec<(K, V)>,
}
impl<K: Ord, V> SortedVecMap<K, V> {
    fn new(mut inner: Vec<(K, V)>) -> Self {
        inner.sort_unstable_by(|(k0, _), (k1, _)| k0.cmp(k1));
        Self { inner }
    }
}
impl<K: Serialize, V: Serialize> Serialize for SortedVecMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.inner.len()))?;
        for (key, value) in &self.inner {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl<'de, K: Ord + Deserialize<'de>, V: Deserialize<'de>> Deserialize<'de> for SortedVecMap<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SortedVecMapVisitor<K, V>(core::marker::PhantomData<(K, V)>);
        impl<'de, K: Ord + Deserialize<'de>, V: Deserialize<'de>> serde::de::Visitor<'de>
            for SortedVecMapVisitor<K, V>
        {
            type Value = SortedVecMap<K, V>;
            fn expecting(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(out, "a map of key-value pairs")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut inner = Vec::with_capacity(map.size_hint().unwrap_or(0));
                while let Some((key, value)) = map.next_value()? {
                    inner.push((key, value));
                }
                Ok(SortedVecMap::new(inner))
            }
        }
        deserializer.deserialize_map(SortedVecMapVisitor(core::marker::PhantomData))
    }
}

/// A transformed view into a `WeakDom` or `Instance` that has been redacted and
/// transformed to be more readable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewedInstance {
    referent: String,
    name: String,
    class: Ustr,
    properties: SortedVecMap<Ustr, ViewedValue>,
    children: Vec<ViewedInstance>,
}

/// Wrapper around Variant with refs replaced to be redacted, stable versions of
/// their original IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum ViewedValue {
    Ref(String),
    SharedString { len: usize, hash: String },
    NetAssetRef { len: usize, hash: String },
    Other(Variant),
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::types::SharedString;
    use crate::InstanceBuilder;

    #[test]
    fn redact_single() {
        let dom = WeakDom::new(InstanceBuilder::new("Folder").with_name("Root"));

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }

    #[test]
    fn redact_multi() {
        let dom = WeakDom::new(
            InstanceBuilder::new("Folder")
                .with_name("Root")
                .with_children(
                    (0..4).map(|i| InstanceBuilder::new("Folder").with_name(format!("Child {i}"))),
                ),
        );

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }

    #[test]
    fn redact_values() {
        let root = InstanceBuilder::new("ObjectValue").with_name("Root");
        let root_ref = root.referent;
        let root = root.with_property("Value", root_ref);

        let dom = WeakDom::new(root);

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }

    #[test]
    fn abbreviate_shared_string() {
        let shared_string = SharedString::new("foo".into());

        let root = InstanceBuilder::new("UnionOperation")
            .with_name("Root")
            .with_property("PhysicalConfigData", shared_string);

        let dom = WeakDom::new(root);

        insta::assert_yaml_snapshot!(DomViewer::new().view(&dom));
    }
}
