use rbx_classes::instances;
use rbx_types::Ref;

#[derive(Debug)]
pub struct InstanceInner {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,
}

impl Default for InstanceInner {
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
                $class(Box<instances::$class<InstanceInner>>),
            )*
        }
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
        impl AsClass<instances::$class<InstanceInner>> for StrongInstance {
            fn as_class(&self) -> Option<&instances::$class<InstanceInner>> {
                Some(match self {
                    $(
                        StrongInstance::$descendant(class) => class,
                    )*
                    _ => return None,
                })
            }
            fn as_class_mut(&mut self) -> Option<&mut instances::$class<InstanceInner>> {
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

impl StrongInstance {
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
