use hash_str::{hstr, HashStr, HashStrMap};
use rbx_types::{Ref, Variant};

/**
Represents an instance that can be turned into a new
[`WeakDom`][crate::WeakDom], or inserted into an existing one.

## Examples
Instances have the given ClassName and Name and no properties by default.

```
use rbx_dom_weak::{hstr, InstanceBuilder, WeakDom};

let data_model = InstanceBuilder::new(hstr!("DataModel"));
let dom = WeakDom::new(data_model);
```

Properties and children can be added to the builder.

```
use rbx_dom_weak::{hstr, InstanceBuilder, WeakDom};
use rbx_dom_weak::types::Color3;

let data_model = InstanceBuilder::new(hstr!("DataModel"))
    .with_child(InstanceBuilder::new(hstr!("Workspace"))
        .with_property(hstr!("FilteringEnabled"), true))
    .with_child(InstanceBuilder::new(hstr!("Lighting"))
        .with_property(hstr!("Ambient"), Color3::new(1.0, 0.0, 0.0)));

let dom = WeakDom::new(data_model);
```
*/
#[derive(Debug)]
pub struct InstanceBuilder<'a> {
    pub(crate) referent: Ref,
    pub(crate) name: String,
    pub(crate) class: &'a HashStr,
    pub(crate) properties: Vec<(&'a HashStr, Variant)>,
    pub(crate) children: Vec<InstanceBuilder<'a>>,
}

impl<'a> InstanceBuilder<'a> {
    /// Create a new `InstanceBuilder` with the given ClassName. This is also
    /// used as the instance's Name, unless overwritten later.
    pub fn new(class: &'a HashStr) -> Self {
        let name = class.to_string();

        InstanceBuilder {
            referent: Ref::new(),
            name,
            class,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Create a new `InstanceBuilder` with the given ClassName and with a
    /// property table with at least enough space for the given capacity.
    pub fn with_property_capacity(class: &'a HashStr, capacity: usize) -> Self {
        let name = class.to_string();

        InstanceBuilder {
            referent: Ref::new(),
            name,
            class,
            properties: Vec::with_capacity(capacity),
            children: Vec::new(),
        }
    }

    /// Create a new `InstanceBuilder` with all values set to empty.
    pub fn empty() -> Self {
        InstanceBuilder {
            referent: Ref::new(),
            name: String::new(),
            class: hstr!(""),
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Return the referent of the instance that the `InstanceBuilder` refers to.
    pub fn referent(&self) -> Ref {
        self.referent
    }

    /// Change the referent of the `InstanceBuilder`.
    pub fn with_referent<R: Into<Ref>>(self, referent: R) -> Self {
        Self {
            referent: referent.into(),
            ..self
        }
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn with_name<S: Into<String>>(self, name: S) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Change the class of the `InstanceBuilder`.
    pub fn with_class(self, class: &'a HashStr) -> Self {
        Self { class, ..self }
    }

    /// Change the class of the `InstanceBuilder`.
    pub fn set_class(&mut self, class: &'a HashStr) {
        self.class = class;
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn with_property<V: Into<Variant>>(mut self, key: &'a HashStr, value: V) -> Self {
        self.properties.push((key, value.into()));
        self
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn add_property<V: Into<Variant>>(&mut self, key: &'a HashStr, value: V) {
        self.properties.push((key.into(), value.into()));
    }

    /// Check if the `InstanceBuilder` already has a property with the given key.
    pub fn has_property(&self, key: &'a HashStr) -> bool {
        self.properties.iter().any(|(k, _)| *k == key)
    }

    /// Add multiple properties to the `InstanceBuilder` at once.
    pub fn with_properties<V, I>(mut self, props: I) -> Self
    where
        V: Into<Variant>,
        I: IntoIterator<Item = (&'a HashStr, V)>,
    {
        let props = props.into_iter().map(|(k, v)| (k.into(), v.into()));
        self.properties.extend(props);

        self
    }

    /// Add multiple properties to the `InstanceBuilder` at once.
    pub fn add_properties<V, I>(&mut self, props: I)
    where
        V: Into<Variant>,
        I: IntoIterator<Item = (&'a HashStr, V)>,
    {
        let props = props.into_iter().map(|(k, v)| (k.into(), v.into()));
        self.properties.extend(props);
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn with_child(mut self, child: InstanceBuilder<'a>) -> Self {
        self.children.push(child);
        self
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn add_child(&mut self, child: InstanceBuilder<'a>) {
        self.children.push(child);
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn with_children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = InstanceBuilder<'a>>,
    {
        self.children.extend(children);
        self
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn add_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item = InstanceBuilder<'a>>,
    {
        self.children.extend(children);
    }
}

/// An instance contained inside of a [`WeakDom`][crate::WeakDom].
///
/// Operations that could affect other instances contained in the
/// [`WeakDom`][crate::WeakDom] cannot be performed on an `Instance` correctly.
#[derive(Debug)]
pub struct Instance<'a> {
    pub(crate) referent: Ref,
    pub(crate) children: Vec<Ref>,
    pub(crate) parent: Ref,

    /// The instance's name, corresponding to the `Name` property.
    pub name: String,

    /// The instance's class, corresponding to the `ClassName` property.
    pub class: &'a HashStr,

    /// Any properties stored on the object that are not `Name` or `ClassName`.
    pub properties: HashStrMap<'a, Variant>,
}

impl Instance<'_> {
    /// Returns this instance's referent. It will always be non-null.
    #[inline]
    pub fn referent(&self) -> Ref {
        self.referent
    }

    /// Returns a list of the referents corresponding to the instance's
    /// children. All referents returned will be non-null and point to valid
    /// instances in the same [`WeakDom`][crate::WeakDom].
    #[inline]
    pub fn children(&self) -> &[Ref] {
        &self.children
    }

    /// Returns the referent corresponding to this instance's parent. This
    /// referent will either point to an instance in the same
    /// [`WeakDom`][crate::WeakDom] or be null.
    #[inline]
    pub fn parent(&self) -> Ref {
        self.parent
    }
}
