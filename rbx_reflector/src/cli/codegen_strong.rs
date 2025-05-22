use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;
use quote::ToTokens;
use rbx_reflection::{ClassDescriptor, DataType, EnumDescriptor};

/// Generate strong types for all classes and enums.
#[derive(Debug, Parser)]
pub struct CodegenStrongSubcommand {
    /// Where to output the files.  This should be rbx_dom_strong/src/generated/
    pub output: PathBuf,
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
        DataType::Enum(cow) => syn::parse_str(cow).unwrap(),
        _ => unimplemented!(),
    }
}

struct StrongInstancesCollector {
    structs: Vec<syn::ItemStruct>,
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
        // generate fields
        let mut fields = Vec::new();
        for prop in descriptor.properties.values() {
            let ident = syn::Ident::new(
                &heck::ToUpperCamelCase::to_upper_camel_case(prop.name.as_ref()),
                proc_macro2::Span::call_site(),
            );
            let ty = generate_data_type(&prop.data_type);
            let field = syn::Field {
                attrs: Vec::new(),
                vis: syn::Visibility::Public(syn::token::Pub::default()),
                mutability: syn::FieldMutability::None,
                ident: Some(ident),
                colon_token: Some(syn::token::Colon::default()),
                ty,
            };
            fields.push(field);
        }
        // sort props for consistency
        fields.sort_by(|a, b| a.ident.cmp(&b.ident));

        // struct ident
        let ident = syn::Ident::new(&descriptor.name, proc_macro2::Span::call_site());

        // generate the class struct
        self.structs.push(syn::ItemStruct {
            attrs: vec![syn::parse_quote!(#[derive(Debug, Clone)])],
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            struct_token: syn::token::Struct::default(),
            ident: ident.clone(),
            generics: syn::Generics::default(),
            fields: syn::Fields::Named(syn::FieldsNamed {
                brace_token: syn::token::Brace::default(),
                named: fields.into_iter().collect(),
            }),
            semi_token: None,
        });

        // generate the StrongInstances variant
        self.variants.push(syn::Variant {
            attrs: Vec::new(),
            ident: ident.clone(),
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: syn::token::Paren::default(),
                unnamed: [syn::Field {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: None,
                    colon_token: None,
                    ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: [syn::PathSegment {
                                ident,
                                arguments: syn::PathArguments::None,
                            }]
                            .into_iter()
                            .collect(),
                        },
                    }),
                }]
                .into_iter()
                .collect(),
            }),
            discriminant: None,
        });
    }
    fn codegen(mut self) -> syn::File {
        // sort for consistency
        self.structs.sort_by(|a, b| a.ident.cmp(&b.ident));
        self.variants.sort_by(|a, b| a.ident.cmp(&b.ident));

        // generate StrongInstance enum
        let mut strong_instances_enum: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            #[non_exhaustive]
            pub enum StrongInstance {
            }
        };
        strong_instances_enum.variants.extend(self.variants);

        // create complete file including use statements
        let mut complete_file: syn::File = syn::parse_quote! {
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
        };
        complete_file
            .items
            .push(syn::Item::Enum(strong_instances_enum));
        complete_file
            .items
            .extend(self.structs.into_iter().map(syn::Item::Struct));

        complete_file
    }
}

fn fix_enum_ident(ident: &str) -> &str {
    match ident {
        // StudioScriptEditorColorCategories has this variant
        "Self" => "FIXME",
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
            attrs: vec![syn::parse_quote!(#[derive(Debug, Clone)])],
            vis: syn::Visibility::Public(syn::token::Pub::default()),
            enum_token: syn::token::Enum::default(),
            ident: ident.clone(),
            generics: syn::Generics::default(),
            variants: variants.collect(),
            brace_token: syn::token::Brace::default(),
        });

        // generate the StrongInstances variant
        self.variants.push(syn::Variant {
            attrs: Vec::new(),
            ident: ident.clone(),
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed {
                paren_token: syn::token::Paren::default(),
                unnamed: [syn::Field {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    mutability: syn::FieldMutability::None,
                    ident: None,
                    colon_token: None,
                    ty: syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: [syn::PathSegment {
                                ident,
                                arguments: syn::PathArguments::None,
                            }]
                            .into_iter()
                            .collect(),
                        },
                    }),
                }]
                .into_iter()
                .collect(),
            }),
            discriminant: None,
        });
    }
    fn codegen(mut self) -> syn::File {
        // sort for consistency
        self.enums.sort_by(|a, b| a.ident.cmp(&b.ident));
        self.variants.sort_by(|a, b| a.ident.cmp(&b.ident));

        // generate StrongInstance enum
        let mut strong_enum: syn::ItemEnum = syn::parse_quote! {
            #[derive(Debug, Clone)]
            #[non_exhaustive]
            pub enum StrongEnum {
            }
        };
        strong_enum.variants.extend(self.variants);

        // create complete file including use statements
        let mut complete_file: syn::File = syn::parse_quote! {};
        complete_file.items.push(syn::Item::Enum(strong_enum));
        complete_file
            .items
            .extend(self.enums.into_iter().map(syn::Item::Enum));

        complete_file
    }
}

impl CodegenStrongSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let db = rbx_reflection_database::get();

        let dest_instance = self.output.join("instance.rs");
        let dest_enum = self.output.join("enum.rs");

        // ==== generate instances.rs ====
        let instance_code = {
            let mut strong_instances = StrongInstancesCollector::new();
            for descriptor in db.classes.values() {
                strong_instances.push(descriptor);
            }
            let complete_file = strong_instances.codegen();

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
            let complete_file = strong_enum.codegen();

            // make a string of the unformatted code
            let code = complete_file.into_token_stream().to_string();

            // format via cli
            let code = rustfmt(code.as_bytes())?;

            code
        };

        // save to destination file
        write_dest(&instance_code, dest_instance)?;
        write_dest(&enum_code, dest_enum)?;
        Ok(())
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
fn write_dest(code: &[u8], dest: PathBuf) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(dest)?;
    file.write_all(code)?;
    Ok(())
}
