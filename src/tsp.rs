use std::rc::Rc;

use crate::graph::{Graph, PointId};
use crate::tsp_comp::TspComp;
use nannou::rand;

pub type TspPath = Vec<PointId>;

pub enum ScoreCalcTypeTSP {
    Fast,
    Slow,
    Incremental
}

pub struct Tsp {
    graph:  Rc<Graph>,
    history: Vec<TspPath>,
    path: TspPath,
    score_calc_type: ScoreCalcTypeTSP,
    computation_graph: Option<TspComp>,
}

fn unsafe_create_static_pointer(al: Vec<Vec<i32>>) -> &'static Vec<Vec<i32>> {
    static mut AL: *const Vec<Vec<i32>> = std::ptr::null();

    unsafe {
        AL = Box::into_raw(Box::new(al));
    }

    unsafe { &*AL }
}

impl Tsp {
    pub fn new(graph: Rc<Graph>, score_calc_type: ScoreCalcTypeTSP) -> Tsp {
        let number_of_nodes = graph.get_number_of_nodes();

        let computation_graph = match score_calc_type {
            ScoreCalcTypeTSP::Incremental => {
                let al = graph.get_raw_adjacency_list();
                let static_al = unsafe_create_static_pointer(al);
                Some(TspComp::new(static_al, number_of_nodes))
            },
            _ => None
        };

        Tsp {
            graph,
            path: Vec::new(),
            history: Vec::new(),
            computation_graph,
            score_calc_type
        }
    }

    pub fn set_starting_path(&mut self, path: TspPath) {
        let n = self.graph.get_number_of_nodes() as i32;
        if let Some(comp_graph) = &mut self.computation_graph {
            comp_graph.update_input_nodes((0..n).map(|i| (i as usize, path[i as usize])).collect());
        }

        self.path = path;
    }

    pub fn generate_starting_path(&mut self) -> TspPath {
        let mut path = Vec::new();
        let n = self.graph.get_number_of_nodes() as i32;
        let mut vertecies = (0..n).collect::<Vec<i32>>();

        while !vertecies.is_empty() {
            let next_vertex = rand::random_range(0, vertecies.len() as i32);
            path.push(vertecies.swap_remove(next_vertex as usize));
        }

        self.set_starting_path(path);
        self.path.clone()
    }

    fn calculate_path_length_naive(&self) -> i32 {
        let mut length = 0;
        let n = self.path.len();
        for i in 0..(self.path.len()) {
            let u = self.path[i];
            let v = self.path[(i + 1) % n];
            length += self.graph.get_edge_from_lookup(u, v).unwrap().weight;
        }

        length
    }

    fn calculate_path_length(&self) -> i32 {
        match &self.score_calc_type {
            ScoreCalcTypeTSP::Incremental => {
                self.computation_graph.as_ref().unwrap().get_result()
            },
            _ => {
                self.calculate_path_length_naive()
            }
        }
    }

    fn swap_edges(&mut self, mut i: usize, mut j: usize) {
        let mut updates: Vec<(usize, i32)> = Vec::new();

        i += 1;
        while i < j {
            self.path.swap(i, j);
            updates.push((i, self.path[i]));
            updates.push((j, self.path[j]));
            i += 1;
            j -= 1;
        }

       if let Some(comp_graph) = &mut self.computation_graph {
            comp_graph.update_input_nodes(updates);
        }
    }

    fn finish(&mut self) {
        if let Some(comp_graph) = &mut self.computation_graph {
            comp_graph.seal();
        }
    }

    pub fn tsp(&mut self) -> Result<i32, ()> {
        let mut best_length = self.calculate_path_length();
        let n = self.path.len() as usize;
        let mut improved = true;

        let mut history: Vec<TspPath> = Vec::new();

        while improved {
            improved = false;
            for i in 0..n-1 {
                for j in i+2..n {
                    match &self.score_calc_type {
                        ScoreCalcTypeTSP::Fast => {
                            let e1 = self.graph.get_edge_from_lookup(self.path[i], self.path[i+1]).unwrap().weight;
                            let e2 = self.graph.get_edge_from_lookup(self.path[j], self.path[(j+1)%n]).unwrap().weight;
                            let ne1 = self.graph.get_edge_from_lookup(self.path[i], self.path[j]).unwrap().weight;
                            let ne2 = self.graph.get_edge_from_lookup(self.path[i+1], self.path[(j+1)%n]).unwrap().weight;
                            
                            let delta = (ne1 + ne2) - (e1 + e2);

                            if delta < 0 {
                                self.swap_edges(i, j);
                                improved = true;
                                best_length += delta;
                                history.push(self.path.clone());
                            }
                        },
                        _ => {
                            self.swap_edges(i, j);
                            let new_length = self.calculate_path_length();

                            if new_length < best_length {
                                best_length = new_length;
                                improved = true;
                                history.push(self.path.clone());
                            } else {
                                // reverse
                                self.swap_edges(i, j);
                            }
                        }
                    }
                }
            }
        }

        self.finish();
        self.history = history;
        Ok(best_length)
    }

