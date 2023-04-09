use crate::app;
use crate::graph;

use nalgebra::DMatrix;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;

pub fn execute_input(
    vertex_list: Res<graph::VertexList>,
    edge_list: Res<graph::EdgeList>,
    shortest_cycle: ResMut<graph::ShortestCycle>,
    console_input: &str,
){
    println!("execute command: {:?}", console_input);

    if console_input == "test\r"
    {
        println!("executing command: {:?}", console_input);

        let number_of_ants: u32 = 5;
        let pheromone_constant: f32 =   1.0;
        let pheromone_evaporation_rate: f32 = 0.2;

        graph::ant_colony_optimization(
            vertex_list,
            edge_list,
            shortest_cycle,
            number_of_ants,
            pheromone_constant,
            pheromone_evaporation_rate,
        );
    }
}