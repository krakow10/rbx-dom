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

    let instance = dom.get_by_ref(Ref::none());

    if let Some(result) = instance {
        match result {
            Ok(base_part) => {
                let base_part: &BasePart = base_part;
                println!("BasePart is a superclass {:?}", base_part.CFrame)
            }
            Err(other) => println!("Other class {:?}", other),
        }
    }
}
