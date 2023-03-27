use crate::graph;

use std::fmt::Debug;
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::{PresentMode},
    input::keyboard::KeyboardInput,
};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_debug_lines::*;

#[derive(Component)]
pub struct ConsoleInputText;

#[derive(Component)]
pub struct ConsolePastCommand1;

#[derive(Component)]
pub struct ConsolePastCommand2;

#[derive(Component)]
pub struct ConsolePastCommand3;

#[derive(Component)]
pub struct VertexCountText;

#[derive(Component)]
pub struct EdgeCountText;

#[derive(Component)]
pub struct PossibleCyclesText;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
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
pub fn console_input(
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
    )>
)
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
pub fn session_time(mut windows: Query<&mut Window>, time: Res<Time>)
{
    let mut window = windows.single_mut();
    window.title = format!("Computation Engine v1.0 - Session time {:?}", time.elapsed().as_secs_f32().round());
}

// This function returns the cursor position inside the window
pub fn get_cursor_position(win: &Window) -> Vec2
{
    return win.physical_cursor_position().unwrap();
}