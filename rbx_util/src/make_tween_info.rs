use std::{
    io::BufWriter,
    path::PathBuf,
};

use anyhow::Context as _;
use clap::Parser;
use fs_err::File;

use rbx_dom_weak::types::{Attributes, Enum, TweenInfo};
use rbx_dom_weak::{InstanceBuilder, WeakDom};

use crate::ModelKind;

/// The `EasingStyle` values that can be stored in a `TweenInfo`, matching the
/// `Enum.EasingStyle` items in Roblox.
const EASING_LINEAR: u32 = 0;
const EASING_SINE: u32 = 1;
const EASING_QUAD: u32 = 2;
const EASING_CUBIC: u32 = 3;
const EASING_QUART: u32 = 4;

/// The `EasingDirection` values that can be stored in a `TweenInfo`, matching
/// the `Enum.EasingDirection` items in Roblox.
const DIRECTION_OUT: u32 = 0;
const DIRECTION_IN: u32 = 1;
const DIRECTION_IN_OUT: u32 = 2;

/// Builds a `TweenInfo` with the given fields.
fn tween_info(
    time: f32,
    easing_style: u32,
    easing_direction: u32,
    repeat_count: i32,
    reverses: bool,
    delay_time: f32,
) -> TweenInfo {
    TweenInfo {
        time,
        easing_style: Enum::from_u32(easing_style),
        easing_direction: Enum::from_u32(easing_direction),
        repeat_count,
        reverses,
        delay_time,
    }
}

/// Builds the set of `TweenInfo` attributes written into the generated model.
///
/// The values are chosen to flex the `TweenInfo` handling in the attributes
/// reader/writer: they cover zero, fractional, small, and large `f32` fields,
/// every `EasingDirection`, several `EasingStyle` values, positive and negative
/// `repeat_count`s, and both `reverses` states.
fn build_attributes() -> Attributes {
    let mut attributes = Attributes::new();

    // All-zero fields: the simplest valid `TweenInfo`.
    attributes.insert(
        "ZeroValues".into(),
        tween_info(0.0, EASING_LINEAR, DIRECTION_OUT, 0, false, 0.0).into(),
    );

    // The exact values from the serialization documentation, a good
    // round-trip reference point.
    attributes.insert(
        "DocExample".into(),
        tween_info(1.5, EASING_SINE, DIRECTION_IN_OUT, 4, false, 6.5).into(),
    );

    // Negative repeat count (`-1` = repeat indefinitely) with a reversed tween.
    attributes.insert(
        "InfiniteRepeat".into(),
        tween_info(0.25, EASING_LINEAR, DIRECTION_IN, -1, true, 0.5).into(),
    );

    // A very short duration and a high positive repeat count.
    attributes.insert(
        "FastQuadOut".into(),
        tween_info(0.016, EASING_QUAD, DIRECTION_OUT, 10, true, 0.1).into(),
    );

    // A long delay paired with a longer duration.
    attributes.insert(
        "LongCubicDelay".into(),
        tween_info(2.5, EASING_CUBIC, DIRECTION_IN_OUT, 3, false, 12.75).into(),
    );

    // Fractional `f32` values throughout, with a single repeat.
    attributes.insert(
        "FractionalQuart".into(),
        tween_info(0.125, EASING_QUART, DIRECTION_IN, 1, true, 0.375).into(),
    );

    attributes
}

#[derive(Debug, Parser)]
pub struct MakeTweenInfoCommand {
    /// The path to write the generated file to. The format is determined by
    /// the file extension of this path.
    output_path: PathBuf,
}

impl MakeTweenInfoCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        let output_kind = ModelKind::from_path(&self.output_path)?;

        let dom = WeakDom::new(
            InstanceBuilder::new("Folder")
                .with_name("TweenInfoAttributes")
                .with_property("Attributes", build_attributes()),
        );

        let output_file = BufWriter::new(File::create(&self.output_path)?);

        rbx_binary::to_writer(output_file, &dom, &[dom.root_ref()])
            .with_context(|| format!("Failed to write {}", self.output_path.display()))?;

        log::info!(
            "Wrote {output_kind:?} file with TweenInfo attributes to {}",
            self.output_path.display()
        );

        Ok(())
    }
}