    pub fn get_history(&self) -> &Vec<TspPath> {
        &self.history
    }

    pub fn get_path(&self) -> &TspPath {
        &self.path
    }
}

impl From<Graph> for Tsp {
    fn from(graph: Graph) -> Tsp {
        Tsp::new(Rc::new(graph), ScoreCalcTypeTSP::Fast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsp_slow() {
        let size = 5;
        let al = vec![
            vec![0, 1, 7, 6, 1],
            vec![1, 0, 1, 4, 9],
            vec![7, 1, 0, 1, 8],
            vec![6, 4, 1, 0, 1],
            vec![1, 9, 8, 1, 0]
        ];

        let graph = Rc::new(Graph::from((size, al)));
        let path = vec![4, 3, 0, 2, 1];
        let mut tsp = Tsp::new(Rc::clone(&graph), ScoreCalcTypeTSP::Slow);
        tsp.set_starting_path(path);

        let length = tsp.tsp().unwrap();
        assert_eq!(length, 5);

        let path = tsp.get_path();
        assert_eq!(path, &vec![4, 0, 1, 2, 3]);
    }

    #[test]
    fn test_tsp_fast() {
        let size = 5;
        let al = vec![
            vec![0, 1, 7, 6, 1],
            vec![1, 0, 1, 4, 9],
            vec![7, 1, 0, 1, 8],
            vec![6, 4, 1, 0, 1],
            vec![1, 9, 8, 1, 0]
        ];

        let graph = Rc::new(Graph::from((size, al)));
        let path = vec![4, 3, 0, 2, 1];
        let mut tsp = Tsp::new(Rc::clone(&graph), ScoreCalcTypeTSP::Fast);
        tsp.set_starting_path(path);

        let length = tsp.tsp().unwrap();
        assert_eq!(length, 5);

        let path = tsp.get_path();
        assert_eq!(path, &vec![4, 0, 1, 2, 3]);
    }

    #[test]
    fn test_tsp_incremental() {
        let size = 5;
        let al = vec![
            vec![0, 1, 7, 6, 1],
            vec![1, 0, 1, 4, 9],
            vec![7, 1, 0, 1, 8],
            vec![6, 4, 1, 0, 1],
            vec![1, 9, 8, 1, 0]
        ];

        let graph = Rc::new(Graph::from((size, al)));
        let path = vec![4, 3, 0, 2, 1];
        let mut tsp = Tsp::new(Rc::clone(&graph), ScoreCalcTypeTSP::Incremental);
        tsp.set_starting_path(path);

        let length = tsp.tsp().unwrap();
        assert_eq!(length, 5);
    }

    #[test]
    fn test_swap_edges() {
        let size = 5;
        let al = vec![
            vec![0, 1, 7, 6, 1],
            vec![1, 0, 1, 4, 9],
            vec![7, 1, 0, 1, 8],
            vec![6, 4, 1, 0, 1],
            vec![1, 9, 8, 1, 0]
        ];

        let graph = Rc::new(Graph::from((size, al)));
        let path = vec![4, 3, 0, 2, 1];
        let mut tsp = Tsp::new(Rc::clone(&graph), ScoreCalcTypeTSP::Slow);
        tsp.set_starting_path(path);

        let length = tsp.tsp().unwrap();
        assert_eq!(length, 5);

        tsp.swap_edges(1, 3);
        tsp.swap_edges(1, 3);
        let length2 = tsp.calculate_path_length();

        assert_eq!(length, length2);

        tsp.swap_edges(0, 4);
        tsp.swap_edges(0, 4);
        let length3 = tsp.calculate_path_length();

        assert_eq!(length, length3);
    } 
}

