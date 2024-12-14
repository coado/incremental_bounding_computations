use std::rc::Rc;
use crate::graph::{Graph, PointId};
use crate::graph_coloring_comp::GraphColoringComp;

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
    score_type: ScoreCalcTypeGraphColoring,
    comp: Option<GraphColoringComp>
}

pub enum ScoreCalcTypeGraphColoring {
    Fast,
    Slow,
    Incremental
}

impl GraphColoring {
    pub fn new(graph: Rc<Graph>, score_type: ScoreCalcTypeGraphColoring) -> GraphColoring {
        let number_of_nodes = graph.get_number_of_nodes() as i32;
        let coloring = (0..number_of_nodes)
            .map(|_| Color(0))
            .collect::<Vec<Color>>();

        let comp = match &score_type {
            ScoreCalcTypeGraphColoring::Incremental => {
                let mut comp = GraphColoringComp::new(Rc::clone(&graph), number_of_nodes as usize);
                comp.create_computation_graph();
                comp.get_result();
                Some(comp)
            },
            _ => None
        };


        GraphColoring {
            graph,
            coloring,
            number_of_colors: 1,
            history: Vec::new(),
            score_type,
            comp
        }
    }

    fn calculate_score_slow(&self) -> i32 {
        let mut score: i32 = 0;

        for color in 0..self.number_of_colors {
            let mut amount: i32 = 0;
            for u in 0..self.graph.get_number_of_nodes() as i32 {
                if self.coloring[u as usize] == Color(color) {
                    amount += 1;
                }
            }

            score -= amount.pow(2);
        }

        for color in 0..self.number_of_colors {
            let mut amount = 0;
            let mut illegal_edges = 0;

            for u in 0..self.graph.get_number_of_nodes() as i32 {
                if self.coloring[u as usize] == Color(color) {
                    amount += 1;
                    for v in self.graph.get_adjacent_nodes(u) {
                        if self.coloring[v as usize] == Color(color) {
                            illegal_edges += 1;
                        }
                    }
                }
            }

            illegal_edges /= 2;
            score += 2 * illegal_edges * amount;
        }

        score
    }

    fn calculate_score_naive(&self) -> i32 {
        let mut score = 0;
        let mut colors_freq = (0..self.graph.get_number_of_nodes()).map(|_| i32::default()).collect::<Vec<i32>>();
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
            let number_of_violating_edges = violating_edges_freq[color as usize] / 2;

            score += number_of_nodes * (2 * number_of_violating_edges - number_of_nodes);
        }

