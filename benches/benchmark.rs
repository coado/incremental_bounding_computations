extern crate incremental_computations;
use incremental_computations::{graph, tsp};
use criterion::{criterion_group, criterion_main, Criterion};

use std::rc::Rc;

fn create_graph(n: i32) -> Rc<graph::Graph> {
    let mut tsp_graph = graph::Graph::new();
    tsp_graph.fill_with_random_points(n);
    tsp_graph.fill_with_edges();

    Rc::new(tsp_graph)
}

fn benchmark_naive_20(c: &mut Criterion) {
    c.bench_function("naive_20", |b| b.iter(|| {
        let tsp_graph = create_graph(20);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), false);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

fn benchmark_incremental_20(c: &mut Criterion) {
    c.bench_function("incremental_20", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let tsp_graph = create_graph(20);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), true);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

fn benchmark_naive_100(c: &mut Criterion) {
    c.bench_function("naive_100", |b| b.iter(|| {
        let tsp_graph = create_graph(100);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), false);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

fn benchmark_incremental_100(c: &mut Criterion) {
    c.bench_function("incremental_100", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let tsp_graph = create_graph(100);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), true);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

fn benchmark_naive_200(c: &mut Criterion) {
    c.bench_function("naive_200", |b| b.iter(|| {
        let tsp_graph = create_graph(200);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), false);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

fn benchmark_incremental_200(c: &mut Criterion) {
    c.bench_function("incremental_200", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let tsp_graph = create_graph(200);
        let mut tsp = tsp::Tsp::new(Rc::clone(&tsp_graph), true);
        tsp.generate_starting_path();
        tsp.tsp_2_opt().unwrap();
    }));
}

criterion_group!(
    benches, 
    benchmark_naive_20, 
    benchmark_incremental_20,
    benchmark_naive_100, 
    benchmark_incremental_100, 
    benchmark_naive_200, 
    benchmark_incremental_200
);
criterion_main!(benches);