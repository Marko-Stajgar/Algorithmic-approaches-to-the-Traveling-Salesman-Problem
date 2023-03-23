use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, Cursor},
    input::mouse:: MouseButtonInput,
    input::keyboard::KeyboardInput,
};
use std::thread;
use std::time::Duration;

const WIN_HEIGHT: f32 = 1536.;
const WIN_WIDTH: f32 = 864.;

fn main()
{

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.17254902, 0.176470588, 0.176470588)))
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
        .add_startup_system(setup)
        .add_system(session_time)
        .add_system(console_input)
        // -----------------------------
        .add_system(add_vertex_to_graph)
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

fn add_vertex_to_graph(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&mut Window>,
)
{
    let win = window.single();

    if mouse_button_input.just_pressed(MouseButton::Left)
    {
        draw_vertex(commands, meshes, materials, win.height(), win.width() ,get_cursor_position(win));
    }
}

// This function draws a circle on the app canvas
fn draw_vertex(mut commands: Commands,
               mut meshes: ResMut<Assets<Mesh>>,
               mut materials: ResMut<Assets<ColorMaterial>>,
               win_height: f32,
               win_width: f32,
               position: Vec2,
)
{
    commands.spawn(MaterialMesh2dBundle
    {
        mesh: meshes.add(shape::Circle::new(15.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(position.x - (win_width / 2.), position.y - (win_height / 2.), 0.)),
        ..default()
    });
}