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
    ($($variant:ident($builder_ty:ty, $builder_ident:ident),)*) => {
        // use rbx_dom_weak::types::$type;
        #[derive(Debug)]
        pub enum PropVariantBuilder<'a>{
            $(
                $variant($builder_ty),
            )*
        }
        impl<'a> PropVariantBuilder<'a>{
            pub fn new(variant_type:VariantType) -> Self {
                match variant_type{
                    $(
                        VariantType::$variant => PropVariantBuilder::$variant($builder_ident::new()),
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
                match self {
                    $(
                        PropVariantBuilder::$variant(builder) => builder.push(variant),
                    )*
                }
            }
            pub fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
                match self{
                    $(
                        PropVariantBuilder::$variant(builder) => builder.dump(chunk),
                    )*
                }
            }
        }
    };
}

impl_prop_variant_builder! {
    Axes(AxesBuilder<'a>, AxesBuilder),
}
    // BinaryString(BinaryStringBuilder),
    // Bool(BoolBuilder),
    // BrickColor(BrickColorBuilder),
    // CFrame(CFrameBuilder),
    // Color3(Color3Builder),
    // Color3uint8(Color3uint8Builder),
    // ColorSequence(ColorSequenceBuilder),
    // ContentId(ContentIdBuilder),
    // Enum(EnumBuilder),
    // Faces(FacesBuilder),
    // Float32(Float32Builder),
    // Float64(Float64Builder),
    // Int32(Int32Builder),
    // Int64(Int64Builder),
    // NumberRange(NumberRangeBuilder),
    // NumberSequence(NumberSequenceBuilder),
    // PhysicalProperties(PhysicalPropertiesBuilder),
    // Ray(RayBuilder),
    // Rect(RectBuilder),
    // Ref(RefBuilder),
    // // Region3(Region3Builder),
    // // Region3int16(Region3int16Builder),
    // SharedString(SharedStringBuilder),
    // String(StringBuilder),
    // UDim(UDimBuilder),
    // UDim2(UDim2Builder),
    // Vector2(Vector2Builder),
    // // Vector2int16(Vector2int16Builder),
    // Vector3(Vector3Builder),
    // Vector3int16(Vector3int16Builder),
    // OptionalCFrame(OptionBuilder<CFrame>),
    // Tags(TagsBuilder),
    // Attributes(AttributesBuilder),
    // Font(FontBuilder),
    // UniqueId(UniqueIdBuilder),
    // MaterialColors(MaterialColorsBuilder),
    // SecurityCapabilities(SecurityCapabilitiesBuilder),
    // // EnumItem(EnumItemBuilder),
    // Content(ContentBuilder),

macro_rules! impl_simple_builder {
    ($builder:ident, $variant:ident, $type:ty) => {
        #[derive(Debug)]
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

impl_simple_builder!(AxesBuilder, Axes, Axes);
impl AxesBuilder<'_>{
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_u8(value.bits())?;
        }
        Ok(())
    }
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
