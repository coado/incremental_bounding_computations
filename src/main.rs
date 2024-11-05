#[macro_use] extern crate adapton;
mod graph;
mod tsp;
mod tsp_draw;
mod tsp_comp;

fn main() {
    // tsp_draw::draw();

    tsp_comp::run();
}
