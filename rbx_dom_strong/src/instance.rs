use rbx_classes::instances;
use rbx_types::Ref;

#[derive(Debug)]
pub struct InstanceInner {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,
}

#[derive(Debug)]
pub struct InstanceBuilderInner {
    referent: Ref,
    children: Vec<StrongInstance<InstanceBuilderInner>>,
}

impl InstanceBuilderInner {
    /// Return the referent of the instance that the `InstanceBuilder` refers to.
    pub fn referent(&self) -> Ref {
        self.referent
    }
}

impl Default for InstanceBuilderInner {
    fn default() -> Self {
        Self {
            referent: Ref::new(),
            children: Vec::new(),
        }
    }
}

macro_rules! impl_strong_instance {
    ($($class:ident),*) => {
        #[derive(Debug)]
        pub enum StrongInstance<I> {
            $(
                $class(Box<instances::$class<I>>),
            )*
        }

        // From impls
        $(
            impl<I> From<instances::$class<I>> for StrongInstance<I> {
                fn from(value: instances::$class<I>) -> Self {
                    Self::$class(Box::new(value))
                }
            }
        )*
    };
}
rbx_classes::for_each_class!(impl_strong_instance);

pub trait AsClass<Class> {
    fn as_class(&self) -> Option<&Class>;
    fn as_class_mut(&mut self) -> Option<&mut Class>;
}
macro_rules! impl_as_class_for_instance_and_descendants {
    ($class:ident, [$($descendant:ident),*]) => {
        #[allow(unreachable_patterns)]
        impl<I> AsClass<instances::$class<I>> for StrongInstance<I> {
            fn as_class(&self) -> Option<&instances::$class<I>> {
                Some(match self {
                    $(
                        StrongInstance::$descendant(class) => class,
                    )*
                    _ => return None,
                })
            }
            fn as_class_mut(&mut self) -> Option<&mut instances::$class<I>> {
                Some(match self {
                    $(
                        StrongInstance::$descendant(class) => class,
                    )*
                    _ => return None,
                })
            }
        }
    };
}
macro_rules! impl_as_class {
    ($(($class:ident,$descendants:tt)),*) => {
        $(
            impl_as_class_for_instance_and_descendants!($class,$descendants);
        )*
    };
}
rbx_classes::for_each_class_descendants!(impl_as_class);

impl<I> StrongInstance<I> {
    pub fn as_class<Class>(&self) -> Option<&Class>
    where
        Self: AsClass<Class>,
    {
        AsClass::as_class(self)
    }
    pub fn as_class_mut<Class>(&mut self) -> Option<&mut Class>
    where
        Self: AsClass<Class>,
    {
        AsClass::as_class_mut(self)
    }
}
