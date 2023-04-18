use crate::app;
use crate::graph;

use nalgebra::DMatrix;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;

pub fn execute_input(
    mut edit_mode: ResMut<graph::EditMode>,
    vertex_list: Res<graph::VertexList>,
    mut ant_colony_parameters: ResMut<graph::AntColonyParameters>,
    console_input: &str,
){
    println!("execute command: {:?}", console_input);

    if console_input == "compute using ant-colony\r"
    {
        println!("executing command: {:?}", console_input);

        edit_mode.activate = false;

        ant_colony_parameters.activate = true;
        ant_colony_parameters.pheromone_matrix = DMatrix::from_diagonal_element(vertex_list.count as usize, vertex_list.count as usize, 0.0);
    }

    if console_input == "stop\r"
    {
        println!("executing command: {:?}", console_input);

        ant_colony_parameters.activate = false;
        edit_mode.activate = true;
    }
}