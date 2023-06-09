use crate::app;
use rand_distr::{Distribution, WeightedIndex};
use nalgebra::DMatrix;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;

#[derive(Resource)]
pub struct EditMode{
    pub activate: bool,
}

// Declaration of the vertex list that stores vertex as tuple in the form of (vertex_number, vertex_position_height, vertex_position_width)
#[derive(Resource)]
pub struct VertexList {
    pub vector: Vec<(u32, f32, f32)>,
    pub count: u32,
}

// Declaration of the edge list that stores edge as tuple in the form of (vertex1, vertex2, distance_between_vertices)
#[derive(Resource)]
pub struct EdgeList {
    pub vector: Vec<(u32, u32, f32)>,
    pub count: u32,
}

// Declaration of the adjacency matrix resource that is used to store a given graph as a square matrix
#[derive(Resource)]
pub struct AdjacencyMatrix{
    pub matrix: DMatrix<f32>,
}

#[derive(Resource)]
pub struct ShortestCycle {
    pub vector: Vec<(u32, u32)>,
    pub total_cycle_weight: f32,
}


// Waits for user input and stores it as a graph accordingly
pub fn graph_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&mut Window>,
    edit_mode: Res<EditMode>,
    mut adjacency_matrix: ResMut<AdjacencyMatrix>,
    mut vertex_list: ResMut<VertexList>,
    mut edge_list: ResMut<EdgeList>,
    mut info_text_param_set: ParamSet<(
        Query<&mut Text, With<app::VertexCountText>>,
        Query<&mut Text, With<app::EdgeCountText>>,
        Query<&mut Text, With<app::PossibleCyclesText>>,
    )>,
) {
    if edit_mode.activate == true
    {
        let win = window.single();
        let count: u32;
        let mut possible_cycles_count: u128 = 1;

        let mut x1: f32;
        let mut y1: f32;

        let mut x2: f32;
        let mut y2: f32;

        let mut distance_x: f32;
        let mut distance_y: f32;

        let mut temp: f32;

        if mouse_button_input.just_pressed(MouseButton::Left) {
            vertex_list.count += 1;
            count = vertex_list.count;
            vertex_list.vector.push((
                count,
                app::get_cursor_position(win).y - (win.height() / 2.),
                app::get_cursor_position(win).x - (win.width() / 2.),
            ));
            println!(
                "new vertex number: {:?}",
                vertex_list.vector[(count - 1) as usize].0
            );

            for mut vertex_count_text in &mut info_text_param_set.p0().iter_mut() {
                vertex_count_text.sections[0].value =
                    format!("Number of vertices: {}", vertex_list.count.to_string());
            }

            edge_list.count += count - 1;

            if count > 1 {
                for i in 1..count {
                    x1 = vertex_list.vector[(count - 1) as usize].2;
                    y1 = vertex_list.vector[(count - 1) as usize].1;

                    x2 = vertex_list.vector[(i - 1) as usize].2;
                    y2 = vertex_list.vector[(i - 1) as usize].1;

                    if x2 < x1 {
                        temp = x1;
                        x1 = x2;
                        x2 = temp;

                        temp = y1;
                        y1 = y2;
                        y2 = temp;
                    }

                    distance_x = (x2 - x1).abs();
                    distance_y = (y2 - y1).abs();

                    edge_list.vector.push((
                        count,
                        i,
                        (distance_x.powf(2.) + distance_y.powf(2.)).sqrt(),
                    ));
                    println!(
                        "new edge: {:?}",
                        edge_list.vector[(edge_list.vector.len() - 1) as usize]
                    );
                }
            }

            adjacency_matrix.matrix = DMatrix::from_diagonal_element(count as usize, count as usize, 0.0);

            for i in 0..edge_list.count
            {
                adjacency_matrix.matrix[(edge_list.vector[i as usize].1 as usize - 1, edge_list.vector[i as usize].0 as usize - 1)] = edge_list.vector[i as usize].2;
                adjacency_matrix.matrix[(edge_list.vector[i as usize].0 as usize - 1 ,edge_list.vector[i as usize].1 as usize - 1)] = edge_list.vector[i as usize].2;
            }

            for i in 1..count{
                possible_cycles_count *= (count - i) as u128;
            }

            possible_cycles_count /= 2;

            for mut edge_count_text in &mut info_text_param_set.p1().iter_mut() {
                edge_count_text.sections[0].value =
                    format!("Number of edges: {}", edge_list.count.to_string())
            }

            for mut possible_cycles_text in &mut info_text_param_set.p2().iter_mut()
            {
                possible_cycles_text.sections[0].value = format!("Number of possible cycles: {}", possible_cycles_count);
            }
        }
    }
}

