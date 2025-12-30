use crate::{
    Attributes, Axes, BinaryString, BrickColor, CFrame, Color3, Color3uint8, ColorSequence,
    ColorSequenceKeypoint, Content, ContentId, Enum, EnumItem, Faces, Font, MaterialColors,
    NetAssetRef, NumberRange, NumberSequence, NumberSequenceKeypoint, PhysicalProperties, Ray,
    Rect, Ref, Region3, Region3int16, SecurityCapabilities, SharedString, Tags, UDim, UDim2,
    UniqueId, Vector2, Vector2int16, Vector3, Vector3int16,
};

/// Reduces boilerplate from listing different values of Variant by wrapping
/// them into a macro.
macro_rules! make_variant {
    (
        $(
            $( #[$attr:meta] )*
            $variant_name:ident ($inner_type:ty),
        )*
    ) => {
        /// Represents any Roblox type. Useful for operating generically on
        /// Roblox instances.
        ///
        /// ## Stability
        ///
        /// New variants may be added to `Variant` in minor releases. As
        /// such, it is marked `#[non_exhaustive]`.
        #[derive(Debug, Clone, PartialEq)]
        #[non_exhaustive]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
        )]
        pub enum Variant {
            $(
                $(
                    #[$attr]
                )*
                $variant_name($inner_type),
            )*
        }

        impl Variant {
            pub fn ty(&self) -> VariantType {
                match self {
                    $(
                        Variant::$variant_name(_) => VariantType::$variant_name,
                    )*
                }
            }
        }

        $(
            impl From<$inner_type> for Variant {
                fn from(value: $inner_type) -> Self {
                    Self::$variant_name(value)
                }
            }
        )*

        /// Represents any type that can be held in a `Variant`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
        )]
        pub enum VariantType {
            $(
                $variant_name,
            )*
        }

        #[cfg(test)]
        mod generated_test {
            use super::*;

            /// This test makes sure that every type represented in `Variant`
            /// can be converted via `Into` into Variant.
            ///
            /// If we forget to impl From when new types are added to Variant,
            /// this test will start failing.
            #[allow(dead_code)]
            fn conversions_are_exhaustive() {
                fn trait_test<T: Into<Variant>>() {}

                $( trait_test::<$inner_type>(); )*
                trait_test::<SharedString>();
            }
        }
    };
}

// IMPORTANT! The order of this enum is very important in order to preserve the
// discriminant values that Rust assigns for both Variant and VariantType. Any
// newly-added variants MUST be added to the end!
make_variant! {
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
    NetAssetRef(NetAssetRef),
}

