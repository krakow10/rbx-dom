use rbx_types::{CFrame, Matrix3, Vector3};
use rbx_types_strong::instances::Part;

fn default_cframe() -> CFrame {
    CFrame::new(Vector3::new(0.0, 0.0, 0.0), Matrix3::identity())
}
fn default_part() -> Part {
    // currently no way to initialize
    todo!()
}
#[test]
fn part_inherits_instance() {
    // Part::default() would be nice
    let mut part: Part = default_part();

    // look ma, inheritance in rust!
    part.Name = "Part".to_owned();
    part.CFrame = default_cframe();
}
