use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
    input::mouse:: MouseButtonInput,
    input::keyboard::KeyboardInput
};

fn main()
{
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.17254902, 0.176470588, 0.176470588)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Computation Engine v1.0".into(),
                resolution: (1536., 864.).into(),
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
        .add_system(text_input)
        .add_system(add_vertex_to_graph)
        .run();
}

#[derive(Component)]
struct ConsoleTextInput;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2dBundle::default());

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
        ConsoleTextInput,
    ));
}


// This system prints time elapsed since execution in seconds as app title
fn session_time(mut windows: Query<&mut Window>, time: Res<Time>)
{
    let mut window = windows.single_mut();
    window.title = format!("Computation Engine v1.0 - Session time {:?}", time.elapsed().as_secs_f32().round());
}


// This system takes keyboard input and updates the console text on screen acordingly
fn text_input(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut query: Query<&mut Text, With<ConsoleTextInput>>)
{
    for ev in char_evr.iter()
    {
        println!("Got char: '{}'", ev.char);

        if ev.char == '\x08'
        {
            string.pop();
        }

        else
        {
            string.push(ev.char);
        }

        for mut text in &mut query
        {
            text.sections[0].value = format!("{}", *string)
        }
    }

    if keys.just_pressed(KeyCode::Return)
    {
        println!("Text input: {}", *string);
        string.clear();

        for mut text in &mut query
        {
            text.sections[0].value = format!("{}", *string)
        }
    }
}

fn add_vertex_to_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>)
{
    /*for event in cursor_moved_events.iter()
    {
        info!("{:?}", event);
    }*/

    if mouse_button_input.pressed(MouseButton::Left)
    {
        draw_circle(commands, meshes, materials);
    }
}

// This function draws a circle on the app canvas
fn draw_circle(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(10.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}