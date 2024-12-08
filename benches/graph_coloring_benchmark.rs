extern crate incremental_computations;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use incremental_computations::{graph::Graph, graph_coloring::{GraphColoring, ScoreCalcType}};

use std::rc::Rc;

fn run_graph_coloring(n: i32, score_type: ScoreCalcType) {
    let mut graph = Graph::new();
    graph.fill_with_random_points(n);
    graph.fill_with_edges_full();
    let rc_graph = Rc::new(graph);

    let mut graph_coloring = GraphColoring::new(Rc::clone(&rc_graph), score_type);
    graph_coloring.graph_coloring();
}

fn graph_coloring_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_coloring_benchmark");
    group.sample_size(20);
    for n in [10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Naive", n), n, |b, &n| {
            b.iter(|| {
                run_graph_coloring(n, ScoreCalcType::Naive);
            });
        });

        group.bench_with_input(BenchmarkId::new("Slow", n), n, |b, &n| {
            b.iter(|| {
                run_graph_coloring(n, ScoreCalcType::Slow);
            });
        });

        group.bench_with_input(BenchmarkId::new("Incremental", n), n, |b, &n| {
            b.iter(|| {
                run_graph_coloring(n, ScoreCalcType::Incremental);
            });
        });
    }

}

criterion_group!(
    benches, 
    graph_coloring_benchmark
);
criterion_main!(benches);
