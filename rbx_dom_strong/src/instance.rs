use core::ops::{Deref, DerefMut};

use rbx_types::{CFrame, Enum, Ref};

macro_rules! impl_inherits {
    ($class:ident,$inherits:ident) => {
        impl Deref for $class {
            type Target = $inherits;
            fn deref(&self) -> &$inherits {
                &self.superclass
            }
        }
        impl DerefMut for $class {
            fn deref_mut(&mut self) -> &mut $inherits {
                &mut self.superclass
            }
        }
    };
}

/// The base class for all other classes
#[derive(Debug)]
pub struct Instance {
    pub(crate) referent: Ref,
    pub(crate) children: Vec<Ref>,
    pub(crate) parent: Ref,

    pub Archivable: bool,
    pub name: String,
    // more properties ...
}

#[derive(Debug)]
pub struct PVInstance {
    superclass: Instance,
    pub Origin: CFrame,
    // more properties ...
}
impl_inherits!(PVInstance, Instance);

#[derive(Debug)]
pub struct BasePart {
    superclass: PVInstance,
    pub CFrame: CFrame,
    // more properties ...
}
impl_inherits!(BasePart, PVInstance);

#[derive(Debug)]
pub struct FormFactorPart {
    superclass: BasePart,
    pub FormFactor: Enum,
}
impl_inherits!(FormFactorPart, BasePart);

#[derive(Debug)]
pub struct Part {
    superclass: FormFactorPart,
    pub Shape: Enum,
}
impl_inherits!(Part, FormFactorPart);

#[derive(Debug)]
pub struct WedgePart {
    superclass: FormFactorPart,
}
impl_inherits!(WedgePart, FormFactorPart);

#[derive(Debug)]
pub enum StrongInstance {
    Part(Box<Part>),
    WedgePart(Box<WedgePart>),
}
