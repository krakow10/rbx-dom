use crate::dom::StrongDom;
use crate::instance::InstanceBuilderInner;
use rbx_classes::instances;
use rbx_types::{CFrame, Ref};

type PartBuilder = instances::Part<InstanceBuilderInner>;

#[test]
fn builder() {
    let mut part_builder = PartBuilder::default();

    part_builder.Name = "Part".to_owned();
    part_builder.CFrame = CFrame::identity();
    part_builder.Anchored = true;

    let dom = StrongDom::new(part_builder);
}

#[test]
fn part_inherits_instance() {
    use crate::instances::Part;
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
