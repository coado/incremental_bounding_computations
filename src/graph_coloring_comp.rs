use adapton::macros::*;
use adapton::engine::*;
use adapton::reflect;

use crate::diagnostics::Diagnostics;
use crate::graph::Graph;

#[derive(Debug, Default)]
pub struct GraphColouringFlags {
    pub enable_firewall: bool,
    pub enable_dynamic_branches: bool,
    pub merge_computation_layers: bool,
}

#[derive(Debug, Clone)]
enum Guards {
    Normal(Vec<Art<bool>>),
    Firewall(Vec<Art<Art<bool>>>)
}

impl Guards {
    fn len(&self) -> usize {
        match self {
            Guards::Normal(guards) => guards.len(),
            Guards::Firewall(guards) => guards.len()
        }
    }
}

impl GraphColouringFlags {
    pub fn new(enable_firewall: bool, enable_dynamic_branches: bool, merge_computation_layers: bool) -> GraphColouringFlags {
        GraphColouringFlags {
            enable_firewall,
            enable_dynamic_branches,
            merge_computation_layers
        }
    }

    pub fn default() -> GraphColouringFlags {
        GraphColouringFlags {
            enable_firewall: false,
            enable_dynamic_branches: false,
            merge_computation_layers: false
        }
    }
}

pub struct GraphColoringComp {
    input_nodes_layer: Vec<Art<i32>>,
    computation_nodes_layer: Vec<Art<i32>>,
    result: Option<Art<i32>>,
    sealed: bool,
    max_number_of_colours: i32,
    used_colours: usize,
    diagnostics: Option<Diagnostics>,
    graph: Rc<Graph>,
    flags: GraphColouringFlags
}

