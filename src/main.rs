#[path = "modules/app.rs"]
mod app;
#[path = "modules/graph.rs"]
mod graph;
#[path = "modules/console.rs"]
mod console;

use bevy::{prelude::*, window::PresentMode};
use bevy_prototype_debug_lines::*;
use nalgebra::DMatrix;

const WIN_WIDTH: f32 = 1920.;
const WIN_HEIGHT: f32 = 1080.;

fn main() {
    App::new()
        .insert_resource(graph::EditMode {
            activate: true,
        })
        .insert_resource(graph::AdjacencyMatrix {
            matrix: DMatrix::from_diagonal_element(0, 0, 0.0),
        })
        .insert_resource(graph::VertexList {
            vector: Vec::new(),
            count: 0,
        })
        .insert_resource(graph::EdgeList {
            vector: Vec::new(),
            count: 0,
        })
        .insert_resource(graph::ShortestCycle {
            vector: Vec::new(),
            total_cycle_weight: 0.0,
        })
        .insert_resource(graph::AntColonyParameters{
            activate: false,
            number_of_ants: 50,
            pheromone_constant: 1.0,
            pheromone_evaporation_rate: 0.2,
            alpha: 1.0,
            beta: 4.0,
            pheromone_matrix: DMatrix::from_diagonal_element(0, 0, 0.0),
            ant_paths: Vec::new(),
        })
        .insert_resource(ClearColor(Color::rgb(0.17254902, 0.176470588, 0.176470588)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Computation Engine v1.0".into(),
                resolution: (WIN_WIDTH,WIN_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(app::setup)
        .add_system(app::session_time)
        .add_system(app::console_input)
        // -----------------------------
        .add_system(graph::graph_handler)
        .add_system(graph::draw_graph)
        .add_system(graph::ant_colony_optimization)
        .run();
}