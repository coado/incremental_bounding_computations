extern crate incremental_computations;
use incremental_computations::{graph, tsp};
use criterion::{criterion_group, criterion_main, Criterion};

use std::rc::Rc;


fn benchmarks(c: &mut Criterion) {
    c.bench_function("my benchmark", |b| b.iter(|| {
        let mut tsp_graph = graph::Graph::new();
        tsp_graph.fill_with_random_points(200);
        tsp_graph.fill_with_edges();

        let tsp_graph = Rc::new(tsp_graph);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), true);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);