impl GraphColoringComp {
    pub fn new(graph: Rc<Graph>, n: usize, flags: GraphColouringFlags) -> GraphColoringComp {
        manage::init_dcg();

        if cfg!(feature = "traces") {
            reflect::dcg_reflect_begin();
        }

        let input_nodes_layer = (0..n).map(|_| {
            cell!(0)
        }).collect();
        
        GraphColoringComp {
            input_nodes_layer,
            computation_nodes_layer: Vec::new(),
            result: None,
            sealed: false,
            max_number_of_colours: n as i32,
            used_colours: 1,
            diagnostics: None,
            graph,
            flags
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
        assert!(val < self.max_number_of_colours, "Invalid colour");
        self.ensure_unsealed();
        
        if val == self.used_colours as i32 {
            if self.flags.enable_dynamic_branches {
                self.update_root_with_new_colour(val);
            }
            self.used_colours += 1;
        }

        set(&self.input_nodes_layer[idx], val);
    }

    pub fn create_computation_graph(&mut self) {
        let root_node = self.create_root_node();
        self.result = Some(root_node);
    }

    fn update_root_with_new_colour(&mut self, colour: i32) {
        let computation_node = self.create_branch(colour);
        self.computation_nodes_layer.push(computation_node);
        let root_node = self.create_final_layer();
        self.result = Some(root_node);
    }

    fn create_root_node(&mut self) -> Art<i32> {
        let mut computation_layer = Vec::new();
        match self.flags.enable_dynamic_branches {
            true => {
                // at this point we have only one colour
                let computation_node = self.create_branch(0);
                computation_layer.push(computation_node);
            },
            false => {
                for i in 0..self.max_number_of_colours {
                    computation_layer.push(self.create_branch(i));
                };
            }
        };
        
        self.computation_nodes_layer = computation_layer;
        self.create_final_layer()
    }

    fn create_branch(&self, colour: i32) -> Art<i32> {
        let guards_layer = self.create_guards_layer(colour);
        let computation_layer_node = self.create_computation_layer(&guards_layer);
        computation_layer_node
    }

    fn create_guards_layer(&self, colour: i32) -> Guards {
        match self.flags.enable_firewall {
            true => {
                let guards_layer = self.input_nodes_layer.iter().map(|input_node| {
                    let input_node_clone = input_node.clone();
                    thunk!({
                        let val = get!(input_node_clone);
                        cell!(val == colour)
                    })
                }).collect::<Vec<Art<Art<bool>>>>();

                Guards::Firewall(guards_layer)
            },
            false => {
                let guards_layer = self.input_nodes_layer.iter().map(|input_node| {
                    let input_node_clone = input_node.clone();
                    thunk!(get!(input_node_clone) == colour)
                }).collect::<Vec<Art<bool>>>();

                Guards::Normal(guards_layer)
            }
        }
    }

    fn create_computation_layer(&self, guards_layer: &Guards) -> Art<i32> {
        match self.flags.merge_computation_layers {
            true => {
                let graph_rc = Rc::clone(&self.graph);

                match guards_layer {
                    Guards::Normal(guards_layer) => {
                        let guards_layer_clone = guards_layer.clone();
                        thunk!({
                            let mut nodes: Vec<usize> = Vec::new();
                            for (i, g) in guards_layer_clone.iter().enumerate() {
                                // let is_active = force(&g);
                                if get!(g) {
                                    // push if node is in active state for this colour
                                    nodes.push(i);
                                }
                            }
                            
                            let mut invalid_edges = 0 as i32;
                            for i in 0..nodes.len() {
                                for j in i+1..nodes.len() {
                                    let edge = graph_rc.get_edge_from_lookup(nodes[i] as i32, nodes[j] as i32);
                                    match edge {
                                        Some(_) => invalid_edges += 1,
                                        None => ()
                                    }
                                }
                            }
        
                            let vertices_of_colour = nodes.len() as i32;
                            let result = 2 * vertices_of_colour * invalid_edges - vertices_of_colour.pow(2);
                            result as i32
                        })
                    },
                    Guards::Firewall(guards_layer) => {
                        let guards_layer_clone = guards_layer.clone();
                        thunk!({
                            let mut nodes: Vec<usize> = Vec::new();
                            for (i, g) in guards_layer_clone.iter().enumerate() {
                                let is_active = force(&g);
                                if get!(is_active) {
                                    // push if node is in active state for this colour
                                    nodes.push(i);
                                }
                            }
                            
                            let mut invalid_edges = 0 as i32;
                            for i in 0..nodes.len() {
                                for j in i+1..nodes.len() {
                                    let edge = graph_rc.get_edge_from_lookup(nodes[i] as i32, nodes[j] as i32);
                                    match edge {
                                        Some(_) => invalid_edges += 1,
                                        None => ()
                                    }
                                }
                            }
        
                            let vertices_of_colour = nodes.len() as i32;
                            let result = 2 * vertices_of_colour * invalid_edges - vertices_of_colour.pow(2);
                            result as i32
                        })
                    }
                }
            },
            false => {
                let invalid_edges_node = self.create_invalid_edges_node(guards_layer);
                let summing_node = self.create_summing_node(guards_layer);
                thunk!(2 * get!(summing_node) * get!(invalid_edges_node) - get!(summing_node).pow(2))
            }
        }
    }

    fn create_final_layer(&self) -> Art<i32> {
        let computation_nodes_layer_clone = self.computation_nodes_layer.clone();
        let root_node = thunk!({
            computation_nodes_layer_clone.iter().fold(0, |acc, node| {
                let node_clone = node.clone();
                acc + i32::from(get!(node_clone))
            })
        });

        root_node
    }

    fn create_invalid_edges_node(&self, guards_layer: &Guards) -> Art<i32> {
        let graph_rc = Rc::clone(&self.graph);

        match guards_layer {
            Guards::Normal(guards_layer) => {
                let guards_layer_clone = guards_layer.clone();
                thunk![{
                    let mut nodes: Vec<usize> = Vec::new();
                    for (i, g) in guards_layer_clone.iter().enumerate() {
                        // let is_active = force(&g);
                        if get!(g) {
                            // push if node is in active state for this colour
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
                }]
            },
            Guards::Firewall(guards_layer) => {
                let guards_layer_clone = guards_layer.clone();
                thunk![{
                    let mut nodes: Vec<usize> = Vec::new();
                    for (i, g) in guards_layer_clone.iter().enumerate() {
                        let is_active = force(&g);
                        if get!(is_active) {
                            // push if node is in active state for this colour
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
                }]
            }
        }
    }

    fn create_summing_node(&self, guards_layer: &Guards) -> Art<i32> {

        match guards_layer {
            Guards::Normal(guards_layer) => {
                let guards_layer_clone = guards_layer.clone();
                thunk!(guards_layer_clone.iter().fold(0, |acc, guard| {
                    acc + i32::from(get!(&guard))
                }))
            },
            Guards::Firewall(guards_layer) => {
                let guards_layer_clone = guards_layer.clone();
                thunk!(guards_layer_clone.iter().fold(0, |acc, guard| {
                    let is_active = force(&guard);
                    acc + i32::from(get!(&is_active))
                }))
            }
        }
    }

    pub fn get_result(&self) -> Option<i32> {
       self.result.as_ref().and_then(|res| Some(i32::from(get!(res))))
    }

    fn ensure_unsealed(&mut self) {
        assert!(!self.sealed, "Graph Coloring is sealed");
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Point;

    use super::*;
    use std::{result, sync::Once};

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

    fn make_result_tests(flags: GraphColouringFlags) -> GraphColoringComp {
        let mut graph = Graph::new();
        graph.add_nodes((0..4).map(|_| Point::random()).collect());
        graph.add_2d_edge(0, 2);
        graph.add_2d_edge(0, 3);
        graph.add_2d_edge(1, 2);
        graph.add_2d_edge(1, 3);

        let graph_rc = Rc::new(graph);
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::clone(&graph_rc), 4, flags);
        graph_coloring_comp.create_computation_graph();

        let result = graph_coloring_comp.get_result();

        // 2 * 4 * 4 - 16 = 16
        // -vertecies_of_color**2 + 2 * vertecies_of_color * invalid_edges
        assert_eq!(result, Some(16), "Result should be 16");
        assert_eq!(graph_coloring_comp.used_colours, 1, "Used colours should be 1");
        graph_coloring_comp.update_input_node(0, 1);
        let result = graph_coloring_comp.get_result();

        // 0: -9 + 2 * 3 * 2 = 3
        // 1: -1 + 2 * 1 * 0 = -1
        // 2 
        assert_eq!(result, Some(2), "Result should be 2");
        assert_eq!(graph_coloring_comp.used_colours, 2, "Used colours should be 2");
        graph_coloring_comp.update_input_node(1, 1);
        let result = graph_coloring_comp.get_result();

        // 0: -4 + 2 * 2 * 0 = -4
        // 1: -4 + 2 * 2 * 0 = -4
        // -8
        assert_eq!(result, Some(-8), "Result should be -8");

        graph_coloring_comp
    }

    #[test]
    fn test_guards_layer() {
        let n = 3;
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::new(Graph::default()), n, GraphColouringFlags::default());
        
        let guards_layer_zero: Guards = graph_coloring_comp.create_guards_layer(0);
        let guards_layer_one = graph_coloring_comp.create_guards_layer(1);
        let guards_layer_two = graph_coloring_comp.create_guards_layer(2);
        
        assert!(guards_layer_zero.len() == 3, "Guards layer should have 3 guards");
        assert!(guards_layer_one.len() == 3, "Guards layer should have 3 guards");
        assert!(guards_layer_two.len() == 3, "Guards layer should have 3 guards");

        let guards_layer_zero_clone = guards_layer_zero.clone();
        let guards_layer_one_clone = guards_layer_one.clone();
        let guards_layer_two_clone = guards_layer_two.clone();

        for i in 0..n {
            if let Guards::Normal(guards) = &guards_layer_zero_clone {
                assert_eq!(get!(guards[i]), true);
            }
            if let Guards::Normal(guards) = &guards_layer_one_clone {
                assert_eq!(get!(guards[i]), false);
            }
            if let Guards::Normal(guards) = &guards_layer_two_clone {
                assert_eq!(get!(guards[i]), false);
            }
        }

        graph_coloring_comp.update_input_node(1, 1);
        graph_coloring_comp.update_input_node(2, 2);

        for i in 0..n {
            if let Guards::Normal(guards) = &guards_layer_zero_clone {
                assert_eq!(get!(guards[i]), i == 0);
            }
            if let Guards::Normal(guards) = &guards_layer_one_clone {
                assert_eq!(get!(guards[i]), i == 1);
            }
            if let Guards::Normal(guards) = &guards_layer_two_clone {
                assert_eq!(get!(guards[i]), i == 2);
            }
        }
    }

    #[test]
    fn test_diagnostics() {
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::new(Graph::default()), 3, GraphColouringFlags::default());
        let guards_layer = graph_coloring_comp.create_guards_layer(0);
        assert!(guards_layer.len() == 3, "Guards layer should have 3 guards");

        if let Guards::Normal(guards) = &guards_layer {
            get!(guards[0]);
            get!(guards[1]);
            get!(guards[2]);
        }
        graph_coloring_comp.update_input_node(0, 1);
        if let Guards::Normal(guards) = &guards_layer {
            get!(guards[0]);
            get!(guards[1]);
            get!(guards[2]);
        }
        graph_coloring_comp.seal();
        let diagnostics = graph_coloring_comp.diagnostics;
        
        if let Some(diag) = diagnostics {
            assert!(diag.cells_count == 3, "Cells count should be 3");
            assert!(diag.thunks_count == 3, "Thunks count should be 12");
        }
    }

    #[test]
    fn test_computation_graph_default_flags() {
        let mut graph_coloring_comp = make_result_tests(GraphColouringFlags::default());

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
    fn test_computation_graph_merge_layers_flag() {
        let mut graph_coloring_comp = make_result_tests(GraphColouringFlags::new(false, false, true));

        graph_coloring_comp.seal();
        if let Some(diag) = graph_coloring_comp.diagnostics {
            assert_eq!(diag.cells_count, 4, "Cells count should be 4");

            // granular_layer: 16
            // computations_layer: 4
            // final_layer: 1
            // total: 21
            assert_eq!(diag.thunks_count, 21, "Thunks count should be 21");
        }
    }

    #[test]
    fn test_computation_graph_dynamic_branches_flag() {
        let mut graph_coloring_comp = make_result_tests(GraphColouringFlags::new(false, true, false));

        graph_coloring_comp.seal();
        if let Some(diag) = graph_coloring_comp.diagnostics {
            assert_eq!(diag.cells_count, 4, "Cells count should be 4");

            // guards_layer: 8
            // invalid_edges_layer: 2
            // summing_layer: 2
            // computations_layer: 2
            // previous final_layer: 1
            // final_layer: 1
            // total: 16
            // for now it will duplicate final layers each time the new colour is added
            // but we will only use the most recent one to calculate the result
            assert_eq!(diag.thunks_count, 16, "Thunks count should be 16");
        }
    }

    #[test]
    fn test_computation_graph_enable_firewall_flag() {
        make_result_tests(GraphColouringFlags::new(true, false, false));
    }

    #[test]
    fn test_computation_graph_enable_firewall_and_dynamic_branches_flag() {
        make_result_tests(GraphColouringFlags::new(true, true, false));
    }

    #[test]
    fn test_computation_graph_enable_firewall_and_merge_computations_flag() {
        make_result_tests(GraphColouringFlags::new(true, false, true));
    }

    #[test]
    fn test_computation_graph_enable_dynamic_branches_and_merge_computations_flag() {
        make_result_tests(GraphColouringFlags::new(false, true, true));
    }

    #[test]
    fn test_computation_graph_enable_all_flags() {
        make_result_tests(GraphColouringFlags::new(true, true, true));
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
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::clone(&graph_rc), 4, GraphColouringFlags::default());
        
        let guards_layer_zero = graph_coloring_comp.create_guards_layer(0);
        let guards_layer_one = graph_coloring_comp.create_guards_layer(1);
        let guards_layer_two = graph_coloring_comp.create_guards_layer(2);
        let guards_layer_three = graph_coloring_comp.create_guards_layer(3);

        let invalid_edges_node_zero = graph_coloring_comp.create_invalid_edges_node(&guards_layer_zero);
        let invalid_edges_node_one = graph_coloring_comp.create_invalid_edges_node(&guards_layer_one);
        let invalid_edges_node_two = graph_coloring_comp.create_invalid_edges_node(&guards_layer_two);
        let invalid_edges_node_three = graph_coloring_comp.create_invalid_edges_node(&guards_layer_three);

        let invalid_edges = vec![
            get!(invalid_edges_node_zero),
            get!(invalid_edges_node_one),
            get!(invalid_edges_node_two),
            get!(invalid_edges_node_three)
        ];

        assert_eq!(invalid_edges, vec![4, 0, 0, 0], "Invalid edges should be [4, 0, 0, 0]");

        graph_coloring_comp.update_input_node(0, 1);

        let invalid_edges = vec![
            get!(invalid_edges_node_zero),
            get!(invalid_edges_node_one),
            get!(invalid_edges_node_two),
            get!(invalid_edges_node_three)
        ];

        assert_eq!(invalid_edges, vec![2, 0, 0, 0], "Invalid edges should be [2, 0, 0, 0]");

        graph_coloring_comp.update_input_node(1, 1);

        let invalid_edges = vec![
            get!(invalid_edges_node_zero),
            get!(invalid_edges_node_one),
            get!(invalid_edges_node_two),
            get!(invalid_edges_node_three)
        ];

        assert_eq!(invalid_edges, vec![0, 0, 0, 0], "Invalid edges should be [0, 0, 0, 0]");
        
    }

    #[test]
    fn test_new_color() {
        let mut graph = Graph::new();
        graph.add_nodes((0..5).map(|_| Point::random()).collect());
        graph.add_2d_edge(0, 1);
        graph.add_2d_edge(0, 4);
        graph.add_2d_edge(1, 3);
        graph.add_2d_edge(1, 2);
        graph.add_2d_edge(2, 4);
        graph.add_2d_edge(2, 3);
        graph.add_2d_edge(3, 4);

        let graph_rc = Rc::new(graph);
        let mut graph_coloring_comp = GraphColoringComp::new(Rc::clone(&graph_rc), 5, GraphColouringFlags::default());
        graph_coloring_comp.create_computation_graph();
        let result = graph_coloring_comp.get_result();

        assert_eq!(result, Some(45), "Result should be 45");

        graph_coloring_comp.update_input_node(0, 1);
        let result = graph_coloring_comp.get_result();
        assert_eq!(result, Some(23), "Result should be 23");

        graph_coloring_comp.update_input_node(1, 2);
        let result = graph_coloring_comp.get_result();
        assert_eq!(result, Some(7), "Result should be 7");

        graph_coloring_comp.update_input_node(2, 1);
        let result = graph_coloring_comp.get_result();
        assert_eq!(result, Some(-5), "Result should be -5");

        graph_coloring_comp.update_input_node(4, 2);
        let result = graph_coloring_comp.get_result();
        assert_eq!(result, Some(-9), "Result should be -9");
    }
}