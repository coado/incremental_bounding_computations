extern crate incremental_computations;
use incremental_computations::{graph, tsp};
use criterion::{criterion_group, criterion_main, Criterion, black_box};

use std::rc::Rc;

fn run_tsp(n: i32, is_incremental: bool) {
    let mut tsp_graph = graph::Graph::new();
    tsp_graph.fill_with_random_points(n);
    tsp_graph.fill_with_edges();
    let rc_tsp_graph = Rc::new(tsp_graph);

    let mut tsp = tsp::Tsp::new(Rc::clone(&rc_tsp_graph), is_incremental);
    tsp.generate_starting_path();
    tsp.tsp_2_opt().unwrap();
}

fn benchmark_naive_20(c: &mut Criterion) {
    c.bench_function("naive_20", |b| b.iter(|| {
        black_box(run_tsp(20, false));
    }));
}

fn benchmark_incremental_20(c: &mut Criterion) {
    c.bench_function("incremental_20", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        run_tsp(20, true);
    }));
}

fn benchmark_naive_100(c: &mut Criterion) {
    c.bench_function("naive_100", |b| b.iter(|| {
        black_box(run_tsp(100, false));
    }));
}

fn benchmark_incremental_100(c: &mut Criterion) {
    c.bench_function("incremental_100", |b: &mut criterion::Bencher<'_>| b.iter(|| {
       run_tsp(100, true);
    }));
}

fn benchmark_naive_200(c: &mut Criterion) {
    c.bench_function("naive_200", |b| b.iter(|| {
        black_box(run_tsp(200, false));
    }));
}

fn benchmark_incremental_200(c: &mut Criterion) {
    c.bench_function("incremental_200", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        run_tsp(200, true);
    }));
}

fn benchmark_naive_500(c: &mut Criterion) {
    c.bench_function("naive_500", |b| b.iter(|| {
        black_box(run_tsp(1000, false));
    }));
}

fn benchmark_incremental_500(c: &mut Criterion) {
    c.bench_function("incremental_500", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        run_tsp(1000, true);
    }));
}

criterion_group!(
    benches, 
    benchmark_naive_20, 
    benchmark_incremental_20,
    benchmark_naive_100, 
    benchmark_incremental_100, 
    benchmark_naive_200, 
    benchmark_incremental_200,
    benchmark_naive_500,
    benchmark_incremental_500
);
criterion_main!(benches);
