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
macro_rules! impl_strong_instance_from {
    ($class:ident) => {
        impl From<$class> for StrongInstance {
            fn from(value: $class) -> Self {
                Self::$class(Box::new(value))
            }
        }
    };
}
impl_strong_instance_from!(Part);
impl_strong_instance_from!(WedgePart);

#[cfg(test)]
mod test {
    use super::*;
    use rbx_types::{CFrame, Matrix3, Vector3};

    fn default_cframe() -> CFrame {
        CFrame::new(Vector3::new(0.0, 0.0, 0.0), Matrix3::identity())
    }
    fn default_part() -> Part {
        Part {
            superclass: FormFactorPart {
                superclass: BasePart {
                    superclass: PVInstance {
                        superclass: Instance {
                            Archivable: true,
                            referent: Ref::none(),
                            children: Vec::new(),
                            parent: Ref::none(),
                            name: "".to_owned(),
                        },
                        Origin: default_cframe(),
                    },
                    CFrame: default_cframe(),
                },
                FormFactor: Enum::from_u32(0),
            },
            Shape: Enum::from_u32(0),
        }
    }
    #[test]
    fn part_inherits_instance() {
        // Part::default() would be nice
        let mut part: Part = default_part();

        // look ma, inheritance in rust!
        part.name = "Part".to_owned();
        part.CFrame = default_cframe();
    }
}
