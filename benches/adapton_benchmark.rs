extern crate incremental_computations;
#[macro_use] extern crate adapton;

use criterion::{criterion_group, criterion_main, Criterion, black_box, BenchmarkId};

use adapton::macros::*;
use adapton::engine::*;

fn get(n: i32) -> i32 {
    return n;
}

fn create_comp_graph(nodes: &Vec<Art<i32>>, left: usize, right: usize) -> Art<i32> {
    if left == right {
        return nodes[left].clone();
    }

    let mid = left + (right - left) / 2;
    let left_res = create_comp_graph(nodes, left, mid);
    let right_res = create_comp_graph(nodes, mid + 1, right);

    thunk!(get!(left_res) + get!(right_res))
}

fn adapton_sum_benchmark(c: &mut Criterion) {
    manage::init_dcg();
    let mut group = c.benchmark_group("adapton_sum_benchmark");

    for n in [100, 500, 1000, 2000, 5000].iter() {
        group.bench_with_input(BenchmarkId::new("Adapton", n), n, |b, &n| {
            let input_nodes = (0..n).map(|_| cell!(0)).collect::<Vec<Art<i32>>>();
            let res = create_comp_graph(&input_nodes, 0, n - 1);

            b.iter(|| {
                input_nodes.iter().for_each(|node| {
                    set(node, 1);
                    get!(res);
                });
            });
        });

        group.bench_with_input(BenchmarkId::new("Naive", n), n, |b, &n| {
            let mut nodes = (0..n).map(|i| i as i32).collect::<Vec<i32>>();

            let calc = |nodes: &Vec<i32>| {
                let mut sum = 0;
                for i in 0..n {
                    get(0);
                    sum += nodes[i as usize];
                }
                sum
            };

            b.iter(|| {
                for i in 0..n {
                    nodes[i as usize] = i as i32;
                    calc(&nodes);
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("Naive with black box", n), n, |b, &n| {
            let mut nodes = (0..n).map(|i| i as i32).collect::<Vec<i32>>();

            let calc = |nodes: &Vec<i32>| {
                let mut sum = 0;
                for i in 0..n {
                    black_box(get(0));
                    sum += nodes[i as usize];
                }
                sum
            };

            b.iter(|| {
                for i in 0..n {
                    nodes[i as usize] = i as i32;
                    calc(&nodes);
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, adapton_sum_benchmark);
criterion_main!(benches);