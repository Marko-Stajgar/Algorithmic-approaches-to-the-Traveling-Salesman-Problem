use crate::app;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;

// Declaration and initialization of the vertex list that stores vertex as tuple in the form of (vertex_number, vertex_position_height, vertex_position_width)
#[derive(Resource)]
pub struct VertexList
{
    pub vector: Vec<(u32, f32, f32)>,
    pub count: u32,
}

// Declaration and initialization of the edge list that stores edge as tuple in the form of (vertex1, vertex2, distance_between_vertices)
#[derive(Resource)]
pub struct EdgeList
{
    pub vector: Vec<(u32, u32, f32)>,
    pub count: u32,
}

pub fn graph_handler(
    commands: Commands,
    asset_server: Res<AssetServer>,
    lines: ResMut<DebugLines>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&mut Window>,
    mut vertex_list: ResMut<VertexList>,
    mut edge_list: ResMut<EdgeList>,
    mut info_text_param_set: ParamSet<(
        Query <&mut Text, With<app::VertexCountText>>,
        Query <&mut Text, With<app::EdgeCountText>>,
        Query <&mut Text, With<app::PossibleCyclesText>>,
    )>)
{
    let win = window.single();
    let count: u32;

    let mut x1: f32 = 0.;
    let mut y1: f32 = 0.;

    let mut x2: f32 = 0.;
    let mut y2: f32 = 0.;

    let mut distance_x: f32 = 0.;
    let mut distance_y: f32 = 0.;

    let mut temp: f32 = 0.;

    if mouse_button_input.just_pressed(MouseButton::Left)
    {
        vertex_list.count += 1;
        count = vertex_list.count;
        vertex_list.vector.push((count, app::get_cursor_position(win).y - (win.height() / 2.), app::get_cursor_position(win).x - (win.width() / 2.)));
        println!("new vertex number: {:?}", vertex_list.vector[(count - 1) as usize].0);

        for mut vertex_count_text in &mut info_text_param_set.p0().iter_mut()
        {
            vertex_count_text.sections[0].value = format!("Number of vertices: {}", vertex_list.count.to_string());
        }

        edge_list.count += count - 1;

        if count > 1
        {
            for i in 1..count
            {
                x1 = vertex_list.vector[(count - 1) as usize].2;
                y1 = vertex_list.vector[(count - 1) as usize].1;

                x2 = vertex_list.vector[(i - 1) as usize].2;
                y2 = vertex_list.vector[(i - 1) as usize].1;

                if x2 < x1
                {
                    temp = x1;
                    x1 = x2;
                    x2 = temp;

                    temp = y1;
                    y1 = y2;
                    y2 = temp;
                }

                distance_x = (x2 - x1).abs();
                distance_y = (y2 - y1).abs();

                edge_list.vector.push((count, i, (distance_x.powf(2.) + distance_y.powf(2.)).sqrt()));
                println!("new edge: {:?}", edge_list.vector[(edge_list.vector.len() - 1) as usize]);
            }
        }

        for mut edge_count_text in &mut info_text_param_set.p1().iter_mut()
        {
            edge_count_text.sections[0].value = format!("Number of edges: {}", edge_list.count.to_string())
        }

        draw_graph(commands, meshes, materials, lines, asset_server, window, vertex_list, edge_list, x1, y1, x2, y2);
    }
}

#[derive(Component)]
struct Vertex;

#[derive(Component)]
struct VertexNumber;

#[derive(Component)]
struct Edge;

// This function draws the graph on the canvas on every new frame
fn draw_graph(mut commands: Commands,
              mut meshes: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<ColorMaterial>>,
              mut lines: ResMut<DebugLines>,
              asset_server: Res<AssetServer>,
              window: Query<&mut Window>,
              vertex_list: ResMut<VertexList>,
              edge_list: ResMut<EdgeList>,
              mut x1: f32,
              mut y1: f32,
              mut x2: f32,
              mut y2: f32,
)
{
    let win = window.single();


    let duration: f32 = f32::MAX;

    commands.despawn_all::<With<Vertex>>();

    for i in 0..vertex_list.count
    {
        commands.spawn(MaterialMesh2dBundle
        {
            mesh: meshes.add(shape::Circle::new(15.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(Vec3::new(vertex_list.vector[(i as usize)].2, vertex_list.vector[(i as usize)].1, 0.)),
            ..default()
        }).insert(Vertex);

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

    for i in 0..edge_list.count
    {
        x1 = vertex_list.vector[(edge_list.vector[(i as usize)].0 - 1) as usize].2;
        y1 = vertex_list.vector[(edge_list.vector[(i as usize)].0 - 1) as usize].1;

        x2 = vertex_list.vector[(edge_list.vector[(i as usize)].1 - 1) as usize].2;
        y2 = vertex_list.vector[(edge_list.vector[(i as usize)].1 - 1) as usize].1;

        lines.line_colored(
            Vec3::new(x1, y1, 0.),
            Vec3::new(x2, y2, 0.),
            duration,
            Color::BLACK,
        );
    }
}