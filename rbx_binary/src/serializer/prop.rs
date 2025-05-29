use std::io::Write;

use crate::chunk::ChunkBuilder;
use crate::core::RbxWriteExt;
use crate::types::Type;

use rbx_dom_weak::types::{
    Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
    Content, ContentId, Enum, Faces, Font, MaterialColors, NumberRange, NumberSequence,
    PhysicalProperties, Ray, Rect, Ref, SecurityCapabilities, SharedString, Tags, UDim, UDim2,
    UniqueId, Vector2, Vector3, Vector3int16,
};
use rbx_dom_weak::types::{ContentType, Variant, VariantType};
#[derive(Debug)]
pub struct VariantError {
    expected: VariantType,
    observed: VariantType,
}
macro_rules! impl_prop_variant_builder {
    ($($variant:ident($builder_ty:ty, $builder_ident:ident),)*) => {
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
    Axes(AxesBuilder, AxesBuilder),
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
    // Region3(Region3Builder<'a>, Region3Builder),
    // Region3int16(Region3int16Builder<'a>, Region3int16Builder),
    SharedString(SharedStringBuilder<'a>, SharedStringBuilder),
    String(StringBuilder<'a>, StringBuilder),
    UDim(UDimBuilder<'a>, UDimBuilder),
    UDim2(UDim2Builder<'a>, UDim2Builder),
    Vector2(Vector2Builder<'a>, Vector2Builder),
    // Vector2int16(Vector2int16Builder<'a>, Vector2int16Builder),
    Vector3(Vector3Builder<'a>, Vector3Builder),
    Vector3int16(Vector3int16Builder<'a>, Vector3int16Builder),
    OptionalCFrame(OptionalCFrameBuilder<'a>, OptionalCFrameBuilder),
    Tags(TagsBuilder<'a>, TagsBuilder),
    Attributes(AttributesBuilder<'a>, AttributesBuilder),
    Font(FontBuilder<'a>, FontBuilder),
    UniqueId(UniqueIdBuilder<'a>, UniqueIdBuilder),
    MaterialColors(MaterialColorsBuilder<'a>, MaterialColorsBuilder),
    SecurityCapabilities(SecurityCapabilitiesBuilder, SecurityCapabilitiesBuilder),
    // EnumItem(EnumItemBuilder<'a>, EnumItemBuilder),
    Content(ContentBuilder<'a>, ContentBuilder),
}

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

impl_convert_builder!(AxesBuilder, Axes, u8, |value: &Axes| value.bits());
impl AxesBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_all(&self.values)?;
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

impl_convert_builder!(
    BrickColorBuilder,
    BrickColor,
    u32,
    |&value: &BrickColor| value as u32
);
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

impl_convert_builder!(EnumBuilder, Enum, u32, |value: &Enum| value.to_u32());
impl EnumBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_u32_array(self.values.iter().copied())
    }
}

impl_convert_builder!(FacesBuilder, Faces, u8, |value: &Faces| value.bits());
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

impl_ref_builder!(
    PhysicalPropertiesBuilder,
    PhysicalProperties,
    PhysicalProperties
);
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

impl_ref_builder!(SharedStringBuilder, SharedString, SharedString);
impl SharedStringBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let it = self.values.iter().map(|&value| {
            if let Some(id) = shared_string_ids.get(value) {
                *id
            } else {
                panic!(
                    "SharedString {} was not found during type collection",
                    value.hash()
                )
            }
        });
        chunk.write_interleaved_u32_array(it)?;
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

impl_ref_builder!(UDimBuilder, UDim, UDim);
impl UDimBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut scale = Vec::with_capacity(self.values.len());
        let mut offset = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            scale.push(value.scale);
            offset.push(value.offset);
        }
        chunk.write_interleaved_f32_array(scale)?;
        chunk.write_interleaved_i32_array(offset)?;
        Ok(())
    }
}

impl_ref_builder!(UDim2Builder, UDim2, UDim2);
impl UDim2Builder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut scale_x = Vec::with_capacity(self.values.len());
        let mut scale_y = Vec::with_capacity(self.values.len());
        let mut offset_x = Vec::with_capacity(self.values.len());
        let mut offset_y = Vec::with_capacity(self.values.len());

        for &value in &self.values {
            scale_x.push(value.x.scale);
            scale_y.push(value.y.scale);
            offset_x.push(value.x.offset);
            offset_y.push(value.y.offset);
        }

        chunk.write_interleaved_f32_array(scale_x)?;
        chunk.write_interleaved_f32_array(scale_y)?;
        chunk.write_interleaved_i32_array(offset_x)?;
        chunk.write_interleaved_i32_array(offset_y)?;
        Ok(())
    }
}

impl_ref_builder!(Vector2Builder, Vector2, Vector2);
impl Vector2Builder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut x = Vec::with_capacity(self.values.len());
        let mut y = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            x.push(value.x);
            y.push(value.y);
        }
        chunk.write_interleaved_f32_array(x)?;
        chunk.write_interleaved_f32_array(y)?;
        Ok(())
    }
}

impl_ref_builder!(Vector3Builder, Vector3, Vector3);
impl Vector3Builder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut x = Vec::with_capacity(self.values.len());
        let mut y = Vec::with_capacity(self.values.len());
        let mut z = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            x.push(value.x);
            y.push(value.y);
            z.push(value.z);
        }
        chunk.write_interleaved_f32_array(x)?;
        chunk.write_interleaved_f32_array(y)?;
        chunk.write_interleaved_f32_array(z)?;
        Ok(())
    }
}

impl_ref_builder!(Vector3int16Builder, Vector3int16, Vector3int16);
impl Vector3int16Builder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_le_i16(value.x)?;
            chunk.write_le_i16(value.y)?;
            chunk.write_le_i16(value.z)?;
        }
        Ok(())
    }
}

