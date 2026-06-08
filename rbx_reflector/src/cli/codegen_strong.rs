use std::path::PathBuf;

use clap::Parser;
use quote::ToTokens;
use rbx_reflection::{ClassDescriptor, DataType, EnumDescriptor, ReflectionDatabase};
use rbx_types::{Variant, VariantType};

/// Generate strong types for all classes and enums.
#[derive(Debug, Parser)]
pub struct CodegenStrongSubcommand {
    /// Where to output the files.  This should be rbx_classes/src/
    pub output: PathBuf,
}

impl CodegenStrongSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let db = rbx_reflection_database::get().unwrap();

        let dest_instance = self.output.join("instances.rs");
        let dest_enum = self.output.join("enums.rs");
        let dest_macro = self.output.join("macros.rs");

        let t = std::time::Instant::now();

        // collect sorted instances
        let mut print_once = std::collections::HashSet::new();
        let instances = {
            let mut strong_instances = StrongInstancesCollector::with_capacity(db.classes.len());
            for descriptor in db.classes.values() {
                strong_instances.push(&mut print_once, descriptor, db);
            }
            strong_instances.sort()
        };

        let t_instances = t.elapsed();
        let t = std::time::Instant::now();

        // collect sorted enums
        let enums = {
            let mut strong_enum = EnumCollector::with_capacity(db.enums.len());
            for descriptor in db.enums.values() {
                strong_enum.push(descriptor);
            }
            strong_enum.sort()
        };

        let t_enums = t.elapsed();
        let t = std::time::Instant::now();

        // generate macros
        let macro_code = generate_macros(db);

        let t_macros = t.elapsed();

        // print timings
        println!("t_instances={t_instances:?}");
        println!("t_enums={t_enums:?}");
        println!("t_macros={t_macros:?}");

        // ==== generate macros.rs ====
        let macro_code = macro_code.into_token_stream().to_string();
        let macro_code = rustfmt(macro_code.as_bytes())?;

        // ==== generate instances.rs ====
        let instance_code = instances.into_token_stream().to_string();
        let instance_code = rustfmt(instance_code.as_bytes())?;

        // ==== generate enum.rs ====
        let enum_code = enums.into_token_stream().to_string();
        let enum_code = rustfmt(enum_code.as_bytes())?;

        // write destination files
        std::fs::write(dest_macro, macro_code)?;
        std::fs::write(dest_instance, instance_code)?;
        std::fs::write(dest_enum, enum_code)?;

        Ok(())
    }
}

fn new_ident(ident: &str) -> syn::Ident {
    let ident = match ident {
        // StudioScriptEditorColorCategories has this variant
        "Self" => "Self_",
        other => other,
    };
    syn::parse_str(ident).unwrap()
    // .unwrap_or_else(|_| syn::Ident::new_raw(ident, proc_macro2::Span::call_site()))
}

