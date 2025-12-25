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

    if let Some(instance) = dom.get_by_ref(Ref::none()) {
        match instance.as_class::<BasePart>() {
            Some(base_part) => {
                println!("BasePart is a superclass {:?}", base_part.CFrame)
            }
            None => println!("Other class {:?}", instance),
        }
    }
}
