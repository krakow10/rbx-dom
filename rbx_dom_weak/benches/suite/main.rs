use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rbx_dom_weak::{InstanceBuilder, WeakDom};

pub fn clone_into_external(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_into_external");

    group
        // cranking sample size and measurement time to ensure we get a
        // statistically significant result
        .sample_size(1000)
        .measurement_time(Duration::from_secs(30))
        .bench_function("1000 ref rewrites", |b| {
            b.iter_batched(
                || {
                    let mut dest_tree = WeakDom::new(InstanceBuilder::new("Folder"));
                    dest_tree.reserve(1000);

                    let mut src_tree = WeakDom::new(InstanceBuilder::new("Folder"));
                    let src_instance = InstanceBuilder::new("Folder");
                    let src_referent = src_instance.referent();
                    let mut children = Vec::with_capacity(1000);

                    for _ in 0..1000 {
                        children.push(
                            InstanceBuilder::new("ObjectValue")
                                .with_property("Value", src_referent),
                        );
                    }
                    src_tree.insert(src_tree.root_ref(), src_instance.with_children(children));
                    (src_referent, src_tree, dest_tree)
                },
                |(src_instance, src_tree, mut dest_tree)| {
                    src_tree.clone_into_external(src_instance, &mut dest_tree);
                },
                BatchSize::SmallInput,
            );
        });
}

criterion_group!(bench_suite, clone_into_external,);

criterion_main!(bench_suite);
