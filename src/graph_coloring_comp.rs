use adapton::macros::*;
use adapton::engine::*;
use adapton::reflect;

use crate::diagnostics::Diagnostics;
use crate::graph::Graph;

pub struct GraphColoringComp {
    input_nodes_layer: Vec<Art<i32>>,
    granular_guards_layer: Vec<Vec<Art<bool>>>,
    guards_layer: Vec<Art<i32>>,
    computations_layer: Vec<Art<i32>>,
    invalid_edges_layer: Vec<Art<i32>>,
    result: Option<Art<i32>>,
    sealed: bool,
    number_of_colors: i32,
    diagnostics: Option<Diagnostics>,
    graph: Rc<Graph>
}

impl GraphColoringComp {
    pub fn new(graph: Rc<Graph>, n: usize) -> GraphColoringComp {
        manage::init_dcg();

        if cfg!(feature = "traces") {
            println!("GraphColoringComp: traces enabled");
            reflect::dcg_reflect_begin();
        }

        let input_nodes_layer = (0..n).map(|_| {
            cell!(0)
        }).collect();
        
        GraphColoringComp {
            input_nodes_layer,
            guards_layer: Vec::new(),
            granular_guards_layer: Vec::new(),
            computations_layer: Vec::new(),
            invalid_edges_layer: Vec::new(),
            result: None,
            sealed: false,
            number_of_colors: n as i32,
            diagnostics: None,
            graph
        }
    }

    pub fn seal(&mut self) {
        self.ensure_unsealed();
        self.sealed = true;

        if cfg!(feature = "traces") {
            let traces = reflect::dcg_reflect_end();
            let diagnostics = Diagnostics::new(traces).analyse();
            self.diagnostics = Some(diagnostics);
        }
    }

    pub fn update_input_node(&mut self, idx: usize, val: i32) {
        self.ensure_unsealed();
        set(&self.input_nodes_layer[idx], val);
    }

    pub fn create_computation_graph(&mut self) {
        self.create_guards_layer();
        self.create_invalid_edges_layer();
        self.create_computation_layer();
        self.create_final_layer();    
    }

    fn ensure_unsealed(&mut self) {
        assert!(!self.sealed, "Graph Coloring is sealed");
    }

    fn create_guards_layer(&mut self) {
        let guards_layer = (0..self.number_of_colors)
            .map(|c| {
                let guards = self.input_nodes_layer.iter().map(|input_node| {
                    let input_node_clone = input_node.clone();
                    thunk!(get!(input_node_clone) == c)
                }).collect::<Vec<Art<bool>>>();
                self.granular_guards_layer.push(guards.clone());
                thunk!(guards.iter().fold(0, |acc, guard| acc + i32::from(get!(guard))))
            })
            .collect::<Vec<Art<i32>>>();
        
        self.guards_layer = guards_layer;
    }

    fn create_invalid_edges_layer(&mut self) {
        let invalid_edges_layer = self.granular_guards_layer
            .iter()
            .enumerate()
            .map(|(c, granular_guards)| {
                let graph_rc = Rc::clone(&self.graph);
                let granular_guards_clone = granular_guards.clone();
                let invalid_edges_thunk_res = thunk![{
                    let mut nodes: Vec<usize> = Vec::new();
                    for (i, g) in granular_guards_clone.iter().enumerate() {
                        if get!(g) {
                            nodes.push(i);
                        }
                    }
                    
                    let mut invalid_edges = 0;
                    for i in 0..nodes.len() {
                        for j in i+1..nodes.len() {
                            let edge = graph_rc.get_edge_from_lookup(nodes[i] as i32, nodes[j] as i32);
                            match edge {
                                Some(_) => invalid_edges += 1,
                                None => ()
                            }
                        }
                    }

                    invalid_edges
                }];

                invalid_edges_thunk_res
            }).collect::<Vec<Art<i32>>>();

        self.invalid_edges_layer = invalid_edges_layer;
    }

    fn create_computation_layer(&mut self) {
        let computations_layer = (0..self.number_of_colors)
            .map(|c| {
                let invalid_edges = self.invalid_edges_layer[c as usize].clone();
                let vertecies_of_color = self.guards_layer[c as usize].clone();

                let computation = thunk![{
                    let invalid_edges = get!(invalid_edges);
                    let vertecies_of_color = get!(vertecies_of_color);
                    2 * vertecies_of_color * invalid_edges + vertecies_of_color.pow(2)
                }];

                computation
            }).collect::<Vec<Art<i32>>>();


        self.computations_layer = computations_layer;
    }

