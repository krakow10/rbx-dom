use std::{
    io::{self, BufWriter},
    path::PathBuf,
};

use clap::Parser;

use crate::ModelKind;

#[derive(Debug, Parser)]
pub struct ViewBinaryCommand {
    /// The file to emit the contents of.
    input: PathBuf,
}

impl ViewBinaryCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let input_kind = ModelKind::from_path(&self.input)?;

        if input_kind != ModelKind::Binary {
            anyhow::bail!("not a binary model or place file: {}", self.input.display());
        }

        let input_file = fs_err::read(&self.input)?;

        log::debug!("Decoding file into text format");
        let model = rbx_binary::text_format::DecodedModel::from_slice(input_file.as_slice());

        log::debug!("Writing to stdout");
        let stdout = io::stdout();
        let output = BufWriter::new(stdout.lock());
        serde_yaml::to_writer(output, &model)?;

        Ok(())
    }
}