#[derive(Component)]
struct Vertex;

#[derive(Component)]
struct VertexNumber;

#[derive(Component)]
struct Edge;

// This function draws the graph on the canvas on every new frame
pub fn draw_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lines: ResMut<DebugLines>,
    asset_server: Res<AssetServer>,
    window: Query<&mut Window>,
    vertex_list: ResMut<VertexList>,
    edge_list: ResMut<EdgeList>,
    shortest_cycle: ResMut<ShortestCycle>,
) {
    let mut x1: f32;
    let mut y1: f32;

    let mut x2: f32;
    let mut y2: f32;

    let win = window.single();

    commands.despawn_all::<With<Vertex>>();
    commands.despawn_all::<With<VertexNumber>>();

    for i in 0..edge_list.count
    {
        x1 = vertex_list.vector[(edge_list.vector[(i as usize)].0 - 1) as usize].2;
        y1 = vertex_list.vector[(edge_list.vector[(i as usize)].0 - 1) as usize].1;

        x2 = vertex_list.vector[(edge_list.vector[(i as usize)].1 - 1) as usize].2;
        y2 = vertex_list.vector[(edge_list.vector[(i as usize)].1 - 1) as usize].1;

        lines.line_colored(
            Vec3::new(x1, y1, 0.),
            Vec3::new(x2, y2, 0.),
            0.0,
            Color::BLACK,
        );
    }

    for i in 0..shortest_cycle.vector.len()
    {
        x1 = vertex_list.vector[(shortest_cycle.vector[(i as usize)].0 - 1) as usize].2;
        y1 = vertex_list.vector[(shortest_cycle.vector[(i as usize)].0 - 1) as usize].1;

        x2 = vertex_list.vector[(shortest_cycle.vector[(i as usize)].1 - 1) as usize].2;
        y2 = vertex_list.vector[(shortest_cycle.vector[(i as usize)].1 - 1) as usize].1;

        lines.line_colored(
            Vec3::new(x1, y1, 0.),
            Vec3::new(x2, y2, 0.),
            0.0,
            Color::WHITE,
        );
    }

    for i in 0..vertex_list.count {
        commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(15.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    vertex_list.vector[(i as usize)].2,
                    vertex_list.vector[(i as usize)].1,
                    0.,
                )),
                ..default()
            })
            .insert(Vertex);

        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                vertex_list.vector[i as usize].0.to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-ExtraLight.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::Left)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(vertex_list.vector[(i as usize)].1 + (win.height() / 2.) - 50.),
                        left: Val::Px(vertex_list.vector[(i as usize)].2 + (win.width() / 2.) - 5.),
                        ..default()
                    },
                    ..default()
                }),
            VertexNumber,
        ));
    }
}

// This resource stores the parameters used in the ant-colony-system equations for updating pheromones and probability calculation
#[derive(Resource)]
pub struct AntColonyParameters{
    pub activate: bool,
    pub number_of_ants: u32,
    pub pheromone_constant: f32,
    pub pheromone_evaporation_rate: f32,
    pub alpha: f32,
    pub beta: f32,
    pub pheromone_matrix: DMatrix<f32>,
    pub ant_paths: Vec<(DMatrix<f32>, f32)>,
}

