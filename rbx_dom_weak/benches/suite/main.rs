use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rbx_dom_weak::{InstanceBuilder, WeakDom};

pub fn clone_into_external(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_into_external");

    group
        // cranking sample size and measurement time to ensure we get a
        // statistically significant result
        .measurement_time(Duration::from_secs(30))
        .bench_function("Miner's Haven", |b| {
            b.iter_batched(
                || {
                    let dest_tree = WeakDom::new(InstanceBuilder::new("Folder"));

                    let miners_haven =
                        include_bytes!("../../../rbx_binary/benches/files/miners-haven.rbxl")
                            .as_slice();
                    let src_tree = rbx_binary::from_reader(miners_haven).unwrap();
                    let src_referent = src_tree.root_ref();

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
