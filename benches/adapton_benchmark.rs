// extern crate incremental_computations;
// use incremental_computations::{graph, tsp, input_nodes};
// use criterion::{criterion_group, criterion_main, Criterion};


// fn adapton_sum_benchmark(c: &mut Criterion) {
//     let n = 1000;
//     let input_nodes = input_nodes::create_input_nodes(n);
//     let res = input_nodes::devide_and_conquer(&input_nodes, 0, n - 1);
//     c.bench_function("adapton_benchmark", |b| b.iter(|| {
//         input_nodes::update_input_nodes(&input_nodes, n);
//         input_nodes::get_art(&res);
//     }));
// }


// criterion_group!(benches, adapton_sum_benchmark);
// criterion_main!(benches);