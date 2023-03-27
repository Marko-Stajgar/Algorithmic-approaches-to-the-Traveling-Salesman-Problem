#[path = "modules/graph.rs"] mod graph;
#[path = "modules/app.rs"] mod app;

use std::fmt::Debug;
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode},
    input::keyboard::KeyboardInput,
};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;

const WIN_HEIGHT: f32 = 1920.;
const WIN_WIDTH: f32 = 1080.;

fn main()
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.17254902, 0.176470588, 0.176470588)))
        .insert_resource(graph::VertexList {vector: Vec::new(), count: 0})
        .insert_resource(graph::EdgeList {vector: Vec::new(), count: 0})
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Computation Engine v1.0".into(),
                resolution: (WIN_HEIGHT, WIN_WIDTH).into(),
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
        .run();
}