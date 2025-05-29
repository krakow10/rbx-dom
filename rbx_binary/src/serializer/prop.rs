use std::io::Write;

use crate::chunk::ChunkBuilder;
use crate::core::RbxWriteExt;

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
    BinaryString(BinaryStringBuilder<'a>, BinaryStringBuilder),
    Bool(BoolBuilder, BoolBuilder),
    BrickColor(BrickColorBuilder, BrickColorBuilder),
    CFrame(CFrameBuilder<'a>, CFrameBuilder),
    Color3(Color3Builder<'a>, Color3Builder),
    Color3uint8(Color3uint8Builder, Color3uint8Builder),
    ColorSequence(ColorSequenceBuilder<'a>, ColorSequenceBuilder),
    ContentId(ContentIdBuilder<'a>, ContentIdBuilder),
    Enum(EnumBuilder, EnumBuilder),
    Faces(FacesBuilder, FacesBuilder),
    Float32(Float32Builder, Float32Builder),
    Float64(Float64Builder, Float64Builder),
    Int32(Int32Builder, Int32Builder),
    Int64(Int64Builder, Int64Builder),
    NumberRange(NumberRangeBuilder<'a>, NumberRangeBuilder),
    NumberSequence(NumberSequenceBuilder<'a>, NumberSequenceBuilder),
    PhysicalProperties(PhysicalPropertiesBuilder<'a>, PhysicalPropertiesBuilder),
    Ray(RayBuilder<'a>, RayBuilder),
    Rect(RectBuilder<'a>, RectBuilder),
    Ref(RefBuilder<'a>, RefBuilder),
}
    // // Region3(Region3Builder<'a>, Region3Builder),
    // // Region3int16(Region3int16Builder<'a>, Region3int16Builder),
    // SharedString(SharedStringBuilder<'a>, SharedStringBuilder),
    // String(StringBuilder<'a>, StringBuilder),
    // UDim(UDimBuilder<'a>, UDimBuilder),
    // UDim2(UDim2Builder<'a>, UDim2Builder),
    // Vector2(Vector2Builder<'a>, Vector2Builder),
    // // Vector2int16(Vector2int16Builder<'a>, Vector2int16Builder),
    // Vector3(Vector3Builder<'a>, Vector3Builder),
    // Vector3int16(Vector3int16Builder<'a>, Vector3int16Builder),
    // OptionalCFrame(OptionBuilder<'a>, OptionBuilder<CFrame>),
    // Tags(TagsBuilder<'a>, TagsBuilder),
    // Attributes(AttributesBuilder<'a>, AttributesBuilder),
    // Font(FontBuilder<'a>, FontBuilder),
    // UniqueId(UniqueIdBuilder<'a>, UniqueIdBuilder),
    // MaterialColors(MaterialColorsBuilder<'a>, MaterialColorsBuilder),
    // SecurityCapabilities(SecurityCapabilitiesBuilder<'a>, SecurityCapabilitiesBuilder),
    // // EnumItem(EnumItemBuilder<'a>, EnumItemBuilder),
    // Content(ContentBuilder<'a>, ContentBuilder),

macro_rules! impl_ref_builder {
    ($builder:ident, $variant:ident, $type:ty) => {
        #[derive(Debug)]
        struct $builder<'a> {
            values: Vec<&'a $type>,
        }
        impl<'a> $builder<'a> {
            fn new() -> Self {
                Self { values: Vec::new() }
            }
            fn push(&mut self, variant: &'a Variant) -> Result<(), VariantError> {
                match variant {
                    Variant::$variant(value) => self.values.push(value),
                    observed => {
                        return Err(VariantError {
                            expected: VariantType::$variant,
                            observed: observed.ty(),
                        });
                    }
                }
                Ok(())
            }
        }
    };
}
macro_rules! impl_copy_builder {
    ($builder:ident, $variant:ident, $type:ty) => {
        impl_copy_builder_pushless!($builder, $variant, $type);
        impl $builder {
            fn push(&mut self, variant: &Variant) -> Result<(), VariantError> {
                match variant {
                    Variant::$variant(value) => self.values.push(*value),
                    observed => {
                        return Err(VariantError {
                            expected: VariantType::$variant,
                            observed: observed.ty(),
                        });
                    }
                }
                Ok(())
            }
        }
    };
}
macro_rules! impl_copy_builder_pushless {
    ($builder:ident, $variant:ident, $type:ty) => {
        #[derive(Debug)]
        struct $builder {
            values: Vec<$type>,
        }
        impl $builder {
            fn new() -> Self {
                Self { values: Vec::new() }
            }
        }
    };
}
macro_rules! impl_convert_builder {
    ($builder:ident, $variant:ident, $type:ty, $convert:expr) => {
        #[derive(Debug)]
        struct $builder {
            values: Vec<$type>,
        }
        impl $builder {
            fn new() -> Self {
                Self { values: Vec::new() }
            }
            fn push(&mut self, variant: &Variant) -> Result<(), VariantError> {
                match variant {
                    Variant::$variant(value) => self.values.push($convert(value)),
                    observed => {
                        return Err(VariantError {
                            expected: VariantType::$variant,
                            observed: observed.ty(),
                        });
                    }
                }
                Ok(())
            }
        }
    };
}

impl_ref_builder!(AxesBuilder, Axes, Axes);
impl AxesBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_u8(value.bits())?;
        }
        Ok(())
    }
}

impl_ref_builder!(BinaryStringBuilder, BinaryString, BinaryString);
impl BinaryStringBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_binary_string(value)?;
        }
        Ok(())
    }
}

impl_copy_builder!(BoolBuilder, Bool, bool);
impl BoolBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_bool(value)?;
        }
        Ok(())
    }
}

