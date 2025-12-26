use rbx_classes::instances;

macro_rules! impl_class {
    ($($class:ident),*) => {
        /// Class is an enum of boxed structs,
        /// with each struct being a different class.
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
rbx_classes::for_each_class!(impl_class);

pub trait AsClass<C> {
    fn as_class(&self) -> Option<&C>;
    fn as_class_mut(&mut self) -> Option<&mut C>;
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
