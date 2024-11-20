use std::rc::Rc;
use std::collections::HashMap;
use crate::graph::{Graph, PointId, EdgeId};

const NUMBER_OF_ITERATIONS: i32 = 1000;

#[derive(Debug, Clone, Copy)]
struct Color(i32);

pub struct GraphColoring {
    graph: Rc<Graph>,
    colors: Vec<Option<Color>>,
    number_of_colors: i32,
    colors_buckets: HashMap<Color, Vec<PointId>>,
    violating_edges_buckets: HashMap<Color, EdgeId>
}

impl GraphColoring {
    pub fn new(graph: Rc<Graph>) -> GraphColoring {
        let number_of_colors = graph.get_number_of_nodes() as i32;
        let mut colors: Vec<Option<Color>> = Vec::with_capacity(number_of_colors as usize);
        colors.fill(None);

        GraphColoring {
            graph,
            colors,
            number_of_colors,
            colors_buckets: HashMap::new(),
            violating_edges_buckets: HashMap::new()
        }
    }

    fn run() {
        
        for i in 0..NUMBER_OF_ITERATIONS {
            // Two operations:
        }
        
    }
}