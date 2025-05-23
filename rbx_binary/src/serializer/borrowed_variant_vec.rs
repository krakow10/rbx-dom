use rbx_dom_weak::types::{Variant, VariantType};
use rbx_dom_weak::types::{
    Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
    Content, ContentId, Enum, EnumItem, Faces, Font, MaterialColors, NumberRange, NumberSequence,
    PhysicalProperties, Ray, Rect, Ref, Region3, Region3int16, SecurityCapabilities, SharedString,
    Tags, UDim, UDim2, UniqueId, Vector2, Vector2int16, Vector3, Vector3int16,
};
macro_rules! impl_borrowed_variant_vec {
    ($($variant:ident($type:ty),)*) => {
        // use rbx_dom_weak::types::$type;
        #[derive(Debug)]
        pub enum BorrowedVariantVec<'a>{
            $(
                $variant(Vec<&'a $type>),
            )*
        }
        impl<'a> BorrowedVariantVec<'a>{
            pub fn new(variant_type:VariantType) -> Self {
                match variant_type{
                    $(
                        VariantType::$variant => BorrowedVariantVec::$variant(Vec::new()),
                    )*
                    _=>panic!("Unknown VariantType {:?}", variant_type),
                }
            }
            pub fn push(&mut self, variant: &'a Variant) {
                match (variant, self) {
                    $(
                        (Variant::$variant(value), BorrowedVariantVec::$variant(values)) => values.push(value),
                    )*
                    _=>panic!("Variant does not match {:?}", variant.ty()),
                }
            }
        }
    };
}

impl_borrowed_variant_vec! {
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
    Region3(Region3),
    Region3int16(Region3int16),
    SharedString(SharedString),
    String(String),
    UDim(UDim),
    UDim2(UDim2),
    Vector2(Vector2),
    Vector2int16(Vector2int16),
    Vector3(Vector3),
    Vector3int16(Vector3int16),
    OptionalCFrame(Option<CFrame>),
    Tags(Tags),
    Attributes(Attributes),
    Font(Font),
    UniqueId(UniqueId),
    MaterialColors(MaterialColors),
    SecurityCapabilities(SecurityCapabilities),
    EnumItem(EnumItem),
    Content(Content),
}
