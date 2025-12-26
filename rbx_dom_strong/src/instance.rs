use rbx_classes::instances;
use rbx_types::Ref;

#[derive(Debug)]
pub struct Instance {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,
    class: Class,
}

#[derive(Debug)]
pub struct InstanceBuilder<C> {
    referent: Ref,
    children: Vec<InstanceBuilder<Class>>,
    class: C,
}

impl<C> InstanceBuilder<C> {
    /// Return the referent of the instance that the `InstanceBuilder` refers to.
    pub fn referent(&self) -> Ref {
        self.referent
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

macro_rules! impl_strong_instance {
    ($($class:ident),*) => {
        #[derive(Debug)]
        pub enum Class {
            $(
                $class(Box<instances::$class>),
            )*
        }

        // From impls
        $(
            impl From<instances::$class> for Class {
                fn from(value: instances::$class) -> Self {
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
        impl AsClass<instances::$class> for Class {
            fn as_class(&self) -> Option<&instances::$class> {
                Some(match self {
                    $(
                        Class::$descendant(class) => class,
                    )*
                    _ => return None,
                })
            }
            fn as_class_mut(&mut self) -> Option<&mut instances::$class> {
                Some(match self {
                    $(
                        Class::$descendant(class) => class,
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

impl Instance {
    pub fn as_class<C>(&self) -> Option<&C>
    where
        Class: AsClass<C>,
    {
        self.class.as_class()
    }
    pub fn as_class_mut<C>(&mut self) -> Option<&mut C>
    where
        Class: AsClass<C>,
    {
        self.class.as_class_mut()
    }
}
