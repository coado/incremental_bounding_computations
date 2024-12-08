extern crate incremental_computations;
use incremental_computations::{graph, tsp::{ScoreCalcTypeTSP, Tsp}};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use std::rc::Rc;

fn run_tsp(n: i32, score_calc_type: ScoreCalcTypeTSP) {
    let mut tsp_graph = graph::Graph::new();
    tsp_graph.fill_with_random_points(n);
    tsp_graph.fill_with_edges_full();
    let rc_tsp_graph = Rc::new(tsp_graph);

    let mut tsp = Tsp::new(Rc::clone(&rc_tsp_graph), score_calc_type);
    tsp.generate_starting_path();
    tsp.tsp().unwrap();
}

fn tsp_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tsp_benchmark");
    group.sample_size(20);
    for n in [10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Fast", n), n, |b, &n| {
            b.iter(|| {
                run_tsp(n, ScoreCalcTypeTSP::Fast);
            });
        });

        group.bench_with_input(BenchmarkId::new("Slow", n), n, |b, &n| {
            b.iter(|| {
                run_tsp(n, ScoreCalcTypeTSP::Slow);
            });
        });

        group.bench_with_input(BenchmarkId::new("Incremental", n), n, |b, &n| {
            b.iter(|| {
                run_tsp(n, ScoreCalcTypeTSP::Incremental);
            });
        });
    }

}

criterion_group!(
    benches, 
    tsp_benchmark
);
criterion_main!(benches);