impl From<&'_ str> for Variant {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl VariantType {
    pub fn fallback_default_value(self) -> Option<&'static Variant> {
        use std::sync::LazyLock;
        static DEFAULT_STRING: Variant = Variant::String(String::new());
        static DEFAULT_BINARYSTRING: Variant = Variant::BinaryString(BinaryString::new());
        static DEFAULT_BOOL: Variant = Variant::Bool(false);
        static DEFAULT_INT32: Variant = Variant::Int32(0);
        static DEFAULT_FLOAT32: Variant = Variant::Float32(0.0);
        static DEFAULT_FLOAT64: Variant = Variant::Float64(0.0);
        static DEFAULT_UDIM: Variant = Variant::UDim(UDim::new(0.0, 0));
        static DEFAULT_UDIM2: Variant =
            Variant::UDim2(UDim2::new(UDim::new(0.0, 0), UDim::new(0.0, 0)));
        static DEFAULT_RAY: Variant = Variant::Ray(Ray::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
        ));
        static DEFAULT_FACES: Variant = Variant::Faces(Faces::from_bits(0).unwrap());
        static DEFAULT_AXES: Variant = Variant::Axes(Axes::from_bits(0).unwrap());
        static DEFAULT_BRICKCOLOR: Variant = Variant::BrickColor(BrickColor::MediumStoneGrey);
        static DEFAULT_CFRAME: Variant = Variant::CFrame(CFrame::identity());
        static DEFAULT_ENUM: Variant = Variant::Enum(Enum::from_u32(u32::MAX));
        static DEFAULT_COLOR3: Variant = Variant::Color3(Color3::new(0.0, 0.0, 0.0));
        static DEFAULT_VECTOR2: Variant = Variant::Vector2(Vector2::new(0.0, 0.0));
        static DEFAULT_VECTOR3: Variant = Variant::Vector3(Vector3::new(0.0, 0.0, 0.0));
        static DEFAULT_REF: Variant = Variant::Ref(Ref::none());
        static DEFAULT_VECTOR3INT16: Variant = Variant::Vector3int16(Vector3int16::new(0, 0, 0));
        static DEFAULT_NUMBERSEQUENCE: LazyLock<Variant> = LazyLock::new(|| {
            Variant::NumberSequence(NumberSequence {
                keypoints: vec![
                    NumberSequenceKeypoint::new(0.0, 0.0, 0.0),
                    NumberSequenceKeypoint::new(0.0, 0.0, 0.0),
                ],
            })
        });
        static DEFAULT_COLORSEQUENCE: LazyLock<Variant> = LazyLock::new(|| {
            Variant::ColorSequence(ColorSequence {
                keypoints: vec![
                    ColorSequenceKeypoint::new(0.0, Color3::new(0.0, 0.0, 0.0)),
                    ColorSequenceKeypoint::new(0.0, Color3::new(0.0, 0.0, 0.0)),
                ],
            })
        });
        static DEFAULT_NUMBERRANGE: Variant = Variant::NumberRange(NumberRange::new(0.0, 0.0));
        static DEFAULT_RECT: Variant =
            Variant::Rect(Rect::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)));
        static DEFAULT_PHYSICALPROPERTIES: Variant =
            Variant::PhysicalProperties(PhysicalProperties::Default);
        static DEFAULT_COLOR3UINT8: Variant = Variant::Color3uint8(Color3uint8::new(0, 0, 0));
        static DEFAULT_INT64: Variant = Variant::Int64(0);
        static DEFAULT_SHAREDSTRING: LazyLock<Variant> =
            LazyLock::new(|| Variant::SharedString(SharedString::new(Vec::new())));
        static DEFAULT_OPTIONALCFRAME: Variant = Variant::OptionalCFrame(None);
        static DEFAULT_TAGS: Variant = Variant::Tags(Tags::new());
        static DEFAULT_CONTENTID: Variant = Variant::ContentId(ContentId::new());
        static DEFAULT_ATTRIBUTES: Variant = Variant::Attributes(Attributes::new());
        static DEFAULT_UNIQUEID: Variant = Variant::UniqueId(UniqueId::nil());
        static DEFAULT_FONT: LazyLock<Variant> = LazyLock::new(|| Variant::Font(Font::default()));
        static DEFAULT_MATERIALCOLORS: Variant = Variant::MaterialColors(MaterialColors::new());
        static DEFAULT_SECURITYCAPABILITIES: Variant =
            Variant::SecurityCapabilities(SecurityCapabilities::from_bits(0));
        static DEFAULT_CONTENT: Variant = Variant::Content(Content::none());
        static DEFAULT_NETASSETREF: LazyLock<Variant> =
            LazyLock::new(|| Variant::NetAssetRef(NetAssetRef::new(Vec::new())));
        Some(match self {
            VariantType::String => &DEFAULT_STRING,
            VariantType::BinaryString => &DEFAULT_BINARYSTRING,
            VariantType::Bool => &DEFAULT_BOOL,
            VariantType::Int32 => &DEFAULT_INT32,
            VariantType::Float32 => &DEFAULT_FLOAT32,
            VariantType::Float64 => &DEFAULT_FLOAT64,
            VariantType::UDim => &DEFAULT_UDIM,
            VariantType::UDim2 => &DEFAULT_UDIM2,
            VariantType::Ray => &DEFAULT_RAY,
            VariantType::Faces => &DEFAULT_FACES,
            VariantType::Axes => &DEFAULT_AXES,
            VariantType::BrickColor => &DEFAULT_BRICKCOLOR,
            VariantType::CFrame => &DEFAULT_CFRAME,
            VariantType::Enum => &DEFAULT_ENUM,
            VariantType::Color3 => &DEFAULT_COLOR3,
            VariantType::Vector2 => &DEFAULT_VECTOR2,
            VariantType::Vector3 => &DEFAULT_VECTOR3,
            VariantType::Ref => &DEFAULT_REF,
            VariantType::Vector3int16 => &DEFAULT_VECTOR3INT16,
            VariantType::NumberSequence => &DEFAULT_NUMBERSEQUENCE,
            VariantType::ColorSequence => &DEFAULT_COLORSEQUENCE,
            VariantType::NumberRange => &DEFAULT_NUMBERRANGE,
            VariantType::Rect => &DEFAULT_RECT,
            VariantType::PhysicalProperties => &DEFAULT_PHYSICALPROPERTIES,
            VariantType::Color3uint8 => &DEFAULT_COLOR3UINT8,
            VariantType::Int64 => &DEFAULT_INT64,
            VariantType::SharedString => &DEFAULT_SHAREDSTRING,
            VariantType::OptionalCFrame => &DEFAULT_OPTIONALCFRAME,
            VariantType::Tags => &DEFAULT_TAGS,
            VariantType::ContentId => &DEFAULT_CONTENTID,
            VariantType::Attributes => &DEFAULT_ATTRIBUTES,
            VariantType::UniqueId => &DEFAULT_UNIQUEID,
            VariantType::Font => &DEFAULT_FONT,
            VariantType::MaterialColors => &DEFAULT_MATERIALCOLORS,
            VariantType::SecurityCapabilities => &DEFAULT_SECURITYCAPABILITIES,
            VariantType::Content => &DEFAULT_CONTENT,
            VariantType::NetAssetRef => &DEFAULT_NETASSETREF,
            _ => return None,
        })
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn human() {
        let vec2 = Variant::Vector2(Vector2::new(5.0, 7.0));

        let ser = serde_json::to_string(&vec2).unwrap();
        assert_eq!(ser, r#"{"Vector2":[5.0,7.0]}"#);

        let de: Variant = serde_json::from_str(&ser).unwrap();
        assert_eq!(de, vec2);
    }

    #[test]
    fn non_human() {
        let vec2 = Variant::Vector2(Vector2::new(5.0, 7.0));

        let ser = bincode::serialize(&vec2).unwrap();

        let de: Variant = bincode::deserialize(&ser).unwrap();
        assert_eq!(de, vec2);
    }
}
