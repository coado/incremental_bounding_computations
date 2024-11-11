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

// extern crate incremental_computations;
// use incremental_computations::{graph, tsp, input_nodes};
// use criterion::{criterion_group, criterion_main, Criterion, black_box};
// use adapton::macros::*;
// use adapton::engine::*;
// use adapton::reflect;


// fn adapton_sum_benchmark(c: &mut Criterion) {
//     manage::init_dcg();
//     let n = 100;
//     let input_nodes = input_nodes::create_input_nodes(n);
//     let res = input_nodes::devide_and_conquer(&input_nodes, 0, n - 1);
//     c.bench_function("adapton_benchmark", |b| b.iter(|| {
//         input_nodes::update_input_nodes(&input_nodes, n);
//     }));
// }

// fn fibonacci(n: u64) -> u64 {
//     if n <= 1 {
//         n
//     } else {
//         fibonacci(n - 1) + fibonacci(n - 2)
//     }
// }

// fn get_test(n: i32) -> i32 {
//     if n == 0 {
//         0
//     } else {
//         get_test(n + 1) + get_test(n + 2)
//     }
// }

// fn get_test2(n: i32) -> i32 {
//     return n;
// }


// fn sum_benchmark(c: &mut Criterion) {
//     let n = 100;
//     let mut nodes = (0..n).map(|i| i).collect::<Vec<i32>>();

//     let calc = |nodes: &Vec<i32>| {
//         let mut sum = 0;
//         for i in 0..n {
//             black_box(get_test2(0));
//             sum += nodes[i as usize];
//         }
//         sum
//     };

//     c.bench_function("sum_benchmark", |b| b.iter(|| {
//         for i in 0..n {
//             nodes[i as usize] = i as i32;
//             calc(&nodes);
//         }
//     }));
// }


// criterion_group!(benches, adapton_sum_benchmark, sum_benchmark);
// criterion_main!(benches);