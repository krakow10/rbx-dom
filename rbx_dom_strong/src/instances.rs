use crate::instance;
use rbx_classes::instances;
macro_rules! impl_instance_aliases {
    ($($class:ident),*) => {
        $(
            pub type $class = instances::$class<instance::Instance>;
        )*
    }
}
rbx_classes::for_each_class!(impl_instance_aliases);