impl_convert_builder!(BrickColorBuilder, BrickColor, u32, |&value|value as u32);
impl BrickColorBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_u32_array(self.values.iter().copied())
    }
}

impl_ref_builder!(CFrameBuilder, CFrame, CFrame);
impl CFrameBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut x = Vec::with_capacity(self.values.len());
        let mut y = Vec::with_capacity(self.values.len());
        let mut z = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            let matrix = &value.orientation;
            if let Some(id) = matrix.to_basic_rotation_id() {
                chunk.write_u8(id)?;
            } else {
                chunk.write_u8(0x00)?;

                chunk.write_le_f32(matrix.x.x)?;
                chunk.write_le_f32(matrix.x.y)?;
                chunk.write_le_f32(matrix.x.z)?;

                chunk.write_le_f32(matrix.y.x)?;
                chunk.write_le_f32(matrix.y.y)?;
                chunk.write_le_f32(matrix.y.z)?;

                chunk.write_le_f32(matrix.z.x)?;
                chunk.write_le_f32(matrix.z.y)?;
                chunk.write_le_f32(matrix.z.z)?;
            }
            x.push(value.position.x);
            y.push(value.position.y);
            z.push(value.position.z);
        }

        chunk.write_interleaved_f32_array(x)?;
        chunk.write_interleaved_f32_array(y)?;
        chunk.write_interleaved_f32_array(z)?;
        Ok(())
    }
}

impl_ref_builder!(Color3Builder, Color3, Color3);
impl Color3Builder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut r = Vec::with_capacity(self.values.len());
        let mut g = Vec::with_capacity(self.values.len());
        let mut b = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            r.push(value.r);
            g.push(value.g);
            b.push(value.b);
        }
        chunk.write_interleaved_f32_array(r)?;
        chunk.write_interleaved_f32_array(g)?;
        chunk.write_interleaved_f32_array(b)?;
        Ok(())
    }
}

impl_copy_builder!(Color3uint8Builder, Color3uint8, Color3uint8);
impl Color3uint8Builder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut r = Vec::with_capacity(self.values.len());
        let mut g = Vec::with_capacity(self.values.len());
        let mut b = Vec::with_capacity(self.values.len());
        for value in &self.values {
            r.push(value.r);
            g.push(value.g);
            b.push(value.b);
        }
        chunk.write_all(r.as_slice())?;
        chunk.write_all(g.as_slice())?;
        chunk.write_all(b.as_slice())?;
        Ok(())
    }
}

impl_ref_builder!(ColorSequenceBuilder, ColorSequence, ColorSequence);
impl ColorSequenceBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_le_u32(value.keypoints.len() as u32)?;

            for keypoint in &value.keypoints {
                chunk.write_le_f32(keypoint.time)?;
                chunk.write_le_f32(keypoint.color.r)?;
                chunk.write_le_f32(keypoint.color.g)?;
                chunk.write_le_f32(keypoint.color.b)?;

                // write out a dummy value for envelope, which is serialized but doesn't do anything
                chunk.write_le_f32(0.0)?;
            }
        }
        Ok(())
    }
}

impl_ref_builder!(ContentIdBuilder, ContentId, ContentId);
impl ContentIdBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_string(value.as_ref())?;
        }
        Ok(())
    }
}

