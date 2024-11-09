#[macro_use] extern crate adapton;
mod graph;
mod tsp;
mod tsp_draw;
mod tsp_comp;

use crate::graph::Graph;
use crate::tsp::Tsp;
use std::rc::Rc;

fn main() {
    tsp_draw::draw();

    // tsp_comp::run();

    // let mut tsp_graph = Graph::new();
    // tsp_graph.fill_with_random_points(200);
    // tsp_graph.fill_with_edges();

    // let tsp_graph = Rc::new(tsp_graph);
    // let mut tsp = Tsp::new(Rc::clone(&tsp_graph));
    // tsp.tsp_2_opt();

    // println!("Path: {:?}", path);
    // println!("Length: {}", length);

}
