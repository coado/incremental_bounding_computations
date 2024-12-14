extern crate incremental_computations;
#[macro_use] extern crate adapton;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use adapton::macros::*;
use adapton::engine::*;

enum GraphType {
    Classic,
    Firewall
}

struct FirewallGraph {
    input_layer: Vec<Art<i32>>,
    root: Art<i32>
}

impl FirewallGraph {
    pub fn new(n: usize, graph_type: GraphType) -> FirewallGraph {
        manage::init_dcg();

        let input_layer = (0..n).map(|_| {
            cell!(0)
        }).collect::<Vec<Art<i32>>>();
    
        let power_layer = input_layer
            .iter()
            .map(|node| {
                let node_clone = node.clone();

                match graph_type {
                    GraphType::Classic => {
                        thunk!({
                            let val = get!(node_clone);
                            val.pow(2)
                        })
                    },
                    GraphType::Firewall => {
                        let t = thunk!({
                            let val = get!(node_clone);
                            cell!(val.pow(2))
                        });
                        force(&t)
                    }
                }
        }).collect::<Vec<Art<i32>>>();
    
        let long_task_layer = power_layer
            .iter()
            .map(|node| {
                let inner_cell = node.clone();
    
                thunk!({
                    let val = get!(inner_cell);
                    for _ in 0..100000 {}
    
                    val
                })
            }).collect::<Vec<Art<i32>>>();
    
        let root = thunk!(long_task_layer
            .iter()
            .fold(0, |acc, node| {
                let node_clone = node.clone();
                acc + i32::from(get!(node_clone))
            })
        );
    
        FirewallGraph {
            input_layer,
            root
        }
    }

    pub fn update_input_node(&mut self, idx: usize, val: i32) {
        set(&self.input_layer[idx], val);
    }

    pub fn get_root(&self) -> i32 {
        get!(self.root)
    }
}



fn firewall_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("firewall_benchmark");

    for n in [5, 10, 20, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("Classic", n), n, |b, &n| {
            let mut graph = FirewallGraph::new(n, GraphType::Classic);
            graph.get_root();
            let mut values = Vec::new();
            for i in 0..n {
                values.push(i as i32);
                values.push(-(i as i32));
            };

            b.iter(|| {
                for val in values.iter() {
                    graph.update_input_node(0, *val);
                    graph.get_root();
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("Firewall", n), n, |b, &n| {
            let mut graph = FirewallGraph::new(n, GraphType::Firewall);
            graph.get_root();
            let mut values = Vec::new();
            for i in 0..n {
                values.push(i as i32);
                values.push(-(i as i32));
            };

            b.iter(|| {
                for val in values.iter() {
                    graph.update_input_node(0, *val);
                    graph.get_root();
                }
            });
        });
    }
}

criterion_group!(
    benches, 
    firewall_benchmark
);
criterion_main!(benches);