impl_ref_builder!(OptionalCFrameBuilder, OptionalCFrame, Option<CFrame>);
impl OptionalCFrameBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut bools = Vec::with_capacity(self.values.len());
        let mut x = Vec::with_capacity(self.values.len());
        let mut y = Vec::with_capacity(self.values.len());
        let mut z = Vec::with_capacity(self.values.len());

        chunk.write_u8(Type::CFrame as u8)?;
        for &value in &self.values {
            if let Some(value) = value {
                x.push(value.position.x);
                y.push(value.position.y);
                z.push(value.position.z);
                bools.push(0x01);

                let matrix = &value.orientation;

                // write matrix
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
            } else {
                x.push(0.0);
                y.push(0.0);
                z.push(0.0);
                bools.push(0x00);

                // Identity basic rotation id
                chunk.write_u8(0x02)?;
            };
        }

        chunk.write_interleaved_f32_array(x)?;
        chunk.write_interleaved_f32_array(y)?;
        chunk.write_interleaved_f32_array(z)?;

        chunk.write_u8(Type::Bool as u8)?;
        chunk.write_all(bools.as_slice())?;
        Ok(())
    }
}

impl_ref_builder!(TagsBuilder, Tags, Tags);
impl TagsBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            let buf = value.encode();
            chunk.write_binary_string(&buf)?;
        }
        Ok(())
    }
}

impl_ref_builder!(AttributesBuilder, Attributes, Attributes);
impl AttributesBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for (i, &value) in self.values.iter().enumerate() {
            let mut buf = Vec::new();

            value
                .to_writer(&mut buf)
                .map_err(|_| invalid_value(i, &Variant::Attributes(value.clone())))?;

            chunk.write_binary_string(&buf)?;
        }
        Ok(())
    }
}

impl_ref_builder!(FontBuilder, Font, Font);
impl FontBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_string(&value.family)?;
            chunk.write_le_u16(value.weight.as_u16())?;
            chunk.write_u8(value.style.as_u8())?;
            chunk.write_string(value.cached_face_id.as_deref().unwrap_or_default())?;
        }
        Ok(())
    }
}

impl_ref_builder!(UniqueIdBuilder, UniqueId, UniqueId);
impl UniqueIdBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut blobs = Vec::with_capacity(self.values.len());
        for &value in &self.values {
            let mut blob = [0; 16];
            // This is maybe not the best solution to this
            // but we can always change it.
            blob[0..4].copy_from_slice(&value.index().to_be_bytes());
            blob[4..8].copy_from_slice(&value.time().to_be_bytes());
            blob[8..].copy_from_slice(&value.random().rotate_left(1).to_be_bytes());
            blobs.push(blob);
        }

        chunk.write_interleaved_bytes::<16>(&blobs)?;
        Ok(())
    }
}

impl_ref_builder!(MaterialColorsBuilder, MaterialColors, MaterialColors);
impl MaterialColorsBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        for &value in &self.values {
            chunk.write_binary_string(&value.encode())?;
        }
        Ok(())
    }
}

impl_convert_builder!(
    SecurityCapabilitiesBuilder,
    SecurityCapabilities,
    i64,
    |value: &SecurityCapabilities| value.bits() as i64
);
impl SecurityCapabilitiesBuilder {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        chunk.write_interleaved_i64_array(self.values.iter().copied())
    }
}

impl_ref_builder!(ContentBuilder, Content, Content);
impl ContentBuilder<'_> {
    fn dump(&self, chunk: &mut ChunkBuilder) -> Result<(), std::io::Error> {
        let mut source_types = Vec::with_capacity(self.values.len());
        let mut uris = Vec::with_capacity(self.values.len());
        let mut objects = Vec::new();
        for (i, &value) in self.values.iter().enumerate() {
            source_types.push(match value.value() {
                ContentType::None => 0,
                ContentType::Uri(uri) => {
                    uris.push(uri.as_str());
                    1
                }
                ContentType::Object(referent) => {
                    if let Some(id) = id_to_referent.get(referent) {
                        objects.push(*id);
                    } else {
                        objects.push(-1);
                    }
                    2
                }
                _ => return Err(invalid_value(i, &Variant::Content(value.clone()))),
            });
        }
        chunk.write_interleaved_i32_array(source_types)?;

        chunk.write_le_u32(uris.len() as u32)?;
        for uri in uris {
            chunk.write_string(uri)?;
        }
        chunk.write_le_u32(objects.len() as u32)?;
        chunk.write_referent_array(objects)?;

        // If we ever need to support the external referents,
        // we will need to add it here.
        chunk.write_le_u32(0)?;
        Ok(())
    }
}
