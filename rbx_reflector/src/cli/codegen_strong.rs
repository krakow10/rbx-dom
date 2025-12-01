use std::path::PathBuf;

use clap::Parser;
use rbx_reflection::{ClassDescriptor, DataType, EnumDescriptor};
use rbx_types::Variant;

/// Generate strong types for all classes and enums.
#[derive(Debug, Parser)]
pub struct CodegenStrongSubcommand {
    /// Where to output the files.  This should be rbx_types_strong/src/
    pub output: PathBuf,
}

impl CodegenStrongSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        use quote::ToTokens;
        let db = rbx_reflection_database::get().unwrap();

        let dest_instance = self.output.join("instances.rs");
        let dest_enum = self.output.join("enums.rs");

        // ==== generate instances.rs ====
        let instance_code = {
            let mut strong_instances = StrongInstancesCollector::new();
            for descriptor in db.classes.values() {
                strong_instances.push(descriptor);
            }
            let complete_file = strong_instances.sort().codegen();

            // make a string of the unformatted code
            let code = complete_file.into_token_stream().to_string();

            // format via cli
            let code = rustfmt(code.as_bytes())?;

            code
        };
        // ==== generate enum.rs ====
        let enum_code = {
            let mut strong_enum = EnumCollector::new();
            for descriptor in db.enums.values() {
                strong_enum.push(descriptor);
            }
            let complete_file = strong_enum.sort().codegen();

            // make a string of the unformatted code
            let code = complete_file.into_token_stream().to_string();

            // format via cli
            let code = rustfmt(code.as_bytes())?;

            code
        };

        // save to destination file
        std::fs::write(dest_instance, instance_code)?;
        std::fs::write(dest_enum, enum_code)?;
        Ok(())
    }
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
impl quote::ToTokens for WrapToTokens<f32> {
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
impl quote::ToTokens for WrapToTokens<&rbx_types::CFrame> {
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
impl quote::ToTokens for WrapToTokens<&Variant> {
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
            Variant::Faces(value) => append(q! {unimplemented!()}),
            Variant::Axes(value) => append(q! {unimplemented!()}),
            Variant::BrickColor(value) => {
                let number: u16 = *value as u16;
                append(q! {BrickColor::from_number(#number).unwrap()});
            }
            Variant::CFrame(value) => {
                let value = WrapToTokens(value);
                append(q! {#value});
            }
            Variant::Enum(value) => {
                append(q! {unimplemented!("convert u32 Enum to precise strong variant")})
            }
            Variant::Color3(value) => {
                let r = WrapToTokens(value.r);
                let g = WrapToTokens(value.g);
                let b = WrapToTokens(value.b);
                append(q! {Color3::new(#r,#g,#b)});
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
            Variant::NumberSequence(value) => append(q! {unimplemented!("NumberSequence")}),
            Variant::ColorSequence(value) => append(q! {unimplemented!("ColorSequence")}),
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
                let r = value.r;
                let g = value.g;
                let b = value.b;
                append(q! {Color3uint8::new(#r,#g,#b)});
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
            Variant::Attributes(value) => append(q! {unimplemented!("Attributes")}),
            Variant::UniqueId(value) => {
                if !value.is_nil() {
                    panic!("Cannot create default UniqueId");
                }
                append(q! {UniqueId::nil()});
            }
            Variant::Font(value) => append(q! {unimplemented!("Font")}),
            Variant::MaterialColors(value) => append(q! {unimplemented!("MaterialColors")}),
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

struct Sorted<T>(T);

// A struct with impls trailing under it.
// Sorted together so the impls stay postfixed to the struct.
struct StructWithImpls {
    item: syn::ItemStruct,
    impls: Vec<syn::Item>,
}
struct StrongInstancesCollector {
    structs: Vec<StructWithImpls>,
    variants: Vec<syn::Variant>,
}
impl StrongInstancesCollector {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
            variants: Vec::new(),
        }
    }
    fn push(&mut self, descriptor: &ClassDescriptor) {
        struct FieldInfo<'a> {
            prop: &'a rbx_reflection::PropertyDescriptor<'a>,
            ident: syn::Ident,
            field: syn::Field,
        }
        // generate fields
        let mut fields = Vec::new();
        for prop in descriptor.properties.values() {
            match &prop.kind {
                rbx_reflection::PropertyKind::Canonical {
                    serialization: rbx_reflection::PropertySerialization::Serializes,
                } => (),
                // skip properties which are migrated or serialize as something else
                _ => continue,
            }
            let field_name = heck::ToUpperCamelCase::to_upper_camel_case(prop.name.as_ref());
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
            fields.push(FieldInfo { prop, ident, field });
        }
        // sort props for consistency
        fields.sort_by(|a, b| a.ident.cmp(&b.ident));

        // struct ident
        let ident = syn::Ident::new(&descriptor.name, proc_macro2::Span::call_site());

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
        impls.push(syn::parse_quote! {
            impl_strong_instance_from!(#ident);
        });

        // Attributes
        let mut attrs: Vec<syn::Attribute> = syn::parse_quote!(
            #[derive(Debug, Clone)]
            #[allow(nonstandard_style)]
        );

        // Simply derive default if there are no fields
        if fields.is_empty() {
            attrs.push(syn::parse_quote!(#[derive(Default)]));
        } else {
            let mut default_struct: syn::ExprStruct = syn::parse_quote! {
                Self{
                }
            };
            // superclass default value
            if let Some(superclass_ident) = &superclass_ident {
                default_struct.fields.push(syn::parse_quote! {
                    superclass: #superclass_ident::default()
                });
            }
            default_struct
                .fields
                .extend(fields.iter().map(|FieldInfo { prop, ident, .. }| {
                    let value = descriptor
                        .default_properties
                        .get(prop.name.as_ref())
                        .unwrap_or_else(|| prop.data_type.ty().fallback_default_value().unwrap());
                    let value = WrapToTokens(value);
                    let field: syn::FieldValue = syn::parse_quote! {
                        #ident: #value
                    };
                    field
                }));
            let default_impl = syn::parse_quote! {
                impl Default for #ident{
                    fn default() -> Self {
                        #default_struct
                    }
                }
            };
            impls.push(default_impl);
        }

        // superclass field is added to the top
        let superclass_field = superclass_ident.map(|superclass_ident| syn::Field {
            attrs: Vec::new(),
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(syn::parse_quote!(superclass)),
            colon_token: Some(syn::token::Colon::default()),
            ty: syn::parse_quote!(#superclass_ident),
        });

        // generate the class struct
        self.structs.push(StructWithImpls {
            item: syn::ItemStruct {
                attrs,
                vis: syn::Visibility::Public(syn::token::Pub::default()),
                struct_token: syn::token::Struct::default(),
                ident: ident.clone(),
                generics: syn::Generics::default(),
                fields: syn::Fields::Named(syn::FieldsNamed {
                    brace_token: syn::token::Brace::default(),
                    named: superclass_field
                        .into_iter()
                        .chain(fields.into_iter().map(|field_info| field_info.field))
                        .collect(),
                }),
                semi_token: None,
            },
            impls,
        });

        // generate the StrongInstances variant
        self.variants.push(syn::parse_quote!(#ident(Box<#ident>)));
    }
    fn sort(mut self) -> Sorted<Self> {
        // sort for consistency
        self.structs.sort_by(|a, b| a.item.ident.cmp(&b.item.ident));
        self.variants.sort_by(|a, b| a.ident.cmp(&b.ident));
        Sorted(self)
    }
}
impl Sorted<StrongInstancesCollector> {
    fn codegen(self) -> syn::File {
        let Sorted(instances) = self;
        // generate StrongInstance enum
        let mut strong_instances_enum: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            #[non_exhaustive]
            pub enum StrongInstance {
            }
        };
        strong_instances_enum.variants.extend(instances.variants);

        // create complete file including use statements
        let mut complete_file: syn::File = syn::parse_quote! {
            use core::ops::{Deref, DerefMut};
            use crate::{impl_inherits, impl_strong_instance_from};
            use super::enums;
            use rbx_types::*;
        };
        complete_file
            .items
            .push(syn::Item::Enum(strong_instances_enum));
        complete_file
            .items
            .extend(
                instances
                    .structs
                    .into_iter()
                    .flat_map(|StructWithImpls { item, impls }| {
                        std::iter::once(syn::Item::Struct(item)).chain(impls)
                    }),
            );

        complete_file
    }
}

fn fix_enum_ident(ident: &str) -> &str {
    match ident {
        // StudioScriptEditorColorCategories has this variant
        "Self" => "Self_",
        other => other,
    }
}

struct EnumCollector {
    enums: Vec<syn::ItemEnum>,
    variants: Vec<syn::Variant>,
}
impl EnumCollector {
    fn new() -> Self {
        Self {
            enums: Vec::new(),
            variants: Vec::new(),
        }
    }
    fn push(&mut self, descriptor: &EnumDescriptor) {
        // collect enum items
        let mut items: Vec<_> = descriptor
            .items
            .iter()
            .map(|(name, &value)| (fix_enum_ident(name), value))
            .collect();

        // sort variants by discriminant for consistency
        items.sort_by_key(|&(_, value)| value);

        // generate fields
        let variants = items.into_iter().map(|(name, value)| {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            let discriminant = syn::parse_str(&value.to_string()).unwrap();
            syn::Variant {
                attrs: Vec::new(),
                ident,
                fields: syn::Fields::Unit,
                discriminant: Some((syn::token::Eq::default(), discriminant)),
            }
        });

        // enum ident
        let ident = syn::Ident::new(&descriptor.name, proc_macro2::Span::call_site());

        // generate the enum
        self.enums.push(syn::ItemEnum {
            attrs: syn::parse_quote! {
                #[derive(Debug, Clone)]
                #[allow(nonstandard_style)]
            },
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            enum_token: syn::token::Enum::default(),
            ident: ident.clone(),
            generics: syn::Generics::default(),
            variants: variants.collect(),
            brace_token: syn::token::Brace::default(),
        });

        // generate the StrongEnum variant
        self.variants.push(syn::parse_quote!(#ident(#ident)));
    }
    fn sort(mut self) -> Sorted<Self> {
        // sort for consistency
        self.enums.sort_by(|a, b| a.ident.cmp(&b.ident));
        self.variants.sort_by(|a, b| a.ident.cmp(&b.ident));
        Sorted(self)
    }
}
impl Sorted<EnumCollector> {
    fn codegen(self) -> syn::File {
        let Sorted(enums) = self;

        // generate StrongInstance enum
        let mut strong_enum: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            #[non_exhaustive]
            pub enum StrongEnum {
            }
        };
        strong_enum.variants.extend(enums.variants);

        // create complete file including use statements
        let mut complete_file: syn::File = syn::parse_quote! {};
        complete_file.items.push(syn::Item::Enum(strong_enum));
        complete_file
            .items
            .extend(enums.enums.into_iter().map(syn::Item::Enum));

        complete_file
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
