use rbx_types::Ref;

use crate::class::{AsClass, Class};

#[derive(Debug)]
pub struct InstanceBuilder<C> {
    referent: Ref,
    children: Vec<InstanceBuilder<Class>>,
    class: C,
}

impl<C> InstanceBuilder<C> {
    /// Create a new `InstanceBuilder` with the given Class.
    pub fn new(class: C) -> Self {
        InstanceBuilder {
            referent: Ref::new(),
            children: Vec::new(),
            class,
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

    /// Change the referent of the `InstanceBuilder`.
    pub fn set_referent<R: Into<Ref>>(&mut self, referent: R) {
        self.referent = referent.into();
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn with_child<ChildClass>(mut self, child: InstanceBuilder<ChildClass>) -> Self
    where
        ChildClass: Into<Class>,
    {
        self.children.push(child.into_class());
        self
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn add_child<ChildClass>(&mut self, child: InstanceBuilder<ChildClass>)
    where
        ChildClass: Into<Class>,
    {
        self.children.push(child.into_class());
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn with_children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = InstanceBuilder<Class>>,
    {
        self.children.extend(children);
        self
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn add_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item = InstanceBuilder<Class>>,
    {
        self.children.extend(children);
    }

    pub(crate) fn children(&self) -> &[InstanceBuilder<Class>] {
        &self.children
    }

    pub(crate) fn into_class(self) -> InstanceBuilder<Class>
    where
        C: Into<Class>,
    {
        InstanceBuilder {
            referent: self.referent,
            children: self.children,
            class: self.class.into(),
        }
    }
}

impl<C: Default> Default for InstanceBuilder<C> {
    fn default() -> Self {
        Self {
            referent: Ref::new(),
            children: Vec::new(),
            class: C::default(),
        }
    }
}
impl<C> core::ops::Deref for InstanceBuilder<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.class
    }
}
impl<C> core::ops::DerefMut for InstanceBuilder<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.class
    }
}

#[derive(Debug)]
pub struct Instance {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,
    class: Class,
}

impl Instance {
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

    /// Cast the instance's class as a superclass.  For example, if the
    /// Instance's Class is Part, `instance.as_class::<BasePart>()` gives
    /// `Some(&BasePart)`.
    pub fn as_class<C>(&self) -> Option<&C>
    where
        Class: AsClass<C>,
    {
        self.class.as_class()
    }

    /// Cast the instance's class as a superclass.  For example, if the
    /// Instance's Class is Part, `instance.as_class_mut::<BasePart>()` gives
    /// `Some(&mut BasePart)`.
    pub fn as_class_mut<C>(&mut self) -> Option<&mut C>
    where
        Class: AsClass<C>,
    {
        self.class.as_class_mut()
    }

    pub(crate) fn children_mut(&mut self) -> &mut Vec<Ref> {
        &mut self.children
    }
    pub(crate) fn from_builder<C: Into<Class>>(
        parent: Ref,
        builder: InstanceBuilder<C>,
    ) -> (Self, Vec<InstanceBuilder<Class>>) {
        (
            Instance {
                referent: builder.referent,
                children: Vec::with_capacity(builder.children.len()),
                parent,
                class: builder.class.into(),
            },
            builder.children,
        )
    }
}