    fn create_final_layer(&mut self) -> &Art<i32> {
       let layer = self.computations_layer.clone();
        
        let res = thunk!(
            layer.iter().fold(0, |acc, node| {
                let node_clone = node.clone();
                acc + i32::from(get!(node_clone))
            })
        );

        self.result = Some(res);
        self.result.as_ref().unwrap()
    }

    pub fn get_result(&self) -> Option<i32> {
       self.result.as_ref().and_then(|res| Some(i32::from(get!(res))))
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Point;

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
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::new(Graph::default()), 3);
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
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::new(Graph::default()), 3);
        graph_coloring_comp.create_guards_layer();
        assert!(graph_coloring_comp.guards_layer.len() == 3, "Guards layer should have 3 guards");

        get!(graph_coloring_comp.guards_layer[0]);
        get!(graph_coloring_comp.guards_layer[1]);
        get!(graph_coloring_comp.guards_layer[2]);
        graph_coloring_comp.update_input_node(1, 1);
        get!(graph_coloring_comp.guards_layer[0]);
        get!(graph_coloring_comp.guards_layer[1]);
        get!(graph_coloring_comp.guards_layer[2]);

        graph_coloring_comp.seal();
        let diagnostics = graph_coloring_comp.diagnostics;
        
        if let Some(diag) = diagnostics {
            assert!(diag.cells_count == 3, "Cells count should be 3");
            assert!(diag.thunks_count == 12, "Thunks count should be 12");
        }
    }

    #[test]
    fn test_computation_layer() {
        let mut graph = Graph::new();
        graph.add_nodes((0..4).map(|_| Point::random()).collect());
        graph.add_2d_edge(0, 2);
        graph.add_2d_edge(0, 3);
        graph.add_2d_edge(1, 2);
        graph.add_2d_edge(1, 3);

        let graph_rc = Rc::new(graph);
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::clone(&graph_rc), 4);
        graph_coloring_comp.create_computation_graph();

        let result = graph_coloring_comp.get_result();

        // 16 + 2 * 4 * 4 = 48
        // vertecies_of_color**2 + 2 * vertecies_of_color * invalid_edges
        assert_eq!(result, Some(48), "Result should be 48");
        graph_coloring_comp.update_input_node(0, 1);
        let result = graph_coloring_comp.get_result();

        // 0: 9 + 2 * 3 * 2 = 21
        // 1: 1 + 2 * 1 * 0 = 1
        // 22 
        assert_eq!(result, Some(22), "Result should be 22");
        graph_coloring_comp.update_input_node(1, 1);
        let result = graph_coloring_comp.get_result();

        // 0: 4 + 2 * 2 * 0 = 4
        // 1: 4 + 2 * 2 * 0 = 4
        // 8
        assert_eq!(result, Some(8), "Result should be 8");

        graph_coloring_comp.seal();
        
        if let Some(diag) = graph_coloring_comp.diagnostics {
            assert_eq!(diag.cells_count, 4, "Cells count should be 4");

            // granular_layer: 16
            // guards_layer: 4
            // invalid_edges_layer: 4
            // computations_layer: 4
            // final_layer: 1
            // total: 29
            assert_eq!(diag.thunks_count, 29, "Thunks count should be 29");
        }
    }

    #[test]
    fn test_invalid_edges_layer() {
        let mut graph = Graph::new();
        graph.add_nodes((0..4).map(|_| Point::random()).collect());
        graph.add_2d_edge(0, 2);
        graph.add_2d_edge(0, 3);
        graph.add_2d_edge(1, 2);
        graph.add_2d_edge(1, 3);

        let graph_rc = Rc::new(graph);
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::clone(&graph_rc), 4);
        graph_coloring_comp.create_guards_layer();
        graph_coloring_comp.create_invalid_edges_layer();

        let invalid_edges_layer = graph_coloring_comp.invalid_edges_layer.clone();
        let invalid_edges = invalid_edges_layer.iter().map(|invalid_edges| {
            get!(invalid_edges)
        }).collect::<Vec<i32>>();

        assert_eq!(invalid_edges, vec![4, 0, 0, 0], "Invalid edges should be [4, 0, 0, 0]");

        graph_coloring_comp.update_input_node(0, 1);

        let invalid_edges = invalid_edges_layer.iter().map(|invalid_edges| {
            get!(invalid_edges)
        }).collect::<Vec<i32>>();

        assert_eq!(invalid_edges, vec![2, 0, 0, 0], "Invalid edges should be [2, 0, 0, 0]");

        graph_coloring_comp.update_input_node(1, 1);

        let invalid_edges = invalid_edges_layer.iter().map(|invalid_edges| {
            get!(invalid_edges)
        }).collect::<Vec<i32>>();

        assert_eq!(invalid_edges, vec![0, 0, 0, 0], "Invalid edges should be [0, 0, 0, 0]");
        
    }
}