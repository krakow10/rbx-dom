use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;
use quote::ToTokens;
use rbx_reflection::{ClassDescriptor, EnumDescriptor};

/// Generate strong types for all classes and enums.
#[derive(Debug, Parser)]
pub struct CodegenStrongSubcommand {
    /// Where to output the files.  This should be rbx_dom_strong/src/generated/
    pub output: PathBuf,
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
        // TODO
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

            use serde::{Serialize, Deserialize};
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
        // TODO
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
