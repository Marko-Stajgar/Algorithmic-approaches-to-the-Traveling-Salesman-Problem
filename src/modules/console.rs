use crate::graph;

use nalgebra::DMatrix;
use bevy::prelude::*;

pub fn execute_input(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut edit_mode: ResMut<graph::EditMode>,
    mut vertex_list: ResMut<graph::VertexList>,
    mut edge_list: ResMut<graph::EdgeList>,
    mut shortest_cycle: ResMut<graph::ShortestCycle>,
    mut ant_colony_parameters: ResMut<graph::AntColonyParameters>,
    console_input: &str,
){
    println!("execute command: {:?}", console_input);

    let parts = console_input.split(" ");

    let collection: Vec<&str> = parts.collect();

    for i in 0.. collection.len()
    {
        println!("{}: {}", i,collection[i]);
    }

    if collection[0] == "solve"
    {
        if collection[1] =="with"
        {
            if collection[2] == "ant-colony\r"
            {
                println!("executing command: {:?}", console_input);

                edit_mode.activate = false;

                ant_colony_parameters.pheromone_matrix = DMatrix::from_diagonal_element(0, 0, 0.0);
                ant_colony_parameters.ant_paths = Vec::new();

                ant_colony_parameters.activate = true;
                ant_colony_parameters.pheromone_matrix = DMatrix::from_diagonal_element(vertex_list.count as usize, vertex_list.count as usize, 0.0);
            }
        }
    }

    if collection[0] == "set"
    {
        if collection[1] == "number_of_ants:"
        {
            if collection[2] !="\r"
            {
                ant_colony_parameters.number_of_ants = collection[2].replace("\r", "").parse().unwrap();
                println!("number_of_ants: {}", ant_colony_parameters.number_of_ants);
            }
        }

        if collection[1] == "pheromone_constant:"
        {
            if collection[2] !="\r"
            {
                ant_colony_parameters.pheromone_constant = collection[2].replace("\r", "").parse().unwrap();
                println!("pheromone_constant: {}", ant_colony_parameters.pheromone_constant);
            }
        }

        if collection[1] == "pheromone_evaporation_rate:"
        {
            if collection[2] !="\r"
            {
                ant_colony_parameters.pheromone_evaporation_rate = collection[2].replace("\r", "").parse().unwrap();
                println!("pheromone_evaporation_rate: {}", ant_colony_parameters.pheromone_evaporation_rate);
            }
        }

        if collection[1] == "alpha:"
        {
            if collection[2] !="\r"
            {
                ant_colony_parameters.alpha = collection[2].replace("\r", "").parse().unwrap();
                println!("alpha: {}", ant_colony_parameters.alpha);
            }
        }

        if collection[1] == "beta:"
        {
            if collection[2] !="\r"
            {
                ant_colony_parameters.beta = collection[2].replace("\r", "").parse().unwrap();
                println!("beta: {}", ant_colony_parameters.beta);
            }
        }
    }

    if console_input == "clear\r"
    {
        vertex_list.vector = Vec::new();
        vertex_list.count = 0;

        edge_list.vector = Vec::new();
        edge_list.count = 0;

        shortest_cycle.vector = Vec::new();
        shortest_cycle.total_cycle_weight = 0.0;
    }

    if console_input == "stop\r"
    {
        println!("executing command: {:?}", console_input);

        ant_colony_parameters.activate = false;
        edit_mode.activate = true;
    }

    if console_input == "exit\r"
    {
        app_exit_events.send(bevy::app::AppExit);
    }
}