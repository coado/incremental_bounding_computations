#[macro_use] extern crate adapton;
mod graph;
mod tsp;
mod tsp_draw;
mod tsp_comp;
mod graph_coloring;
mod graph_coloring_draw;
mod graph_coloring_comp;
mod diagnostics;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let write_dcg = match env::var("ADAPTON_WRITE_DCG") {
        Ok(_) => true,
        _ => false,
    };

    println!("Write DCG enabled: {}", write_dcg);
    graph_coloring_draw::draw();
    
    // tsp_draw::draw();
}
