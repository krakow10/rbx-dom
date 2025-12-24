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
                $class(Box<rbx_classes::instances::$class<InstanceInner>>),
            )*
        }

        // From impls
        $(
            impl From<rbx_classes::instances::$class<InstanceInner>> for StrongInstance {
                fn from(value: rbx_classes::instances::$class<InstanceInner>) -> Self {
                    Self::$class(Box::new(value))
                }
            }
        )*
    };
}
rbx_classes::for_each_class!(impl_strong_instance);

impl<'a> From<&'a StrongInstance>
    for Result<&'a rbx_classes::instances::BasePart<InstanceInner>, &'a StrongInstance>
{
    fn from(value: &'a StrongInstance) -> Self {
        match value {
            StrongInstance::CornerWedgePart(class) => Ok(class),
            StrongInstance::FormFactorPart(class) => Ok(class),
            StrongInstance::Terrain(class) => Ok(class),
            StrongInstance::TriangleMeshPart(class) => Ok(class),
            StrongInstance::TrussPart(class) => Ok(class),
            StrongInstance::VehicleSeat(class) => Ok(class),
            StrongInstance::Part(class) => Ok(class),
            StrongInstance::WedgePart(class) => Ok(class),
            StrongInstance::FlagStand(class) => Ok(class),
            StrongInstance::Platform(class) => Ok(class),
            StrongInstance::Seat(class) => Ok(class),
            StrongInstance::SkateboardPlatform(class) => Ok(class),
            StrongInstance::SpawnLocation(class) => Ok(class),
            other => Err(other),
        }
    }
}
