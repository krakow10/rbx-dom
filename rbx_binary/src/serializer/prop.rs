use crate::core::RbxWriteExt;
use crate::chunk::ChunkBuilder;

use rbx_dom_weak::types::{
    Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
    Content, ContentId, Enum, Faces, Font, MaterialColors, NumberRange, NumberSequence,
    PhysicalProperties, Ray, Rect, Ref, SecurityCapabilities, SharedString, Tags, UDim, UDim2,
    UniqueId, Vector2, Vector3, Vector3int16,
};
use rbx_dom_weak::types::{Variant, VariantType};
#[derive(Debug)]
pub struct VariantError {
    expected: VariantType,
    observed: VariantType,
}
macro_rules! impl_prop_variant_builder {
    ($($variant:ident($builder:path),)*) => {
        // use rbx_dom_weak::types::$type;
        #[derive(Debug)]
        pub enum PropVariantBuilder<'a>{
            $(
                $variant($builder),
            )*
        }
        impl<'a> PropVariantBuilder<'a>{
            pub fn new(variant_type:VariantType) -> Self {
                match variant_type{
                    $(
                        VariantType::$variant => PropVariantBuilder::$variant($builder::new()),
                    )*
                    _=>panic!("Unknown VariantType {:?}", variant_type),
                }
            }
            pub fn ty(&self) -> VariantType {
                match self{
                    $(
                        PropVariantBuilder::$variant(_) => VariantType::$variant,
                    )*
                }
            }
            pub fn push(&mut self, variant: &'a Variant) -> Result<(),VariantError> {
                match (variant, self) {
                    $(
                        (Variant::$variant(value), PropVariantBuilder::$variant(values)) => values.push(value),
                    )*
                    (observed,expected)=>return Err(VariantError{
                        expected:expected.ty(),
                        observed:observed.ty(),
                    }),
                }
                Ok(())
            }
            pub fn cloned_variant_vec(&self) -> Vec<Variant> {
                match self{
                    $(
                        PropVariantBuilder::$variant(values) => values.iter().copied().cloned().map(Variant::$variant).collect(),
                    )*
                }
            }
        }
    };
}

impl_prop_variant_builder! {
    Axes(Axes),
    BinaryString(BinaryString),
    Bool(bool),
    BrickColor(BrickColor),
    CFrame(CFrame),
    Color3(Color3),
    Color3uint8(Color3uint8),
    ColorSequence(ColorSequence),
    ContentId(ContentId),
    Enum(Enum),
    Faces(Faces),
    Float32(f32),
    Float64(f64),
    Int32(i32),
    Int64(i64),
    NumberRange(NumberRange),
    NumberSequence(NumberSequence),
    PhysicalProperties(PhysicalProperties),
    Ray(Ray),
    Rect(Rect),
    Ref(Ref),
    // Region3(Region3),
    // Region3int16(Region3int16),
    SharedString(SharedString),
    String(String),
    UDim(UDim),
    UDim2(UDim2),
    Vector2(Vector2),
    // Vector2int16(Vector2int16),
    Vector3(Vector3),
    Vector3int16(Vector3int16),
    OptionalCFrame(Option<CFrame>),
    Tags(Tags),
    Attributes(Attributes),
    Font(Font),
    UniqueId(UniqueId),
    MaterialColors(MaterialColors),
    SecurityCapabilities(SecurityCapabilities),
    // EnumItem(EnumItem),
    Content(Content),
}

macro_rules! impl_simple_builder {
    ($builder:ident, $variant:ident, $type:ty) => {
        struct $builder<'a>{
            values:Vec<&'a $type>,
        }
        impl<'a> $builder<'a>{
            fn new() -> Self{
                Self{
                    values: Vec::new(),
                }
            }
            fn push(&mut self, variant: &'a Variant) -> Result<(),VariantError> {
                match variant {
                    Variant::$variant(value) => self.values.push(value),
                    observed=>return Err(VariantError{
                        expected:VariantType::$variant,
                        observed:observed.ty(),
                    }),
                }
                Ok(())
            }
        }
    };
}

impl_simple_builder!(StringBuilder, String, str);
impl StringBuilder<'_>{
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_string(value)?;
        }
        Ok(())
    }
}
