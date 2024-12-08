use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;

use nannou::prelude::*;

use crate::graph::{Graph, Point};
use crate::tsp::{Tsp, TspPath, ScoreCalcTypeTSP};

struct Model {
    graph: Rc<Graph>,
    path: TspPath,
    history: VecDeque<TspPath>
}

fn draw_vertices(draw: &Draw, boundary: &Rect, model: &Model) {
    let nodes: &Vec<Point> = model.graph.get_nodes();
    let graph_boundary = model.graph.get_boundary();

    for node in nodes {
        let x = map_range(node.x, graph_boundary.2, graph_boundary.3, boundary.left(), boundary.right());
        let y = map_range(node.y, graph_boundary.0, graph_boundary.1, boundary.bottom(), boundary.top());
        draw.ellipse().x_y(x, y).radius(5.0).color(WHITE);
    }
}

fn draw_path(draw: &Draw, boundary: &Rect, path: TspPath, model: &Model) {
    let nodes = model.graph.get_nodes();
    let graph_boundary = model.graph.get_boundary();

    let n = path.len();
    for i in 0..n {
        let u = path[i];
        let v = path[(i + 1) % n];
        let u = &nodes[u as usize];
        let v = &nodes[v as usize];

        let ux = map_range(u.x, graph_boundary.2, graph_boundary.3, boundary.left(), boundary.right());
        let uy = map_range(u.y, graph_boundary.0, graph_boundary.1, boundary.bottom(), boundary.top());
        let vx = map_range(v.x, graph_boundary.2, graph_boundary.3, boundary.left(), boundary.right());
        let vy = map_range(v.y, graph_boundary.0, graph_boundary.1, boundary.bottom(), boundary.top());

        draw.line().start(pt2(ux, uy)).end(pt2(vx, vy)).color(ROYALBLUE);
    }
}

fn model(_app: &App) -> Model {
    let mut tsp_graph = Graph::new();
    tsp_graph.fill_with_random_points(100);
    tsp_graph.fill_with_edges_full();

    let tsp_graph = Rc::new(tsp_graph);
    let mut tsp = Tsp::new(Rc::clone(&tsp_graph), ScoreCalcTypeTSP::Fast);
    let path = tsp.generate_starting_path();
    let length = tsp.tsp().unwrap();

    Model {
        graph: tsp_graph,
        path: path.clone(),
        history: VecDeque::from(tsp.get_history().clone())
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let path = model.history.pop_front();

    if path.is_none() {
        return;
    }

    let path = path.unwrap();
    model.path = path.clone();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    
    let draw = app.draw();
    let boundary = app.window_rect();

    draw_vertices(&draw, &boundary, model);
    draw_path(&draw, &boundary, model.path.clone(), model);

    draw.to_frame(app, &frame).unwrap();
}

pub fn draw() {    
    nannou::app(model)
        .loop_mode(LoopMode::Rate { update_interval: Duration::from_millis(1000) })
        .update(update)
        .simple_window(view)
        .run();
}