// This system draws the shortest path using the Ant-Colony Optimization algorithm
pub fn ant_colony_optimization(
    mut ant_colony_parameters: ResMut<AntColonyParameters>,
    vertex_list: Res<VertexList>,
    adjacency_matrix: Res<AdjacencyMatrix>,
    mut shortest_cycle: ResMut<ShortestCycle>,
){
    if ant_colony_parameters.activate == true
    {
        let count = vertex_list.count;

        let mut ant_paths = ant_colony_parameters.ant_paths.clone();
        let pheromone_matrix = ant_colony_parameters.pheromone_matrix.clone();
        
        ant_colony_parameters.ant_paths = release_ants(
            ant_colony_parameters.alpha,
            ant_colony_parameters.beta,
            &adjacency_matrix.matrix,
            &pheromone_matrix,
            ant_paths,
            &count,
            &ant_colony_parameters.number_of_ants,
        );

        ant_paths = ant_colony_parameters.ant_paths.clone();

        ant_colony_parameters.pheromone_matrix = update_pheromones(
            pheromone_matrix,
            &ant_paths,
            &ant_colony_parameters.pheromone_constant,
            &ant_colony_parameters.pheromone_evaporation_rate,
            &count,
        );

        shortest_cycle.vector = ant_paths_to_shortest_cycle(
            &ant_paths,
            &count,
        )
    }
}


// This function simulates ants walking through each vertex until a hamiltonian cycle is complete
fn release_ants(
    alpha: f32,
    beta: f32,
    adjacency_matrix: &DMatrix<f32>,
    pheromone_matrix: &DMatrix<f32>,
    mut ant_paths: Vec<(DMatrix<f32>, f32)>,
    vertex_count: &u32,
    number_of_ants: &u32,
) -> Vec<(DMatrix<f32>, f32)> {
    let mut rng = rand::thread_rng();
    let mut current_vertex: u32;
    let mut previous_vertex: u32;
    let mut ant_tour_length: f32;
    let mut c: u32;
    let mut distribution;
    let mut probability_sum: f32;

    for _i in 0..*number_of_ants
    {
        let mut unvisited_vertices: Vec<u32> = Vec::new();
        let mut ant_path = DMatrix::from_diagonal_element(*vertex_count as usize, *vertex_count as usize, 0.0);

        current_vertex = 1;
        ant_tour_length = 0.0;

        for j in 1..*vertex_count
        {
            unvisited_vertices.push(j + 1);
            // println!("unvisited_vertices[{}] = {}", j - 1, unvisited_vertices[j as usize - 1]);
        }

        // println!("----------------------------------------");

        while unvisited_vertices.len() > 0
        {
            let mut probability_list: Vec<f64> = Vec::new();
            probability_sum = 0.0;

            for j in 0..unvisited_vertices.len()
            {
                /*println!("Information used in probability calculation: ");
                println!("Amount of pheromones laid on edge {} - {} stored in the pheromone matrix: {}", unvisited_vertices[j], current_vertex, pheromone_matrix[(unvisited_vertices[j] as usize - 1, current_vertex as usize - 1)]);
                println!("Weight assigned to edge {} - {} stored in the adjacency matrix: {}", unvisited_vertices[j], current_vertex, adjacency_matrix[(unvisited_vertices[j] as usize - 1, current_vertex as usize - 1)]);*/

                probability_sum += f32::powf(pheromone_matrix[(unvisited_vertices[j] as usize - 1, current_vertex as usize - 1)],alpha) * (f32::powf(1.0 / adjacency_matrix[(unvisited_vertices[j] as usize - 1, current_vertex as usize - 1)],beta));
            }

            if probability_sum == 0.0 {
                let equal_prob = 1.0 / unvisited_vertices.len() as f64;

                for _j in 0..unvisited_vertices.len() {
                    probability_list.push(equal_prob);
                    // println!("Propability for vertex {}: {}", unvisited_vertices[j], equal_prob);
                }
            } else {
                for j in 0..unvisited_vertices.len() {
                    let prob: f64 = ((f32::powf(pheromone_matrix[(unvisited_vertices[j] as usize - 1, current_vertex as usize - 1)],alpha) * (f32::powf(1.0 / adjacency_matrix[(unvisited_vertices[j] as usize - 1, current_vertex as usize - 1)],beta))) / probability_sum) as f64;

                    probability_list.push(prob);
                    // println!("Probability for vertex {}: {}", unvisited_vertices[j], prob);
                }
            }


            distribution = WeightedIndex::new(&probability_list).unwrap();
            c = distribution.sample(&mut rng) as u32;


            previous_vertex = current_vertex;
            current_vertex = unvisited_vertices[c as usize];

            unvisited_vertices.remove(c as usize);

            ant_tour_length += adjacency_matrix[(previous_vertex as usize - 1, current_vertex as usize - 1)];

            ant_path[(previous_vertex as usize - 1, current_vertex as usize - 1)] = 1.0;
            ant_path[(current_vertex as usize - 1 ,previous_vertex as usize - 1)] = 1.0;

            /*for j in 0..unvisited_vertices.len()
            {
                println!("unvisited_vertices[{}] = {}", j, unvisited_vertices[j as usize]);
            }

            println!("====================================================");*/
        }

        ant_tour_length += adjacency_matrix[(current_vertex as usize - 1, 0)];

        ant_path[(current_vertex as usize - 1, 0)] += 1.0;
        ant_path[(0, current_vertex as usize - 1)] += 1.0;

        /*for j in 0..*vertex_count
        {
            for x in 0..*vertex_count
            {
                print!("{} ,", ant_path[(x as usize, j as usize)]);
            }
            println!();
        }*/

        ant_paths.push((ant_path, ant_tour_length));

        // println!("Ant tour length: {}" , ant_tour_length);
    }

    return ant_paths;
}

