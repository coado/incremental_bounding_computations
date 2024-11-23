use std::{rc::Rc};
use crate::graph::{Graph, PointId};
use nannou::rand;

const NUMBER_OF_ITERATIONS: i32 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color(pub i32);

impl PartialEq<Color> for i32 {
    fn eq(&self, other: &Color) -> bool {
        self == &other.0
    }
}

impl PartialEq<i32> for Color {
    fn eq(&self, other: &i32) -> bool {
        &self.0 == other
    }
}

pub struct GraphColoring {
    graph: Rc<Graph>,
    pub coloring: Vec<Color>,
    pub number_of_colors: i32,
    pub history: Vec<Vec<Color>>,
    // colors_buckets: HashMap<Color, Vec<PointId>>,
    // violating_edges_buckets: HashMap<Color, Vec<EdgeId>>
}

impl GraphColoring {
    pub fn new(graph: Rc<Graph>) -> GraphColoring {
        let number_of_nodes = graph.get_number_of_nodes() as i32;
        let coloring = (0..number_of_nodes)
            .map(|_| Color(0))
            .collect::<Vec<Color>>();

        // let mut colors_buckets = HashMap::new();
        // let violating_edges_buckets = HashMap::new();
        // colors_buckets.insert(Color(0), (0..number_of_nodes).collect::<Vec<PointId>>());

        GraphColoring {
            graph,
            coloring,
            number_of_colors: 1,
            history: Vec::new(),
            // colors_buckets,
            // violating_edges_buckets
        }
    }

    // fn calculate_score(&self) -> i32 {
    //     let mut score = 0;

    //     for color in 0..self.number_of_colors {
    //         let number_of_nodes = self.colors_buckets[&Color(color)].len();
    //         let number_of_violating_edges = self.violating_edges_buckets[&Color(color)].len();

    //         score += number_of_nodes * (2 * number_of_violating_edges - number_of_nodes);
    //     }

    //     score as i32
    // }

    fn calculate_score_naive(&self) -> i32 {
        let mut score = 0;
        let mut colors_freq = (0..self.number_of_colors).map(|_| i32::default()).collect::<Vec<i32>>();
        let mut violating_edges_freq = colors_freq.clone();

        for u in 0..self.graph.get_number_of_nodes() as i32 {
            let color = self.coloring[u as usize].0;
            colors_freq[color as usize] += 1;

            for v in self.graph.get_adjacent_nodes(u) {
                if self.coloring[v as usize] == color {
                    violating_edges_freq[color as usize] += 1;
                }
            }
        }

        for color in 0..self.number_of_colors {
            let number_of_nodes = colors_freq[color as usize];
            let number_of_violating_edges = violating_edges_freq[color as usize];

            score += (number_of_nodes * (2 * number_of_violating_edges - number_of_nodes));
        }

        score
    }

    fn try_swap_color_operation(&mut self, vertex: PointId, best_score: i32) -> Option<(i32, Color)> {
        let mut current_best_score = best_score;
        let starting_color = self.coloring[vertex as usize];
        let mut best_color = starting_color;

        for c in 0..self.number_of_colors {
            if c == starting_color { continue; } 
            self.coloring[vertex as usize] = Color(c);
            let score = self.calculate_score_naive();

            if score < current_best_score {
                current_best_score = score;
                best_color = Color(c);
            }
        }

        self.coloring[vertex as usize] = starting_color;

        match best_color == starting_color {
            true => None,
            false => Some((current_best_score, best_color))
        }
    }

    fn try_new_color_operation(&mut self, vertex: PointId, best_score: i32) -> Option<(i32, Color)> {

        let starting_color = self.coloring[vertex as usize];

        self.coloring[vertex as usize] = Color(self.number_of_colors);
        self.number_of_colors += 1;
        let score = self.calculate_score_naive();

        self.coloring[vertex as usize] = starting_color;
        self.number_of_colors -= 1;

        match score < best_score {
            true => Some((score, Color(self.number_of_colors))),
            false => None
        }

    }

    pub fn graph_coloring(&mut self) {
        let mut best_score = self.calculate_score_naive();
        
        for _ in 0..NUMBER_OF_ITERATIONS {
            let u = rand::random_range(0, self.graph.get_number_of_nodes() as i32);

            let swap_color_best_result = self.try_swap_color_operation(u as i32, best_score);
            let new_color_best_result = self.try_new_color_operation(u as i32, best_score);

            match (swap_color_best_result, new_color_best_result) {
                (Some(swap_color_res), Some(new_color_res)) => {
                    let swap_color_score = swap_color_res.0;
                    let new_color_score = new_color_res.0;

                    if swap_color_score <= new_color_score {
                        best_score = swap_color_score;
                        self.coloring[u as usize] = swap_color_res.1;
                    } else {
                        best_score = new_color_score;
                        self.coloring[u as usize] = new_color_res.1;
                        self.number_of_colors += 1;
                    }
                    self.history.push(self.coloring.clone());
                },
                (Some(swap_color_res), None) => {
                    best_score = swap_color_res.0;
                    self.coloring[u as usize] = swap_color_res.1;
                    self.history.push(self.coloring.clone());
                },
                (None, Some(new_color_res)) => {
                    best_score = new_color_res.0;
                    self.coloring[u as usize] = new_color_res.1;
                    self.number_of_colors += 1;
                    self.history.push(self.coloring.clone());
                },
                (None, None) => {}
            }
        }
    }
}