impl_convert_builder!(EnumBuilder, Enum, u32, |value:&Enum|value.to_u32());
impl EnumBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_u32_array(
            self.values.iter().copied(),
        )
    }
}

impl_convert_builder!(FacesBuilder, Faces, u8, |value:&Faces|value.bits());
impl FacesBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_all(&self.values)
    }
}

impl_copy_builder!(Float32Builder, Float32, f32);
impl Float32Builder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_f32_array(self.values.iter().copied())
    }
}

impl_copy_builder_pushless!(Float64Builder, Float64, f64);
impl Float64Builder {
    fn push(&mut self, variant: &Variant) -> Result<(), VariantError> {
        match variant {
            Variant::Float32(value) => self.values.push(*value as f64),
            Variant::Float64(value) => self.values.push(*value),
            observed => {
                return Err(VariantError {
                    expected: VariantType::Float64,
                    observed: observed.ty(),
                });
            }
        }
        Ok(())
    }
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_le_f64(value)?;
        }
        Ok(())
    }
}

impl_copy_builder!(Int32Builder, Int32, i32);
impl Int32Builder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_i32_array(self.values.iter().copied())
    }
}

impl_copy_builder_pushless!(Int64Builder, Int64, i64);
impl Int64Builder {
    fn push(&mut self, variant: &Variant) -> Result<(), VariantError> {
        match variant {
            Variant::Int32(value) => self.values.push(*value as i64),
            Variant::Int64(value) => self.values.push(*value),
            observed => {
                return Err(VariantError {
                    expected: VariantType::Int64,
                    observed: observed.ty(),
                });
            }
        }
        Ok(())
    }
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_i64_array(self.values.iter().copied())
    }
}

impl_ref_builder!(NumberRangeBuilder, NumberRange, NumberRange);
impl NumberRangeBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_le_f32(value.min)?;
            chunk.write_le_f32(value.max)?;
        }
        Ok(())
    }
}

impl_ref_builder!(NumberSequenceBuilder, NumberSequence, NumberSequence);
impl NumberSequenceBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_le_u32(value.keypoints.len() as u32)?;

            for keypoint in &value.keypoints {
                chunk.write_le_f32(keypoint.time)?;
                chunk.write_le_f32(keypoint.value)?;
                chunk.write_le_f32(keypoint.envelope)?;
            }
        }
        Ok(())
    }
}

impl_ref_builder!(PhysicalPropertiesBuilder, PhysicalProperties, PhysicalProperties);
impl PhysicalPropertiesBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            if let PhysicalProperties::Custom(props) = value {
                chunk.write_u8(1)?;
                chunk.write_le_f32(props.density)?;
                chunk.write_le_f32(props.friction)?;
                chunk.write_le_f32(props.elasticity)?;
                chunk.write_le_f32(props.friction_weight)?;
                chunk.write_le_f32(props.elasticity_weight)?;
            } else {
                chunk.write_u8(0)?;
            }
        }
        Ok(())
    }
}

impl_ref_builder!(RayBuilder, Ray, Ray);
impl RayBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_le_f32(value.origin.x)?;
            chunk.write_le_f32(value.origin.y)?;
            chunk.write_le_f32(value.origin.z)?;
            chunk.write_le_f32(value.direction.x)?;
            chunk.write_le_f32(value.direction.y)?;
            chunk.write_le_f32(value.direction.x)?;
        }
        Ok(())
    }
}

impl_ref_builder!(RectBuilder, Rect, Rect);
impl RectBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut x_min = Vec::with_capacity(self.values.len());
        let mut y_min = Vec::with_capacity(self.values.len());
        let mut x_max = Vec::with_capacity(self.values.len());
        let mut y_max = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            x_min.push(value.min.x);
            y_min.push(value.min.y);
            x_max.push(value.max.x);
            y_max.push(value.max.y);
        }
        chunk.write_interleaved_f32_array(x_min)?;
        chunk.write_interleaved_f32_array(y_min)?;
        chunk.write_interleaved_f32_array(x_max)?;
        chunk.write_interleaved_f32_array(y_max)?;
        Ok(())
    }
}

impl_ref_builder!(RefBuilder, Ref, Ref);
impl RefBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let it = self.values.iter().map(|&value| {
            if let Some(id) = id_to_referent.get(value) {
                *id
            } else {
                -1
            }
        });

        chunk.write_referent_array(it)?;
        Ok(())
    }
}


impl_ref_builder!(StringBuilder, String, str);
impl StringBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_string(value)?;
        }
        Ok(())
    }
}
