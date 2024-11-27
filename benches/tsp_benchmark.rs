extern crate incremental_computations;
use incremental_computations::{graph, tsp};
use criterion::{criterion_group, criterion_main, Criterion, black_box, BenchmarkId};

use std::rc::Rc;

fn run_tsp(n: i32, is_incremental: bool) {
    let mut tsp_graph = graph::Graph::new();
    tsp_graph.fill_with_random_points(n);
    tsp_graph.fill_with_edges_full();
    let rc_tsp_graph = Rc::new(tsp_graph);

    let mut tsp = tsp::Tsp::new(Rc::clone(&rc_tsp_graph), is_incremental);
    tsp.generate_starting_path();
    tsp.tsp_2_opt().unwrap();
}

fn tsp_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tsp_benchmark");
    group.sample_size(20);
    for n in [10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Naive", n), n, |b, &n| {
            b.iter(|| {
                run_tsp(n, false);
            });
        });

        group.bench_with_input(BenchmarkId::new("Incremental", n), n, |b, &n| {
            b.iter(|| {
                run_tsp(n, true);
            });
        });
    }

}

// fn benchmark_naive_20(c: &mut Criterion) {
//     c.bench_function("naive_20", |b| b.iter(|| {
//         black_box(run_tsp(20, false));
//     }));
// }

// fn benchmark_incremental_20(c: &mut Criterion) {
//     c.bench_function("incremental_20", |b: &mut criterion::Bencher<'_>| b.iter(|| {
//         run_tsp(20, true);
//     })); 
// }

// fn benchmark_naive_100(c: &mut Criterion) {
//     c.bench_function("naive_100", |b| b.iter(|| {
//         black_box(run_tsp(100, false));
//     }));
// }

// fn benchmark_incremental_100(c: &mut Criterion) {
//     c.bench_function("incremental_100", |b: &mut criterion::Bencher<'_>| b.iter(|| {
//        run_tsp(100, true);
//     }));
// }

// fn benchmark_naive_200(c: &mut Criterion) {
//     c.bench_function("naive_200", |b| b.iter(|| {
//         black_box(run_tsp(200, false));
//     }));
// }

// fn benchmark_incremental_200(c: &mut Criterion) {
//     c.bench_function("incremental_200", |b: &mut criterion::Bencher<'_>| b.iter(|| {
//         run_tsp(200, true);
//     }));
// }

// fn benchmark_naive_500(c: &mut Criterion) {
//     c.bench_function("naive_500", |b| b.iter(|| {
//         black_box(run_tsp(1000, false));
//     }));
// }

// fn benchmark_incremental_500(c: &mut Criterion) {
//     c.bench_function("incremental_500", |b: &mut criterion::Bencher<'_>| b.iter(|| {
//         run_tsp(1000, true);
//     }));
// }

criterion_group!(
    benches, 
    tsp_benchmark
);
criterion_main!(benches);
