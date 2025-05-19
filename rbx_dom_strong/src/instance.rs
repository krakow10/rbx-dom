use rbx_types::{CFrame, Enum, Ref};

macro_rules! impl_as_ref_mut {
    ($class:ident,$field:ident,$inherits:ident) => {
        impl AsRef<$inherits> for $class {
            fn as_ref(&self) -> &$inherits {
                &self.$field
            }
        }
        impl AsMut<$inherits> for $class {
            fn as_mut(&mut self) -> &mut $inherits {
                &mut self.$field
            }
        }
    };
}

// Property bags that can be composed to form a complete class
#[derive(Debug)]
pub struct Instance {
    pub(crate) referent: Ref,
    pub(crate) children: Vec<Ref>,
    pub(crate) parent: Ref,
    pub name: String,
}

#[derive(Debug)]
pub struct PVInstance {
    pub Origin: CFrame,
}

#[derive(Debug)]
pub struct BasePart {
    pub CFrame: CFrame,
}

#[derive(Debug)]
pub struct FormFactorPart {
    pub FormFactor: Enum,
}

#[derive(Debug)]
pub struct Part {
    pub Shape: Enum,
}

// A complete class
#[derive(Debug)]
pub struct PartClass {
    instance: Instance,
    pv_instance: PVInstance,
    base_part: BasePart,
    form_factor_part: FormFactorPart,
    part: Part,
}

impl_as_ref_mut!(PartClass, instance, Instance);
impl_as_ref_mut!(PartClass, pv_instance, PVInstance);
impl_as_ref_mut!(PartClass, base_part, BasePart);
impl_as_ref_mut!(PartClass, form_factor_part, FormFactorPart);
impl_as_ref_mut!(PartClass, part, Part);

#[derive(Debug)]
pub enum StrongInstance {
    Part(Box<PartClass>),
}
