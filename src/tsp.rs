use std::rc::Rc;

use crate::graph::{Graph, PointId};
use nannou::rand;

pub type TspPath = Vec<PointId>;

pub struct Tsp {
    pub graph:  Rc<Graph>,
    path: TspPath,
    pub history: Vec<TspPath>
}

impl Tsp {
    pub fn new(graph: Rc<Graph>) -> Tsp {
        Tsp {
            graph,
            path: Vec::new(),
            history: Vec::new()
        }
    }

    pub fn set_starting_path(&mut self, path: TspPath) {
        self.path = path;
    }

    pub fn generate_starting_path(&mut self) -> TspPath {
        let mut path = Vec::new();
        let n = self.graph.nodes.len() as i32;
        let mut vertecies = (0..n).collect::<Vec<i32>>();

        while !vertecies.is_empty() {
            let next_vertex = rand::random_range(0, vertecies.len() as i32);
            path.push(vertecies.swap_remove(next_vertex as usize));
        }

        self.set_starting_path(path);
        self.path.clone()
    }

    pub fn calculate_path_length(&self) -> i32 {
        let mut length = 0;
        let n = self.path.len();
        for i in 0..self.path.len() {
            let u = self.path[i];
            let v = self.path[(i + 1) % n];
            length += self.graph.get_edge_from_lookup(u, v).unwrap().weight;
        }

        length
    }

    fn swap_edges(&mut self, mut i: usize, mut j: usize) {
        i += 1;
        while i < j {
            self.path.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    pub fn tsp_2_opt(&mut self) -> Result<i32, ()> {
        let mut best_length = self.calculate_path_length();
        let n = self.path.len() as usize;
        let mut improved = true;

        let mut history: Vec<TspPath> = Vec::new();

        println!("initial length: {}", best_length);

        while improved {
            improved = false;
            for i in 0..n-1 {
                for j in i+2..n {
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

                }
            }
        }

        self.history = history;
        Ok(best_length)
    }
}

impl From<Graph> for Tsp {
    fn from(graph: Graph) -> Tsp {
        Tsp::new(Rc::new(graph))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsp() {
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
        let mut tsp = Tsp::new(Rc::clone(&graph));
        tsp.set_starting_path(path);

        let length = tsp.tsp_2_opt().unwrap();
        assert_eq!(length, 5);
    } 
}