// This function updates the pheromone amount laid on each edge based on the paths that the ants took
fn update_pheromones(
    mut pheromone_matrix: DMatrix<f32>,
    ant_paths: &Vec<(DMatrix<f32>, f32)>,
    pheromone_constant: &f32,
    pheromone_evaporation_rate: &f32,
    vertex_count: &u32,
) -> DMatrix<f32> {

    for i in 0..ant_paths.len()
    {
        let mut ant_pheromone_path = DMatrix::from_diagonal_element(*vertex_count as usize, *vertex_count as usize, 0.0);

        for j in 0..*vertex_count
        {
            for x in 0..*vertex_count
            {
                if ant_paths[i].0[(x as usize, j as usize)] > 0.0
                {
                    ant_pheromone_path[(x as usize, j as usize)] = pheromone_constant / ant_paths[i].1;
                }
            }
        }

        pheromone_matrix = pheromone_matrix*(1.0 - pheromone_evaporation_rate) + ant_pheromone_path;
    }

    /*for j in 0..*vertex_count
    {
        for x in 0..*vertex_count
        {
            print!("{} ,", pheromone_matrix[(x as usize, j as usize)]);
        }
        println!();
    }

    println!("-------------------------------------------------------------------");*/

    return pheromone_matrix;
}


// Converts ant_paths into shortest_cycle
fn ant_paths_to_shortest_cycle(
    ant_paths: &Vec<(DMatrix<f32>, f32)>,
    count: &u32,
) -> Vec<(u32, u32)> {
    let mut shortest_cycle = Vec::new();
    for j in 0..*count {
        for i in 0..*count {
            if ant_paths[ant_paths.len() - 1].0[(i as usize, j as usize)] > 0.0 {
                shortest_cycle.push((i as u32 + 1, j as u32 + 1));
            }
        }
    }
    remove_duplicates(shortest_cycle)
}

// Removes duplicates from a vector of tuples
fn remove_duplicates(
    vector: Vec<(u32, u32)>,
) -> Vec<(u32, u32)> {
    let mut clean_vector = Vec::new();
    for elem in vector {
        if !clean_vector.contains(&elem) {
            clean_vector.push(elem);
        }
    }
    return clean_vector;
}