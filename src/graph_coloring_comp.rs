use adapton::macros::*;
use adapton::engine::*;
use adapton::reflect;

use crate::diagnostics::Diagnostics;

pub struct GraphColoringComp {
    input_nodes_layer: Vec<Art<i32>>,
    guards_layer: Vec<Art<i32>>,
    computations_layer: Vec<Art<i32>>,
    res: Option<Art<i32>>,
    sealed: bool,
    number_of_colors: i32,
    al: &'static Vec<Vec<i32>>,
    diagnostics: Option<Diagnostics>
}

impl GraphColoringComp {
    pub fn new(al: &'static Vec<Vec<i32>>, n: usize) -> GraphColoringComp {
        manage::init_dcg();
        reflect::dcg_reflect_begin();

        let input_nodes_layer = (0..n).map(|_| {
            cell!(0)
        }).collect();
        
        GraphColoringComp {
            input_nodes_layer,
            guards_layer: Vec::new(),
            computations_layer: Vec::new(),
            res: None,
            sealed: false,
            number_of_colors: n as i32,
            al,
            diagnostics: None
        }
    }

    pub fn create_computation_graph(&self) {
        
    }

    pub fn seal(&mut self) -> &Diagnostics {
        self.ensure_unsealed();
        self.sealed = true;
        let traces = reflect::dcg_reflect_end();
        let diagnostics = Diagnostics::new(traces).analyse();
        self.diagnostics = Some(diagnostics);
        self.diagnostics.as_ref().unwrap()
    }

    pub fn update_input_node(&mut self, idx: usize, val: i32) {
        self.ensure_unsealed();
        set(&self.input_nodes_layer[idx], val);
    }

    fn ensure_unsealed(&mut self) {
        assert!(!self.sealed, "Graph Coloring is sealed");
    }

    fn create_guards_layer(&mut self) -> &Vec<Art<i32>> {
        let guards_layer = (0..self.number_of_colors)
            .map(|c| {
                let guards = self.input_nodes_layer.iter().map(|input_node| {
                    let input_node_clone = input_node.clone();
                    thunk!(get!(input_node_clone) == c)
                }).collect::<Vec<Art<bool>>>();

                thunk!(guards.iter().fold(0, |acc, guard| acc + i32::from(get!(guard))))
            })
            .collect::<Vec<Art<i32>>>();
        
        self.guards_layer = guards_layer;
        &self.guards_layer
    }

    fn create_computation_layer(&mut self) -> &Vec<Art<i32>> {
        let computations_layer = self.guards_layer
            .iter()
            .enumerate()
            .map(|(c, vertecies_of_color)| {
                let vertecies_of_color = vertecies_of_color.clone();
                thunk!(
                    get!(vertecies_of_color).pow(2) + get!(vertecies_of_color) + 2
                )
            }).collect::<Vec<Art<i32>>>();

        self.computations_layer = computations_layer;
        &self.computations_layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    fn lazy_init_static_al() -> &'static Vec<Vec<i32>> {
        static mut AL: *const Vec<Vec<i32>> = std::ptr::null();
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            let al = vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ];

            unsafe {
                AL = Box::into_raw(Box::new(al));
            }
        });

        unsafe { &*AL }
    }

    #[test]
    fn test_guards_layer() {
        let static_al = lazy_init_static_al();
        let mut graph_coloring_comp = GraphColoringComp::new(static_al, 3);
        graph_coloring_comp.create_guards_layer();
        
        assert!(graph_coloring_comp.guards_layer.len() == 3, "Guards layer should have 3 guards");

        for i in 0..graph_coloring_comp.guards_layer.len() {
            graph_coloring_comp.update_input_node(i, i as i32);
        }

        for i in 0..graph_coloring_comp.guards_layer.len() {
            assert_eq!(get!(graph_coloring_comp.guards_layer[i]), 1);
        }
    }

    #[test]
    fn test_diagnostics() {
        let static_al = lazy_init_static_al();
        let mut graph_coloring_comp = GraphColoringComp::new(static_al, 3);
        let guards = graph_coloring_comp.create_guards_layer();
        assert!(guards.len() == 3, "Guards layer should have 3 guards");

        graph_coloring_comp.update_input_node(1, 1);
        let diagnostics = graph_coloring_comp.seal();

        assert!(diagnostics.cells_count == 3, "Cells count should be 3");
        assert!(diagnostics.thunks_count == 12, "Thunks count should be 12");
        
    }
}