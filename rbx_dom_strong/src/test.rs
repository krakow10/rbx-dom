use crate::dom::StrongDom;
use crate::instance::InstanceBuilder;
use rbx_classes::instances::Part;
use rbx_types::{CFrame, Ref};

#[test]
fn builder() {
    let mut part = Part::default();
    part.Name = "Part".to_owned();
    part.CFrame = CFrame::identity();
    part.Anchored = true;

    let part_builder = InstanceBuilder::new(part);
    let referent = part_builder.referent();

    let dom = StrongDom::new(part_builder);

    let part = dom
        .get_by_ref(referent)
        .unwrap()
        .as_class::<Part>()
        .unwrap();

    assert_eq!(part.Name, "Part");
}

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

#[test]
fn large_depth_tree() {
    use rbx_classes::instances::Folder;
    // We've had issues with stack overflows when creating WeakDoms with
    // particularly deep trees, so this test is simply to ensure that does
    // not happen. `i16::MAX` is arbitrary but very large for recursion.
    const N: usize = i16::MAX as usize;

    let mut refs = Vec::with_capacity(N + 1);
    let mut base = InstanceBuilder::<Folder>::default();
    refs.push(base.referent());
    for _ in 0..N {
        base = InstanceBuilder::<Folder>::default().with_child(base);
        refs.push(base.referent());
    }
    let _ = StrongDom::new(base);
}
