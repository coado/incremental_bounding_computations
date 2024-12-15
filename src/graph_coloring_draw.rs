use std::collections::VecDeque;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use nannou::prelude::*;

use crate::graph::{Graph, Point};
use crate::graph_coloring::{Color, GraphColoring, ScoreCalcTypeGraphColoring};
use crate::graph_coloring_comp::GraphColouringFlags;

struct Model {
    graph: Rc<Graph>,
    coloring: Vec<Color>,
    history: VecDeque<Vec<Color>>,
}

fn draw_vertices(draw: &Draw, boundary: &Rect, model: &Model) {
    let nodes: &Vec<Point> = model.graph.get_nodes();
    let graph_boundary = model.graph.get_boundary();

    for (i, node) in nodes.iter().enumerate() {
        let x = map_range(node.x, graph_boundary.2, graph_boundary.3, boundary.left(), boundary.right());
        let y = map_range(node.y, graph_boundary.0, graph_boundary.1, boundary.bottom(), boundary.top());
        draw.ellipse().x_y(x, y).radius(12.0).color(WHITE);
        let color = model.coloring[i].0;
        draw.text(&format!("{}", color))
            .x_y(x, y + 1.0)
            .color(BLACK);
    }
}

fn draw_edges(draw: &Draw, boundary: &Rect, model: &Model) {
    let nodes = model.graph.get_nodes();
    let graph_boundary = model.graph.get_boundary();

    for (i, u) in nodes.iter().enumerate() {
        for v in model.graph.get_adjacent_nodes(i as i32) {
            if v < i as i32 {
                continue;
            }

            let line_color = if model.coloring[i] == model.coloring[v as usize] {
                rgba(1.0, 0.0, 0.0, 0.5)
            } else {
                rgba(1.0, 1.0, 1.0, 0.15)
            };

            let v = &nodes[v as usize];
            let ux = map_range(u.x, graph_boundary.2, graph_boundary.3, boundary.left(), boundary.right());
            let uy = map_range(u.y, graph_boundary.0, graph_boundary.1, boundary.bottom(), boundary.top());
            let vx = map_range(v.x, graph_boundary.2, graph_boundary.3, boundary.left(), boundary.right());
            let vy = map_range(v.y, graph_boundary.0, graph_boundary.1, boundary.bottom(), boundary.top());

            draw.line()
                .start(pt2(ux, uy))
                .end(pt2(vx, vy))
                .color(line_color);
        }
    }

}

fn model(_app: &App) -> Model {
    let mut graph = Graph::new();
    graph.fill_with_random_points(10);
    graph.fill_with_edges_stochastic(0.35);

    let graph = Rc::new(graph);
    let mut graph_coloring = GraphColoring::new(
        Rc::clone(&graph), 
        ScoreCalcTypeGraphColoring::Incremental, 
        Some(GraphColouringFlags::new(false, false, true))
    );
    let starting_coloring = graph_coloring.coloring.clone();
    graph_coloring.graph_coloring();

    Model {
        graph,
        coloring: starting_coloring,
        history: VecDeque::from(graph_coloring.history.clone()),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let coloring = model.history.pop_front();

    thread::sleep(Duration::from_millis(200));

    if coloring.is_none() {
        return;
    }

    model.coloring = coloring.unwrap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let boundary = app.window_rect();

    draw_vertices(&draw, &boundary, model);
    draw_edges(&draw, &boundary, model);

    draw.to_frame(app, &frame).unwrap();
}

pub fn draw() {    
    nannou::app(model)
        .loop_mode(LoopMode::Rate { update_interval: Duration::from_millis(1000) })
        .update(update)
        .simple_window(view)
        .run();
}