        score
    }

    fn calc_score(&mut self) -> i32 {
        let res = match &self.score_type {
            ScoreCalcTypeGraphColoring::Fast => self.calculate_score_naive(),
            ScoreCalcTypeGraphColoring::Slow => self.calculate_score_slow(),
            ScoreCalcTypeGraphColoring::Incremental => {
                let comp = self.comp.as_mut().unwrap();
                comp.get_result().unwrap()
            }
        };

        res
    }

    fn set_color(&mut self, v: usize, color: Color) {
        
        match &mut self.score_type {
            ScoreCalcTypeGraphColoring::Incremental => {
                let comp = self.comp.as_mut().unwrap();
                comp.update_input_node(v, color.0);
            },
            _ => {}
        }

        self.coloring[v] = color;
    }

    fn try_swap_color_operation(&mut self, vertex: PointId, best_score: i32) -> i32 {
        let mut current_best_score = best_score;
        let starting_color = self.coloring[vertex as usize];
        let mut best_color = starting_color;

        for c in 0..self.number_of_colors {
            if c == starting_color { continue; } 
            self.set_color(vertex as usize, Color(c));
            let score = self.calc_score();

            if score < current_best_score {
                current_best_score = score;
                best_color = Color(c);
            }
        }

        self.set_color(vertex as usize, best_color);
        current_best_score
    }

    fn try_new_color_operation(&mut self, vertex: PointId, best_score: i32) -> i32 {
        if self.number_of_colors == self.graph.get_number_of_nodes() as i32 {
            // There are no more colors to try
            return best_score;
        }

        let starting_color = self.coloring[vertex as usize];
        self.set_color(vertex as usize, Color(self.number_of_colors));
        let score = self.calc_score();
        
        if score < best_score {
            self.number_of_colors += 1;
            return score;
        } else {
            self.set_color(vertex as usize, starting_color);
            return best_score;
        }
    }

    pub fn graph_coloring(&mut self) -> i32 {
        let mut best_score = self.calc_score();
        let mut incremented: bool = true;
        
        while incremented {
            incremented = false;
            for u in 0..self.graph.get_number_of_nodes() as i32 {
                let tmp_best_score = best_score;
                best_score = self.try_swap_color_operation(u as i32, best_score);
                best_score = self.try_new_color_operation(u as i32, best_score);

                if tmp_best_score != best_score {
                    self.history.push(self.coloring.clone());
                    incremented = true;
                };
            }
        }

        best_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Point;

    fn create_testing_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_nodes((0..5).map(|_| Point::random()).collect());
        graph.add_2d_edge(0, 1);
        graph.add_2d_edge(0, 4);
        graph.add_2d_edge(1, 3);
        graph.add_2d_edge(1, 2);
        graph.add_2d_edge(2, 4);
        graph.add_2d_edge(2, 3);
        graph.add_2d_edge(3, 4);
        graph
    }

    #[test]
    fn test_calculate_score_naive() {
        let graph = create_testing_graph();
        let graph_rc = Rc::new(graph);
        let mut graph_coloring = GraphColoring::new(Rc::clone(&graph_rc), ScoreCalcTypeGraphColoring::Fast);
        let score = graph_coloring.calculate_score_naive();
        assert_eq!(score, 45, "Calucalte Score Naive: Score is incorrect, should be 45");

        graph_coloring.coloring[0] = Color(1);
        graph_coloring.number_of_colors += 1;
        let score = graph_coloring.calculate_score_naive();
        assert_eq!(score, 23, "Calucalte Score Naive: Score is incorrect, should be 23");

        graph_coloring.coloring[1] = Color(2);
        graph_coloring.number_of_colors += 1;
        let score = graph_coloring.calculate_score_naive();
        assert_eq!(score, 7, "Calucalte Score Naive: Score is incorrect, should be 7");

        graph_coloring.coloring[2] = Color(1);
        let score = graph_coloring.calculate_score_naive();
        assert_eq!(score, -5, "Calucalte Score Naive: Score is incorrect, should be -5");

        graph_coloring.coloring[4] = Color(2);
        let score = graph_coloring.calculate_score_naive();
        assert_eq!(score, -9, "Calucalte Score Naive: Score is incorrect, should be -9");
    }

    #[test]
    fn test_calculate_score_slow() {
        let graph = create_testing_graph();
        let graph_rc = Rc::new(graph);
        let mut graph_coloring = GraphColoring::new(Rc::clone(&graph_rc), ScoreCalcTypeGraphColoring::Slow);
        let score = graph_coloring.calculate_score_slow();
        assert_eq!(score, 45, "Calucalte Score Naive: Score is incorrect, should be 45");

        graph_coloring.coloring[0] = Color(1);
        graph_coloring.number_of_colors += 1;
        let score = graph_coloring.calculate_score_slow();
        assert_eq!(score, 23, "Calucalte Score Naive: Score is incorrect, should be 23");

        graph_coloring.coloring[1] = Color(2);
        graph_coloring.number_of_colors += 1;
        let score = graph_coloring.calculate_score_slow();
        assert_eq!(score, 7, "Calucalte Score Naive: Score is incorrect, should be 7");

        graph_coloring.coloring[2] = Color(1);
        let score = graph_coloring.calculate_score_slow();
        assert_eq!(score, -5, "Calucalte Score Naive: Score is incorrect, should be -5");

        graph_coloring.coloring[4] = Color(2);
        let score = graph_coloring.calculate_score_slow();
        assert_eq!(score, -9, "Calucalte Score Naive: Score is incorrect, should be -9");
    }

    #[test]
    fn test_graph_coloring_fast() {
        let graph = create_testing_graph();
        let graph_rc = Rc::new(graph);
        let mut graph_coloring = GraphColoring::new(Rc::clone(&graph_rc), ScoreCalcTypeGraphColoring::Fast);
        let score  = graph_coloring.graph_coloring();
        let coloring = graph_coloring.coloring;

        assert_eq!(coloring, vec![Color(1), Color(2), Color(1), Color(3), Color(2)], "Fast: coloring is incorrect");
        assert_eq!(score, -9, "Fast: score is incorrect");
    }

    #[test]
    fn test_graph_coloring_slow() {
        let graph = create_testing_graph();
        let graph_rc = Rc::new(graph);
        let mut graph_coloring = GraphColoring::new(Rc::clone(&graph_rc), ScoreCalcTypeGraphColoring::Slow);
        let score  = graph_coloring.graph_coloring();
        let coloring = graph_coloring.coloring;

        assert_eq!(coloring, vec![Color(1), Color(2), Color(1), Color(3), Color(2)], "Slow: coloring is incorrect");
        assert_eq!(score, -9, "Slow: score is incorrect");
    }

    #[test]
    fn test_graph_coloring_incremental() {
        let graph = create_testing_graph();
        let graph_rc = Rc::new(graph);
        let mut graph_coloring = GraphColoring::new(Rc::clone(&graph_rc), ScoreCalcTypeGraphColoring::Incremental);
        let score  = graph_coloring.graph_coloring();
        let coloring = graph_coloring.coloring;

        assert_eq!(coloring, vec![Color(1), Color(2), Color(1), Color(3), Color(2)], "Incremental: coloring is incorrect");
        assert_eq!(score, -9, "Incremental: score is incorrect");
    }
}