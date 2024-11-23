#[macro_use] extern crate adapton;
mod graph;
mod tsp;
mod tsp_draw;
mod tsp_comp;
mod graph_coloring;
mod graph_coloring_draw;

fn main() {
    // graph_coloring_draw::draw();
    tsp_draw::draw();
}
