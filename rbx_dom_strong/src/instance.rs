use rbx_types::Ref;

#[derive(Debug)]
pub struct Instance {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            referent: Ref::new(),
            children: Vec::new(),
            parent: Ref::none(),
        }
    }
}

macro_rules! impl_strong_instance {
    ($($class:ident),*) => {
        #[derive(Debug)]
        pub enum StrongInstance {
            $(
                $class(Box<rbx_classes::instances::$class<Instance>>)
            ),*
        }

        // From impls
        $(
            impl From<rbx_classes::instances::$class<Instance>> for StrongInstance {
                fn from(value: rbx_classes::instances::$class<Instance>) -> Self {
                    Self::$class(Box::new(value))
                }
            }
        )*
    };
}
rbx_classes::for_each_class!(impl_strong_instance);
