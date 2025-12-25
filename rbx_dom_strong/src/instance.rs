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

        // From impls
        $(
            impl From<instances::$class<InstanceInner>> for StrongInstance {
                fn from(value: instances::$class<InstanceInner>) -> Self {
                    Self::$class(Box::new(value))
                }
            }
        )*
    };
}
rbx_classes::for_each_class!(impl_strong_instance);

pub trait AsClass<Class> {
    fn as_class(&self) -> Option<&Class>;
}
pub trait AsClassMut<Class> {
    fn as_class_mut(&mut self) -> Option<&mut Class>;
}
impl AsClass<instances::BasePart<InstanceInner>> for StrongInstance {
    fn as_class(&self) -> Option<&instances::BasePart<InstanceInner>> {
        Some(match self {
            StrongInstance::CornerWedgePart(class) => class,
            StrongInstance::FormFactorPart(class) => class,
            StrongInstance::Terrain(class) => class,
            StrongInstance::TriangleMeshPart(class) => class,
            StrongInstance::TrussPart(class) => class,
            StrongInstance::VehicleSeat(class) => class,
            StrongInstance::Part(class) => class,
            StrongInstance::WedgePart(class) => class,
            StrongInstance::FlagStand(class) => class,
            StrongInstance::Platform(class) => class,
            StrongInstance::Seat(class) => class,
            StrongInstance::SkateboardPlatform(class) => class,
            StrongInstance::SpawnLocation(class) => class,
            _ => return None,
        })
    }
}

impl StrongInstance {
    pub fn as_class<Class>(&self) -> Option<&Class>
    where
        Self: AsClass<Class>,
    {
        AsClass::as_class(self)
    }
    pub fn as_class_mut<Class>(&mut self) -> Option<&mut Class>
    where
        Self: AsClassMut<Class>,
    {
        AsClassMut::as_class_mut(self)
    }
}