fn generate_data_type(data_type: &DataType) -> syn::Type {
    match data_type {
        DataType::Value(rbx_types::VariantType::Axes) => syn::parse_quote!(Axes),
        DataType::Value(rbx_types::VariantType::BinaryString) => syn::parse_quote!(BinaryString),
        DataType::Value(rbx_types::VariantType::Bool) => syn::parse_quote!(bool),
        DataType::Value(rbx_types::VariantType::BrickColor) => syn::parse_quote!(BrickColor),
        DataType::Value(rbx_types::VariantType::CFrame) => syn::parse_quote!(CFrame),
        DataType::Value(rbx_types::VariantType::Color3) => syn::parse_quote!(Color3),
        DataType::Value(rbx_types::VariantType::Color3uint8) => syn::parse_quote!(Color3uint8),
        DataType::Value(rbx_types::VariantType::ColorSequence) => syn::parse_quote!(ColorSequence),
        DataType::Value(rbx_types::VariantType::ContentId) => syn::parse_quote!(ContentId),
        DataType::Value(rbx_types::VariantType::Enum) => syn::parse_quote!(Enum),
        DataType::Value(rbx_types::VariantType::Faces) => syn::parse_quote!(Faces),
        DataType::Value(rbx_types::VariantType::Float32) => syn::parse_quote!(f32),
        DataType::Value(rbx_types::VariantType::Float64) => syn::parse_quote!(f64),
        DataType::Value(rbx_types::VariantType::Int32) => syn::parse_quote!(i32),
        DataType::Value(rbx_types::VariantType::Int64) => syn::parse_quote!(i64),
        DataType::Value(rbx_types::VariantType::NumberRange) => syn::parse_quote!(NumberRange),
        DataType::Value(rbx_types::VariantType::NumberSequence) => {
            syn::parse_quote!(NumberSequence)
        }
        DataType::Value(rbx_types::VariantType::PhysicalProperties) => {
            syn::parse_quote!(PhysicalProperties)
        }
        DataType::Value(rbx_types::VariantType::Ray) => syn::parse_quote!(Ray),
        DataType::Value(rbx_types::VariantType::Rect) => syn::parse_quote!(Rect),
        DataType::Value(rbx_types::VariantType::Ref) => syn::parse_quote!(Ref),
        DataType::Value(rbx_types::VariantType::Region3) => syn::parse_quote!(Region3),
        DataType::Value(rbx_types::VariantType::Region3int16) => syn::parse_quote!(Region3int16),
        DataType::Value(rbx_types::VariantType::SharedString) => syn::parse_quote!(SharedString),
        DataType::Value(rbx_types::VariantType::String) => syn::parse_quote!(String),
        DataType::Value(rbx_types::VariantType::UDim) => syn::parse_quote!(UDim),
        DataType::Value(rbx_types::VariantType::UDim2) => syn::parse_quote!(UDim2),
        DataType::Value(rbx_types::VariantType::Vector2) => syn::parse_quote!(Vector2),
        DataType::Value(rbx_types::VariantType::Vector2int16) => syn::parse_quote!(Vector2int16),
        DataType::Value(rbx_types::VariantType::Vector3) => syn::parse_quote!(Vector3),
        DataType::Value(rbx_types::VariantType::Vector3int16) => syn::parse_quote!(Vector3int16),
        DataType::Value(rbx_types::VariantType::OptionalCFrame) => {
            syn::parse_quote!(Option<CFrame>)
        }
        DataType::Value(rbx_types::VariantType::Tags) => syn::parse_quote!(Tags),
        DataType::Value(rbx_types::VariantType::Attributes) => syn::parse_quote!(Attributes),
        DataType::Value(rbx_types::VariantType::Font) => syn::parse_quote!(Font),
        DataType::Value(rbx_types::VariantType::UniqueId) => syn::parse_quote!(UniqueId),
        DataType::Value(rbx_types::VariantType::MaterialColors) => {
            syn::parse_quote!(MaterialColors)
        }
        DataType::Value(rbx_types::VariantType::SecurityCapabilities) => {
            syn::parse_quote!(SecurityCapabilities)
        }
        DataType::Value(rbx_types::VariantType::EnumItem) => syn::parse_quote!(EnumItem),
        DataType::Value(rbx_types::VariantType::Content) => syn::parse_quote!(Content),
        DataType::Value(rbx_types::VariantType::NetAssetRef) => syn::parse_quote!(NetAssetRef),
        // enums::name
        DataType::Enum(name) => {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            syn::parse_quote!(enums::#ident)
        }
        data_type => unimplemented!("{data_type:?}"),
    }
}

struct WrapToTokens<T>(T);
impl ToTokens for WrapToTokens<f32> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut append = |tt| tokens.extend(tt);
        let WrapToTokens(value) = self;
        match value.classify() {
            std::num::FpCategory::Nan => {
                panic!("Brother there is a NaN in Roblox default values")
            }
            std::num::FpCategory::Infinite => {
                if value.is_sign_negative() {
                    append(quote::quote! {f32::NEG_INFINITY});
                } else {
                    append(quote::quote! {f32::INFINITY});
                }
            }
            std::num::FpCategory::Zero
            | std::num::FpCategory::Subnormal
            | std::num::FpCategory::Normal => append(quote::quote! {#value}),
        }
    }
}
impl ToTokens for WrapToTokens<&rbx_types::CFrame> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut append = |tt| tokens.extend(tt);
        let &WrapToTokens(value) = self;
        if value == &rbx_types::CFrame::identity() {
            append(quote::quote! {CFrame::identity()});
            return;
        }
        let px = WrapToTokens(value.position.x);
        let py = WrapToTokens(value.position.y);
        let pz = WrapToTokens(value.position.z);
        let xx = WrapToTokens(value.orientation.x.x);
        let xy = WrapToTokens(value.orientation.x.y);
        let xz = WrapToTokens(value.orientation.x.z);
        let yx = WrapToTokens(value.orientation.y.x);
        let yy = WrapToTokens(value.orientation.y.y);
        let yz = WrapToTokens(value.orientation.y.z);
        let zx = WrapToTokens(value.orientation.z.x);
        let zy = WrapToTokens(value.orientation.z.y);
        let zz = WrapToTokens(value.orientation.z.z);
        append(
            quote::quote! {CFrame::new(Vector3::new(#px,#py,#pz),Matrix3::new(Vector3::new(#xx,#xy,#xz),Vector3::new(#yx,#yy,#yz),Vector3::new(#zx,#zy,#zz)))},
        );
    }
}

impl ToTokens for WrapToTokens<&rbx_types::Color3> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut append = |tt| tokens.extend(tt);
        let &WrapToTokens(value) = self;
        let r = WrapToTokens(value.r);
        let g = WrapToTokens(value.g);
        let b = WrapToTokens(value.b);
        append(quote::quote! {Color3::new(#r,#g,#b)});
    }
}
impl ToTokens for WrapToTokens<&rbx_types::Color3uint8> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut append = |tt| tokens.extend(tt);
        let &WrapToTokens(rbx_types::Color3uint8 { r, g, b }) = self;
        append(quote::quote! {Color3uint8::new(#r,#g,#b)});
    }
}

impl ToTokens for WrapToTokens<rbx_types::FontWeight> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let &WrapToTokens(value) = self;
        let mut append = |tt| tokens.extend(tt);
        match value {
            rbx_types::FontWeight::Thin => append(quote::quote!(FontWeight::Thin)),
            rbx_types::FontWeight::ExtraLight => append(quote::quote!(FontWeight::ExtraLight)),
            rbx_types::FontWeight::Light => append(quote::quote!(FontWeight::Light)),
            rbx_types::FontWeight::Regular => append(quote::quote!(FontWeight::Regular)),
            rbx_types::FontWeight::Medium => append(quote::quote!(FontWeight::Medium)),
            rbx_types::FontWeight::SemiBold => append(quote::quote!(FontWeight::SemiBold)),
            rbx_types::FontWeight::Bold => append(quote::quote!(FontWeight::Bold)),
            rbx_types::FontWeight::ExtraBold => append(quote::quote!(FontWeight::ExtraBold)),
            rbx_types::FontWeight::Heavy => append(quote::quote!(FontWeight::Heavy)),
            _ => unimplemented!(),
        }
    }
}
impl ToTokens for WrapToTokens<rbx_types::FontStyle> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let &WrapToTokens(value) = self;
        let mut append = |tt| tokens.extend(tt);
        match value {
            rbx_types::FontStyle::Normal => append(quote::quote!(FontStyle::Normal)),
            rbx_types::FontStyle::Italic => append(quote::quote!(FontStyle::Italic)),
            _ => unimplemented!(),
        }
    }
}
impl ToTokens for WrapToTokens<&Variant> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::quote as q;
        let mut append = |tt| tokens.extend(tt);
        use rbx_types::*;
        match &self.0 {
            Variant::String(value) => tokens.extend(q! {#value.to_owned()}),
            Variant::BinaryString(value) => {
                let lit = syn::LitByteStr::new(value.as_ref(), proc_macro2::Span::call_site());
                append(q! {#lit.as_slice().into()});
            }
            Variant::Bool(value) => append(q! {#value}),
            Variant::Int32(value) => append(q! {#value}),
            Variant::Float32(value) => {
                let value = WrapToTokens(*value);
                append(q! {#value});
            }
            Variant::Float64(value) => append(q! {#value}),
            Variant::UDim(UDim { scale, offset }) => {
                append(q! {UDim::new(#scale,#offset)});
            }
            Variant::UDim2(value) => {
                let sx = WrapToTokens(value.x.scale);
                let sy = WrapToTokens(value.y.scale);
                let ox = value.x.offset;
                let oy = value.y.offset;
                append(q! {UDim2::new(UDim::new(#sx,#ox),UDim::new(#sy,#oy))});
            }
            Variant::Ray(value) => {
                let ox = WrapToTokens(value.origin.x);
                let oy = WrapToTokens(value.origin.y);
                let oz = WrapToTokens(value.origin.z);
                let dx = WrapToTokens(value.direction.x);
                let dy = WrapToTokens(value.direction.y);
                let dz = WrapToTokens(value.direction.z);
                append(q! {Ray::new(Vector3::new(#ox,#oy,#oz),Vector3::new(#dx,#dy,#dz))});
            }
            Variant::Faces(value) => {
                if value == &Faces::all() {
                    append(q! {Faces::all()});
                } else if value == &Faces::empty() {
                    append(q! {Faces::empty()});
                } else {
                    // TODO: use iter_names()
                    let mut faces: Vec<syn::Ident> = Vec::with_capacity(6);
                    if value.contains(Faces::RIGHT) {
                        faces.push(syn::parse_str("RIGHT").unwrap());
                    }
                    if value.contains(Faces::TOP) {
                        faces.push(syn::parse_str("TOP").unwrap());
                    }
                    if value.contains(Faces::BACK) {
                        faces.push(syn::parse_str("BACK").unwrap());
                    }
                    if value.contains(Faces::LEFT) {
                        faces.push(syn::parse_str("LEFT").unwrap());
                    }
                    if value.contains(Faces::BOTTOM) {
                        faces.push(syn::parse_str("BOTTOM").unwrap());
                    }
                    if value.contains(Faces::FRONT) {
                        faces.push(syn::parse_str("FRONT").unwrap());
                    }
                    let faces = faces.into_iter();
                    append(q! {#(Faces::#faces)|*});
                }
            }
            Variant::Axes(value) => {
                if value == &Axes::all() {
                    append(q! {Axes::all()});
                } else if value == &Axes::empty() {
                    append(q! {Axes::empty()});
                } else {
                    // TODO: use iter_names()
                    let mut axes: Vec<syn::Ident> = Vec::with_capacity(6);
                    if value.contains(Axes::X) {
                        axes.push(syn::parse_str("X").unwrap());
                    }
                    if value.contains(Axes::Y) {
                        axes.push(syn::parse_str("Y").unwrap());
                    }
                    if value.contains(Axes::Z) {
                        axes.push(syn::parse_str("Z").unwrap());
                    }
                    let axes = axes.into_iter();
                    append(q! {#(Axes::#axes)|*});
                }
            }
            Variant::BrickColor(value) => {
                let number: u16 = *value as u16;
                append(q! {BrickColor::from_number(#number).unwrap()});
            }
            Variant::CFrame(value) => {
                let value = WrapToTokens(value);
                append(q! {#value});
            }
            Variant::Enum(_) => {
                unreachable!("Enums are handled by mathing the DataType");
            }
            Variant::Color3(value) => {
                let value = WrapToTokens(value);
                append(q! {#value});
            }
            Variant::Vector2(value) => {
                let x = WrapToTokens(value.x);
                let y = WrapToTokens(value.y);
                append(q! {Vector2::new(#x,#y)});
            }
            Variant::Vector3(value) => {
                let x = WrapToTokens(value.x);
                let y = WrapToTokens(value.y);
                let z = WrapToTokens(value.z);
                append(q! {Vector3::new(#x,#y,#z)});
            }
            Variant::Ref(value) => {
                if value.is_some() {
                    panic!("Cannot create default Ref");
                }
                append(q! {Ref::none()});
            }
            Variant::Vector3int16(value) => {
                let x = value.x;
                let y = value.y;
                let z = value.z;
                append(q! {Vector3int16::new(#x,#y,#z)});
            }
            Variant::NumberSequence(value) => {
                let times = value.keypoints.iter().map(|keypoint| keypoint.time);
                let values = value.keypoints.iter().map(|keypoint| keypoint.value);
                let envelopes = value.keypoints.iter().map(|keypoint| keypoint.envelope);
                append(
                    q! {NumberSequence{keypoints:vec![#(NumberSequenceKeypoint::new(#times,#values,#envelopes)),*]}},
                );
            }
            Variant::ColorSequence(value) => {
                let times = value.keypoints.iter().map(|keypoint| keypoint.time);
                let colors = value
                    .keypoints
                    .iter()
                    .map(|keypoint| WrapToTokens(&keypoint.color));
                append(
                    q! {ColorSequence{keypoints:vec![#(ColorSequenceKeypoint::new(#times,#colors)),*]}},
                );
            }
            Variant::NumberRange(value) => {
                let min = WrapToTokens(value.min);
                let max = WrapToTokens(value.max);
                append(q! {NumberRange::new(#min,#max)});
            }
            Variant::Rect(value) => {
                let min_x = WrapToTokens(value.min.x);
                let min_y = WrapToTokens(value.min.y);
                let max_x = WrapToTokens(value.max.x);
                let max_y = WrapToTokens(value.max.y);
                append(q! {Rect::new(Vector2::new(#min_x,#min_y),Vector2::new(#max_x,#max_y))});
            }
            Variant::PhysicalProperties(value) => match value {
                rbx_types::PhysicalProperties::Custom(value) => {
                    let density = WrapToTokens(value.density());
                    let friction = WrapToTokens(value.friction());
                    let elasticity = WrapToTokens(value.elasticity());
                    let friction_weight = WrapToTokens(value.friction_weight());
                    let elasticity_weight = WrapToTokens(value.elasticity_weight());
                    let acoustic_absorption = WrapToTokens(value.acoustic_absorption());
                    append(
                        q! {PhysicalProperties::Custom(CustomPhysicalProperties::new(#density,#friction,#elasticity,#friction_weight,#elasticity_weight,#acoustic_absorption))},
                    );
                }
                rbx_types::PhysicalProperties::Default => append(q! {PhysicalProperties::Default}),
            },
            Variant::Color3uint8(value) => {
                let value = WrapToTokens(value);
                append(q! {#value});
            }
            Variant::Int64(value) => append(q! {#value}),
            Variant::SharedString(value) => {
                let lit = syn::LitByteStr::new(value.data(), proc_macro2::Span::call_site());
                append(q! {SharedString::new(#lit.to_vec())});
            }
            Variant::OptionalCFrame(value) => match value {
                Some(value) => {
                    let value = WrapToTokens(value);
                    append(q! {Some(#value)});
                }
                None => append(q! {None}),
            },
            Variant::Tags(value) => {
                if value.is_empty() {
                    append(q! {Tags::new()});
                } else {
                    let values = value.iter();
                    append(q! {vec![#(#values),*].into() });
                }
            }
            Variant::ContentId(value) => {
                let lit = value.as_str();
                append(q! {#lit.into()});
            }
            Variant::Attributes(value) => {
                if value.is_empty() {
                    append(q! {Attributes::new()});
                } else {
                    let iter_k = value.iter().map(|(k, _)| k.as_str());
                    let iter_v = value.iter().map(|(_, v)| WrapToTokens(v));
                    append(q! {
                        Attributes::from_iter([
                            #((#iter_k.to_owned(),#iter_v.into())),*
                        ])
                    })
                }
            }
            Variant::UniqueId(value) => {
                if !value.is_nil() {
                    panic!("Cannot create default UniqueId");
                }
                append(q! {UniqueId::nil()});
            }
            Variant::Font(value) => {
                let family = value.family.as_str();
                let weight = WrapToTokens(value.weight);
                let style = WrapToTokens(value.style);
                let cached_face_id = value.cached_face_id.as_deref();
                match cached_face_id {
                    Some(cached_face_id) => append(q! {
                        Font {
                            family: #family.to_owned(),
                            weight: #weight,
                            style: #style,
                            cached_face_id: Some(#cached_face_id.to_owned()),
                        }
                    }),
                    None => append(q! {Font::new(#family,#weight,#style)}),
                }
            }
            Variant::MaterialColors(value) => {
                if value.is_empty() {
                    append(q! {MaterialColors::new()});
                } else {
                    unimplemented!("non-default MaterialColors");
                }
            }
            Variant::SecurityCapabilities(value) => {
                let bits = value.bits();
                append(q! {SecurityCapabilities::from_bits(#bits)})
            }
            Variant::Content(value) => match value.value() {
                ContentType::None => append(q! {Content::none()}),
                ContentType::Uri(uri) => append(q! {Content::from_uri(#uri)}),
                ContentType::Object(_) => panic!("Cannot create Object Content"),
                _ => panic!(),
            },
            Variant::NetAssetRef(value) => {
                let lit = syn::LitByteStr::new(value.data(), proc_macro2::Span::call_site());
                append(q! {NetAssetRef::new(#lit.to_vec())});
            }
            variant => unimplemented!("{variant:?}"),
        }
    }
}

struct FieldInfo {
    ident: syn::Ident,
    field: syn::Field,
    default_value: syn::FieldValue,
}
fn get_fields_info<'db>(
    print_once: &mut std::collections::HashSet<&'db str>,
    descriptor: &'db ClassDescriptor<'db>,
    default_properties: &std::collections::HashMap<&'db str, Variant>,
    database: &'db ReflectionDatabase<'db>,
) -> Vec<FieldInfo> {
    // generate fields
    let mut fields = Vec::new();
    for prop in descriptor.properties.values() {
        match &prop.kind {
            rbx_reflection::PropertyKind::Canonical {
                serialization:
                    rbx_reflection::PropertySerialization::Serializes
                    | rbx_reflection::PropertySerialization::SerializesAs(_),
            } => (),
            // skip properties which are migrated or serialize as something else
            _ => continue,
        }
        let field_name = heck::ToUpperCamelCase::to_upper_camel_case(prop.name);
        let ident = syn::Ident::new(&field_name, proc_macro2::Span::call_site());
        let ty = generate_data_type(&prop.data_type);
        let field = syn::Field {
            attrs: Vec::new(),
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            mutability: syn::FieldMutability::None,
            ident: Some(ident.clone()),
            colon_token: Some(syn::token::Colon::default()),
            ty,
        };

        let default_value_variant = default_properties
            .get(prop.name)
            .unwrap_or_else(|| prop.data_type.ty().fallback_default_value().unwrap());
        let default_value: syn::FieldValue = match &prop.data_type {
            &DataType::Value(expected_type) => match (default_value_variant, expected_type) {
                (actual, expected) if actual.ty() == expected => {
                    let value = WrapToTokens(default_value_variant);
                    syn::parse_quote! {
                        #ident: #value
                    }
                }
                // Special case for incorrect default value type in BasePart.Color
                (Variant::Color3uint8(color3uint8), VariantType::Color3) => {
                    let value = WrapToTokens(color3uint8);
                    syn::parse_quote! {
                        #ident: #value.into()
                    }
                }
                _ => panic!("Default value type does not match prop type!"),
            },
            DataType::Enum(enum_name) => {
                let Variant::Enum(value) = default_value_variant else {
                    panic!("Data type is Enum but default value is not Enum");
                };

                let find_value = value.to_u32();

                let enum_descriptor = database
                    .enums
                    .get(enum_name)
                    .expect("Expected enum name to exist");

                let enum_variant_name = if let Some((enum_variant_name, _)) = enum_descriptor
                    .items
                    .iter()
                    .find(|&(_, &v)| v == find_value)
                {
                    enum_variant_name
                } else {
                    let (enum_variant_name, &enum_variant_value) = enum_descriptor
                        .items
                        .iter()
                        .min_by_key(|&(_, &v)| v)
                        .expect("Expected enum to have more than 0 variants");
                    if print_once.insert(enum_variant_name) {
                        println!(
                            "Enum {enum_name} discriminant not found {find_value} for class {}. Inventing a default value: {enum_variant_name} = {enum_variant_value}",
                            prop.name
                        );
                    }
                    enum_variant_name
                };

                let enum_name = syn::Ident::new(enum_name, proc_macro2::Span::call_site());
                let variant_name = new_ident(enum_variant_name);

                syn::parse_quote! {
                    #ident: enums::#enum_name::#variant_name
                }
            }
            _ => unimplemented!(),
        };

        fields.push(FieldInfo {
            ident,
            field,
            default_value,
        });
    }
    // sort props for consistency
    fields.sort_by(|a, b| a.ident.cmp(&b.ident));

    fields
}

struct Sorted<T>(T);

// A struct with impls trailing under it.
// Sorted together so the impls stay postfixed to the struct.
struct StructWithImpls {
    item: syn::ItemStruct,
    impls: Vec<syn::Item>,
}
struct StrongInstancesCollector {
    structs: Vec<StructWithImpls>,
}
impl StrongInstancesCollector {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            structs: Vec::with_capacity(capacity),
        }
    }
    fn push<'db>(
        &mut self,
        print_once: &mut std::collections::HashSet<&'db str>,
        descriptor: &'db ClassDescriptor<'db>,
        database: &'db ReflectionDatabase<'db>,
    ) {
        let fields = get_fields_info(
            print_once,
            descriptor,
            &descriptor.default_properties,
            database,
        );

        // struct ident
        let ident = syn::Ident::new(descriptor.name, proc_macro2::Span::call_site());

        // add a superclass field if this instance has a superclass
        let superclass_ident: Option<syn::Ident> = descriptor
            .superclass
            .as_ref()
            .map(|superclass| syn::parse_str(superclass).unwrap());

        // A Vec of impls for this particular instance.
        // this way the impls always stay with the struct
        // even after the structs are sorted
        let mut impls = Vec::new();
        if let Some(superclass_ident) = &superclass_ident {
            impls.push(syn::parse_quote! {
                impl_inherits!(#ident, #superclass_ident);
            });
        }

        // Attributes
        let mut attrs: Vec<syn::Attribute> = syn::parse_quote!(
            #[derive(Debug, Clone)]
            #[allow(nonstandard_style)]
        );

        // Simply derive default if there are no fields
        if fields.is_empty() {
            attrs.push(syn::parse_quote!(#[derive(Default)]));
        } else {
            // generate impl Default
            let mut superclasses: Vec<_> = database.superclasses_iter(descriptor).skip(1).collect();
            superclasses.reverse();

            let mut iter = superclasses.into_iter().map(|class| {
                let ident = syn::parse_str(class.name).unwrap();
                // this is duplicating a lot of work, but whatever
                let fields =
                    get_fields_info(print_once, class, &descriptor.default_properties, database);
                (OtherIdent(ident), fields)
            });

            // Some crap needed because Self isn't a valid ident
            struct OtherIdent(syn::Ident);
            struct SelfIdent;
            impl From<OtherIdent> for syn::ExprStruct {
                fn from(OtherIdent(ident): OtherIdent) -> Self {
                    syn::parse_quote! {#ident{}}
                }
            }
            impl From<SelfIdent> for syn::ExprStruct {
                fn from(_: SelfIdent) -> Self {
                    syn::parse_quote! {Self{}}
                }
            }

            fn push_fields(
                (ident, fields): (impl Into<syn::ExprStruct>, impl AsRef<[FieldInfo]>),
            ) -> syn::ExprStruct {
                let mut default_struct = ident.into();
                default_struct.fields.push(syn::parse_quote! {
                    superclass
                });
                default_struct.fields.extend(
                    fields
                        .as_ref()
                        .iter()
                        .map(|field_info| &field_info.default_value)
                        .cloned(),
                );
                default_struct
            }

            // root superclass has no superclass
            let (OtherIdent(root), _) = iter.next().unwrap();

            // superclasses
            let iter = iter.map(push_fields);

            // self
            let default_struct = push_fields((SelfIdent, &fields));

            let default_impl = syn::parse_quote! {
                impl Default for #ident{
                    fn default() -> Self {
                        let superclass = #root::default();
                        #(let superclass = #iter;)*
                        #default_struct
                    }
                }
            };
            impls.push(default_impl);
        }

        // superclass field is added to the top
        let superclass_field = superclass_ident.map(|superclass_ident| {
            syn::parse_quote! {
                #[doc(hidden)]
                pub superclass: #superclass_ident
            }
        });

        // generate the class struct
        let mut item: syn::ItemStruct = syn::parse_quote! {
            pub struct #ident {}
        };
        item.attrs = attrs;
        item.fields = syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: superclass_field
                .into_iter()
                .chain(fields.into_iter().map(|field_info| field_info.field))
                .collect(),
        });
        self.structs.push(StructWithImpls { item, impls });
    }
    fn sort(mut self) -> Sorted<Self> {
        // sort for consistency
        self.structs.sort_by(|a, b| a.item.ident.cmp(&b.item.ident));
        Sorted(self)
    }
}
impl ToTokens for Sorted<StrongInstancesCollector> {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        unimplemented!();
    }
    fn into_token_stream(self) -> proc_macro2::TokenStream {
        let Sorted(instances) = self;

        let iter = instances
            .structs
            .into_iter()
            .flat_map(|StructWithImpls { item, impls }| {
                std::iter::once(syn::Item::Struct(item)).chain(impls)
            });

        // create complete file including use statements
        quote::quote! {
            use core::ops::{Deref, DerefMut};
            use super::enums;
            use rbx_types::*;
            macro_rules! impl_inherits {
                ($class:path, $inherits:path) => {
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
            #(#iter)*
        }
    }
}

struct EnumCollector {
    enums: Vec<syn::ItemEnum>,
}
impl EnumCollector {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            enums: Vec::with_capacity(capacity),
        }
    }
    fn push(&mut self, descriptor: &EnumDescriptor) {
        // collect enum items
        let mut items: Vec<_> = descriptor
            .items
            .iter()
            .map(|(name, &value)| (name, value))
            .collect();

        // sort variants by discriminant for consistency
        items.sort_by_key(|&(_, value)| value);

        // generate fields
        let variants = items.into_iter().map(|(name, value)| {
            let ident = new_ident(name);
            let discriminant = syn::parse_str(&value.to_string()).unwrap();
            syn::Variant {
                attrs: Vec::new(),
                ident,
                fields: syn::Fields::Unit,
                discriminant: Some((syn::token::Eq::default(), discriminant)),
            }
        });

        // enum ident
        let ident = syn::Ident::new(descriptor.name, proc_macro2::Span::call_site());

        // generate the enum
        let mut item_enum: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            #[allow(nonstandard_style)]
            pub enum #ident {}
        };
        item_enum.variants.extend(variants);
        self.enums.push(item_enum);
    }
    fn sort(mut self) -> Sorted<Self> {
        // sort for consistency
        self.enums.sort_by(|a, b| a.ident.cmp(&b.ident));
        Sorted(self)
    }
}
impl ToTokens for Sorted<EnumCollector> {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        unimplemented!();
    }
    fn into_token_stream(self) -> proc_macro2::TokenStream {
        let Sorted(enums) = self;

        // create complete file including use statements
        let iter = enums.enums.into_iter();
        quote::quote! {#(#iter)*}
    }
}

fn generate_macros(database: &ReflectionDatabase<'_>) -> proc_macro2::TokenStream {
    let mut enums: Vec<_> = database
        .enums
        .values()
        .map(|e| syn::parse_str::<syn::Ident>(e.name).unwrap())
        .collect();
    enums.sort();
    let mut classes: Vec<_> = database
        .classes
        .values()
        .map(|c| (c, syn::parse_str::<syn::Ident>(c.name).unwrap()))
        .collect();
    classes.sort_by_key(|(c, _)| c.name);

    // generate enums macro
    let iter = enums.into_iter();
    let for_each_enum_macro: syn::ItemMacro = syn::parse_quote! {
        /// Invoke a macro with every enum ident,
        /// i.e
        /// ```rust,ignore
        /// for_each_enum!(my_macro);
        /// ```
        /// invokes
        /// ```rust,ignore
        /// my_macro!(AlignType, FormFactor, KeyCode, ...)
        /// ```
        #[macro_export]
        macro_rules! for_each_enum {
            ($my_macro:ident) => {
                $my_macro!(#(#iter),*);
            };
        }
    };

    // generate StrongInstances macro
    let iter = classes.iter().map(|(_, i)| i);
    let for_each_class_macro: syn::ItemMacro = syn::parse_quote! {
        /// Invoke a macro with every class ident,
        /// i.e
        /// ```rust,ignore
        /// for_each_class!(my_macro);
        /// ```
        /// invokes
        /// ```rust,ignore
        /// my_macro!(Accoutrement, Part, WedgePart, ...);
        /// ```
        #[macro_export]
        macro_rules! for_each_class {
            ($my_macro:ident) => {
                $my_macro!(#(#iter),*);
            };
        }
    };

    // generate instance descendants macro
    let iter = classes.iter().map(|&(c, ref ident)| {
        // generate (BasePart, [Part, WedgePart, BasePart, ...])
        let iter = classes.iter().filter_map(|&(descendant, ref desc_ident)| {
            database.has_superclass(descendant, c).then_some(desc_ident)
        });
        quote::quote! {(#ident, [#(#iter),*])}
    });
    let for_each_class_descendants_macro: syn::ItemMacro = syn::parse_quote! {
        /// Invoke a macro with every class ident, and a list of its descendants
        /// i.e
        /// ```rust,ignore
        /// for_each_class_descendants!(my_macro);
        /// ```
        /// invokes
        /// ```rust,ignore
        /// my_macro!((BasePart, [BasePart, Part, WedgePart, ...]), ...);
        /// ```
        #[macro_export]
        macro_rules! for_each_class_descendants {
            ($my_macro:ident) => {
                $my_macro!(#(#iter),*);
            };
        }
    };

    quote::quote! {
        #for_each_enum_macro
        #for_each_class_macro
        #for_each_class_descendants_macro
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum FormatError {
    Io(std::io::Error),
    FormatFailed,
}
impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for FormatError {}
fn rustfmt(code: &[u8]) -> Result<Vec<u8>, FormatError> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let cmd = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(FormatError::Io)?;
    cmd.stdin
        .as_ref()
        .unwrap()
        .write_all(code)
        .map_err(FormatError::Io)?;
    let output = cmd.wait_with_output().map_err(FormatError::Io)?;

    if !output.status.success() {
        return Err(FormatError::FormatFailed);
    }

    Ok(output.stdout)
}
