use std::fmt::Debug;
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, Cursor},
    input::mouse:: MouseButtonInput,
    input::keyboard::KeyboardInput,
};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;
use std::thread;
use std::time::Duration;

const WIN_HEIGHT: f32 = 1920.;
const WIN_WIDTH: f32 = 1080.;

// Declaration and initialization of the vertex list that stores vertex as tuple in the form of (vertex_number, vertex_position_height, vertex_position_width)
#[derive(Resource)]
struct VertexList
{
    vector: Vec<(u32, f32, f32)>,
    count: u32,
}

// Declaration and initialization of the edge list that stores edge as tuple in the form of (vertex1, vertex2, distance_between_vertices)
#[derive(Resource)]
struct EdgeList
{
    vector: Vec<(u32, u32, f32)>,
    count: u32,
}

fn main()
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.17254902, 0.176470588, 0.176470588)))
        .insert_resource(VertexList {vector: Vec::new(), count: 0})
        .insert_resource(EdgeList {vector: Vec::new(), count: 0})
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
        .add_startup_system(setup)
        .add_system(session_time)
        .add_system(console_input)
        // -----------------------------
        .add_system(graph_handler)
        .run();
}

#[derive(Component)]
struct ConsoleInputText;

#[derive(Component)]
struct ConsolePastCommand1;

#[derive(Component)]
struct ConsolePastCommand2;

#[derive(Component)]
struct ConsolePastCommand3;

#[derive(Component)]
struct VertexCountText;

#[derive(Component)]
struct EdgeCountText;

#[derive(Component)]
struct PossibleCyclesText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

    // Spawns a text bundle representing initial console mark ":$ "
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            ":$ ",
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
                    bottom: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
    ));

    // Spawns a text budnle representing the user console input in realtime
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
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
                    bottom: Val::Px(5.0),
                    left: Val::Px(40.0),
                    ..default()
                },
                ..default()
            }),
        ConsoleInputText,
    ));

    // Spawns a text bundle representing past command 1 from user console input
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
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
                    bottom: Val::Px(40.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        ConsolePastCommand1,
    ));

    // Spawns a text bundle representing past command 2 from user console input
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
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
                    bottom: Val::Px(75.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        ConsolePastCommand2,
    ));

    // Spawns a text bundle representing past command 3 from user console input
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "",
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
                    bottom: Val::Px(110.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        ConsolePastCommand3,
    ));

    // Spawns a text bundle representing number of vertices in the graph
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Number of vertices: ",
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
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        VertexCountText,
    ));

    // Spawns a text bundle representing number of edges in the graph
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Number of edges: ",
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
                    top: Val::Px(50.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        EdgeCountText,
    ));

    // Spawns a text bundle representing number of possible hamiltionian cycles inside the graph
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Number of possible cycles: ",
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
                    top: Val::Px(85.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        PossibleCyclesText,
    ));
}

// This system takes keyboard input and updates the console text on screen accordingly
fn console_input(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut console_past_command3: Local<String>,
    mut console_past_command2: Local<String>,
    mut console_past_command1: Local<String>,
    mut string: Local<String>,
    mut console_input_param_set: ParamSet<(
        Query<&mut Text, With<ConsoleInputText>>,
        Query<&mut Text, With<ConsolePastCommand1>>,
        Query<&mut Text, With<ConsolePastCommand2>>,
        Query<&mut Text, With<ConsolePastCommand3>>,
    )>)
{
    for ev in char_evr.iter()
    {
        if ev.char == '\x08'
        {
            string.pop();
        }

        else
        {
            string.push(ev.char);
        }

        for mut text in console_input_param_set.p0().iter_mut()
        {
            text.sections[0].value = format!("{}", *string);
        }
    }

    if keys.just_pressed(KeyCode::Return)
    {
        *console_past_command3 = console_past_command2.to_string();
        *console_past_command2 = console_past_command1.to_string();
        *console_past_command1 = string.to_string();

        /*
        println!("Text input: {}", *string);
        println!("Console past command1: {}", *console_past_command1);
        println!("Console past command2: {}", *console_past_command2);
        println!("Console past command3: {}", *console_past_command3);
        */

        string.clear();

        for mut console_input_text in &mut console_input_param_set.p0().iter_mut()
        {
            console_input_text.sections[0].value = format!("{}", *string);
        }

        for mut console_past_command1_text in &mut console_input_param_set.p1().iter_mut()
        {
            console_past_command1_text.sections[0].value = format!("{}", *console_past_command1);
        }

        for mut console_past_command2_text in &mut console_input_param_set.p2().iter_mut()
        {
            console_past_command2_text.sections[0].value = format!("{}", *console_past_command2);
        }

        for mut console_past_command3_text in &mut console_input_param_set.p3().iter_mut()
        {
            console_past_command3_text.sections[0].value = format!("{}", *console_past_command3)
        }
    }
}

// This system prints time elapsed since execution in seconds as app title
fn session_time(mut windows: Query<&mut Window>, time: Res<Time>)
{
    let mut window = windows.single_mut();
    window.title = format!("Computation Engine v1.0 - Session time {:?}", time.elapsed().as_secs_f32().round());
}

// This function returns the cursor position inside the window
fn get_cursor_position(win: &Window) -> Vec2
{
    return win.physical_cursor_position().unwrap();
}

// ------------------------------------

fn graph_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut lines: ResMut<DebugLines>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&mut Window>,
    mut vertex_list: ResMut<VertexList>,
    mut edge_list: ResMut<EdgeList>,
    mut info_text_param_set: ParamSet<(
        Query <&mut Text, With<VertexCountText>>,
        Query <&mut Text, With<EdgeCountText>>,
        Query <&mut Text, With<PossibleCyclesText>>,
    )>)
{
    let win = window.single();
    let count: u32;

    if mouse_button_input.just_pressed(MouseButton::Left)
    {
        vertex_list.count += 1;
        count = vertex_list.count;
        vertex_list.vector.push((count, get_cursor_position(win).y - (win.height() / 2.), get_cursor_position(win).x - (win.width() / 2.)));
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
                edge_list.vector.push((count, i, 0.));
                println!("new edge: {:?}", edge_list.vector[(edge_list.vector.len() - 1) as usize]);
            }
        }

        for mut edge_count_text in &mut info_text_param_set.p1().iter_mut()
        {
            edge_count_text.sections[0].value = format!("Number of edges: {}", edge_list.count.to_string())
        }

        draw_graph(commands, meshes, materials, lines, asset_server, window, vertex_list, edge_list);
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
)
{
    let win = window.single();

    let mut x1: f32;
    let mut y1: f32;

    let mut x2: f32;
    let mut y2: f32;

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
