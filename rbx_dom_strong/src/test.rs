use crate::dom::StrongDom;
use crate::instances::{BasePart, Part};
use rbx_types::{CFrame, Ref};

#[test]
fn part_inherits_instance() {
    let mut part = Part::default();

    // look ma, inheritance in rust!
    part.Name = "Part".to_owned();
    part.CFrame = CFrame::identity();
}

#[test]
fn get_by_ref() {
    let dom = StrongDom::default();

    // dummy referent just to get this compiling...
    let referent = Ref::none();

    if let Some(instance) = dom.get_by_ref(referent)
        && let Some(base_part) = instance.as_class::<BasePart>()
    {
        println!("BasePart is a superclass {:?}", base_part.CFrame)
    }
}
