use crate::dom::StrongDom;
use crate::instances::Part;
use rbx_types::{CFrame, Ref};

#[test]
fn part_inherits_instance() {
    let mut dom = StrongDom::default();

    // dummy referent just to get this compiling...
    let referent = Ref::none();

    if let Some(instance) = dom.get_by_ref_mut(referent)
        && let Some(part) = instance.as_class_mut::<Part>()
    {
        // look ma, inheritance in rust!
        part.Name = "Part".to_owned();
        part.CFrame = CFrame::identity();
    }
}
