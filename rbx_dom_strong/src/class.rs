use rbx_classes::instances;

macro_rules! impl_class {
    ($($class:ident),*) => {
        /// Class is an enum of boxed structs,
        /// with each struct being a different class.
        // ReflectionMetadataClass annoyingly procs this lint
        #[expect(clippy::enum_variant_names)]
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

/// Convert a `Class` enum into any descendant
/// of a superclass using dereferencing.
pub trait AsClass<C> {
    fn as_class(&self) -> Option<&C>;
    fn as_class_mut(&mut self) -> Option<&mut C>;
}
/// Class to class conversion. Inheritance as a trait bound.
/// Perform an infallible conversion using dereferencing.
/// Part -> BasePart is possible, but BasePart -> Part is not.
pub trait ToClass<C> {
    fn to_class(&self) -> &C;
    fn to_class_mut(&mut self) -> &mut C;
}
macro_rules! impl_traits_for_class_and_descendants {
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
        $(
            impl ToClass<instances::$class> for instances::$descendant {
                fn to_class(&self) -> &instances::$class {
                    self
                }
                fn to_class_mut(&mut self) -> &mut instances::$class {
                    self
                }
            }
        )*
    };
}
macro_rules! impl_as_class {
    ($(($class:ident,$descendants:tt)),*) => {
        $(
            impl_traits_for_class_and_descendants!($class,$descendants);
        )*
    };
}
rbx_classes::for_each_class_descendants!(impl_as_class);
