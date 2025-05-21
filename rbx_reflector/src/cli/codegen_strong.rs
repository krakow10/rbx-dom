use std::path::PathBuf;

use clap::Parser;

/// Generate strong types for all classes and enums.
#[derive(Debug, Parser)]
pub struct CodegenStrongSubcommand {
    /// Where to output the files.  This should be rbx_dom_strong/src/generated/
    pub output: PathBuf,
}
impl CodegenStrongSubcommand {
    pub fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
