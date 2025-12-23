use rbx_types::Ref;

#[derive(Debug)]
pub struct Instance<C> {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,

    class: C,
}

impl<C> Instance<C> {
    pub fn with_class(class: C) -> Self {
        Self {
            referent: Ref::new(),
            children: Vec::new(),
            parent: Ref::none(),
            class,
        }
    }
}

impl<C: Default> Default for Instance<C> {
    fn default() -> Self {
        Self {
            referent: Ref::new(),
            children: Vec::new(),
            parent: Ref::none(),
            class: C::default(),
        }
    }
}

impl<C> core::ops::Deref for Instance<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.class
    }
}
impl<C> core::ops::DerefMut for Instance<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.class
    }
}

macro_rules! impl_strong_instance {
    ($($class:ident),*) => {
        #[derive(Debug)]
        pub enum StrongInstance {
            $(
                $class(Box<Instance<rbx_classes::instances::$class>>)
            ),*
        }

        // From impls
        $(
            impl From<Instance<rbx_classes::instances::$class>> for StrongInstance {
                fn from(value: Instance<rbx_classes::instances::$class>) -> Self {
                    Self::$class(Box::new(value))
                }
            }
        )*
    };
}
rbx_classes::for_each_class!(impl_strong_instance);
