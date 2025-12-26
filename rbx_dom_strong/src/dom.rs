use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};
use rbx_types::{Ref, UniqueId};

use crate::class::Class;
use crate::instance::{Instance, InstanceBuilder};

/// Represents a DOM containing one or more Roblox instances.
#[derive(Debug)]
pub struct StrongDom {
    instances: AHashMap<Ref, Instance>,
    root_ref: Ref,
    unique_ids: AHashSet<UniqueId>,
}

impl StrongDom {
    /// Construct a new `WeakDom` described by the given [`InstanceBuilder`].
    pub fn new<C>(builder: InstanceBuilder<C>) -> Self
    where
        C: Into<Class>,
    {
        let mut dom = Self {
            instances: AHashMap::new(),
            root_ref: builder.referent(),
            unique_ids: AHashSet::new(),
        };

        dom.insert(Ref::none(), builder);
        dom
    }

    /// Reserve at least enough space for `additional` number of instances in
    /// the WeakDom.
    pub fn reserve(&mut self, additional: usize) {
        self.instances.reserve(additional);
    }

    /// Returns the referent of the root instance of the `WeakDom`.
    pub fn root_ref(&self) -> Ref {
        self.root_ref
    }

    /// Returns a reference to the root instance of the `WeakDom`.
    pub fn root(&self) -> &Instance {
        self.instances.get(&self.root_ref).unwrap()
    }

    /// Returns a _mutable_ reference to the root instance of the `WeakDom`.
    pub fn root_mut(&mut self) -> &mut Instance {
        self.instances.get_mut(&self.root_ref).unwrap()
    }

    /// Returns a reference to an instance by referent, or `None` if it is not
    /// found.
    pub fn get_by_ref(&self, referent: Ref) -> Option<&Instance> {
        self.instances.get(&referent)
    }

    /// Returns a _mutable_ reference to an instance by referent, or `None` if
    /// it is not found.
    pub fn get_by_ref_mut(&mut self, referent: Ref) -> Option<&mut Instance> {
        self.instances.get_mut(&referent)
    }

    /// Insert a new instance into the DOM with the given parent. The parent is allowed to
    /// be the none Ref.
    ///
    /// ## Panics
    /// Panics if `parent_ref` is some and does not refer to an instance in the DOM.
    pub fn insert<C>(&mut self, parent_ref: Ref, root_builder: InstanceBuilder<C>) -> Ref
    where
        C: Into<Class>,
    {
        fn insert(
            dom: &mut StrongDom,
            builder: InstanceBuilder<Class>,
            parent: Ref,
            queue: Option<&mut VecDeque<(Ref, InstanceBuilder<Class>)>>,
        ) {
            let referent = builder.referent();
            let (instance, builder_children) = Instance::from_builder(parent, builder);
            dom.inner_insert(referent, instance);

            if parent.is_some() {
                dom.instances
                    .get_mut(&parent)
                    .unwrap_or_else(|| panic!("cannot insert into parent that does not exist"))
                    .children_mut()
                    .push(referent);
            }

            if let Some(queue) = queue {
                for child in builder_children {
                    queue.push_back((referent, child));
                }
            }
        }

        let root_builder = root_builder.into_class();
        let root_referent = root_builder.referent();

        // Fast path: if the builder does not have any children, then we don't have to
        // construct a queue to keep track of descendants for insertion, avoiding a heap
        // allocation.
        if root_builder.children().is_empty() {
            insert(self, root_builder, parent_ref, None);
        } else {
            // Rather than performing this movement recursively, we instead use a
            // queue that we load the children of each `InstanceBuilder` into.
            // Then we can just iter through that.
            let mut queue = VecDeque::with_capacity(1);
            queue.push_back((parent_ref, root_builder));

            while let Some((parent, builder)) = queue.pop_front() {
                insert(self, builder, parent, Some(&mut queue));
            }
        }

        root_referent
    }

    fn inner_insert(&mut self, referent: Ref, mut instance: Instance) {
        // We need to ensure that the value of the Instance.UniqueId property does
        // not collide with another instance. If it does, we must regenerate
        // it. If we *don't* do this, it's possible to use WeakDom::insert to
        // insert UniqueId properties that collide with other instances in the
        // dom, violating the invariant that every UniqueId is unique.
        if let Some(instance) = instance.as_class_mut::<rbx_classes::instances::Instance>() {
            if self.unique_ids.contains(&instance.UniqueId) {
                // We found a collision! We need to replace the UniqueId property with
                // a new value.

                // Unwrap is probably ok. Likely not worth making this method fallible
                // just because the system clock might be out whack, so panicking is fine
                let new_unique_id = UniqueId::now().unwrap();

                self.unique_ids.insert(new_unique_id);
                instance.UniqueId = new_unique_id;
            } else {
                self.unique_ids.insert(instance.UniqueId);
            };
        }
        self.instances.insert(referent, instance);
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
