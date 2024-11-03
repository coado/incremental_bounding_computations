use nannou::rand;
use std::collections::HashMap;

pub const EPS: f64 = 1e-9;

pub type PointId = i32;
type EdgeId = i32;
type Top = f64;
type Bottom = f64;
type Left = f64;
type Right = f64;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Edge {
    pub p1: PointId,
    pub p2: PointId,
    pub weight: i32,
}

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Point>,
    pub edges: Vec<Edge>,
    pub adj_list: Vec<Vec<EdgeId>>,
    pub boundary: (Top, Bottom, Left, Right),
    edges_lookup: HashMap<(PointId, PointId), EdgeId>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            adj_list: Vec::new(),
            boundary: (0.0, 100.0, 0.0, 100.0),
            edges_lookup: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: i32, v: i32, weight: i32) {
        let n = self.adj_list.len() as i32;
        assert!(
            u >= 0 && u < n && v >= 0 && v < n,
            "Node id out of range",
        );
        assert!(u != v, "Self edge is not allowed");

        let id = self.edges.len() as i32;
        let edge = Edge { p1: u, p2: v, weight };
        self.edges.push(edge);
        self.adj_list[u as usize].push(id);
        self.adj_list[v as usize].push(id);
        self.edges_lookup.insert((u.min(v), u.max(v)), id);
    }

    fn add_nodes(&mut self, points: Vec<Point>) {
        for point in points {
            self.nodes.push(point);
            self.adj_list.push(Vec::new());
        }
    }


    pub fn get_edge_from_lookup(&self, u: PointId, v: PointId) -> Option<&Edge> {
        let key = (u.min(v), u.max(v));
        self.edges_lookup.get(&key).map(|&id| &self.edges[id as usize])
    }

    pub fn fill_with_random_points(&mut self, n: i32) {
        assert!(self.nodes.is_empty(), "Graph must be empty");

        for _ in 0..n {
            let x = rand::random_range(self.boundary.2, self.boundary.3);
            let y = rand::random_range(self.boundary.0, self.boundary.1);
            self.nodes.push(Point::new(x, y));
            self.adj_list.push(Vec::new());
        }
    }

    pub fn fill_with_edges(&mut self) {
        assert!(self.edges.is_empty(), "Graph must have no edges");
        assert!(self.nodes.len() > 1, "Graph must have at least 2 nodes");

        let n = self.nodes.len();
        for u in 0..n {
            for v in u+1..n {
                let weight = ((self.nodes[u].x - self.nodes[v].x).powi(2) + (self.nodes[u].y - self.nodes[v].y).powi(2)).sqrt() as i32;
                self.add_edge(u as i32, v as i32, weight);
            }
        }
    }
}

// Creating graph from adjacency list and number of nodes
impl From<(usize, Vec<Vec<i32>>)> for Graph {
    fn from(data: (usize, Vec<Vec<i32>>)) -> Self {
        let (size, adj_list) = data;
        assert!(size > 0, "Graph must have at least 1 node");
        assert!(size == adj_list.len(), "Adjacency list must have the same size as the number of nodes");

        let mut graph = Graph::new();
        let nodes = (0..size)
            .map(|_| Point::new(rand::random_range(0.0, 100.0), rand::random_range(0.0, 100.0)))
            .collect();

        graph.add_nodes(nodes);

        for (u, adj) in adj_list.iter().enumerate() {
            for (v, &weight) in adj.iter().enumerate() {
                if u < v {
                    graph.add_edge(u as i32, v as i32, weight);
                }
            }
        }

        graph
    }
}