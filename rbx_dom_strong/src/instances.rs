use crate::instance::InstanceInner;
use rbx_classes::instances;
macro_rules! impl_instance_aliases {
    ($($class:ident),*) => {
        $(
            pub type $class = instances::$class<InstanceInner>;
        )*
    }
}
rbx_classes::for_each_class!(impl_instance_aliases);
