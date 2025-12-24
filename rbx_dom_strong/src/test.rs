use crate::instance::Instance;
use rbx_classes::instances::Part;
use rbx_types::CFrame;

#[test]
fn part_inherits_instance() {
    let mut part = Part::<Instance>::default();

    // look ma, inheritance in rust!
    part.Name = "Part".to_owned();
    part.CFrame = CFrame::identity();
}
