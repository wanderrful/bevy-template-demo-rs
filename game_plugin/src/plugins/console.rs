use std::collections::HashMap;

use bevy::prelude::*;

use crate::GameState;
use crate::plugins::actions;
use crate::plugins::input::InputBindings;
use crate::plugins::player::Possessed;
use crate::utils::keys::get_adjusted_user_input;


/// Events
pub struct LogToConsole(pub String);
pub struct HandleConsoleCommand(pub String);
pub struct RenderConsoleCommand(pub String);


/// Labels to uniquely identify the Console Window
///     (e.g. `With<ConsoleWindow>`) and its child widgets
struct ConsoleWindow;
struct ConsoleHistoryBox;
struct ConsoleHistoryLogLine;
struct ConsoleTextInputBox;
struct ConsoleTextInput;


/// Label for when the player is focused on the Console, so we should consume their inputs
pub struct IsFocusedOnUI;



/// Plugins
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<LogToConsole>()
            .add_event::<RenderConsoleCommand>()
            .add_event::<HandleConsoleCommand>()
            .add_system_set(SystemSet::on_enter(GameState::Playing)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(handle_toggle_console.system())
                .with_system(handle_key_inputs.system())
                .with_system(log_to_console.system())
                .with_system(handle_console_command.system()));
    }
}



/// Systems

fn on_enter(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    const FONT_ASSET_PATH: &str = "fonts/FiraSans-Bold.ttf";
    let font: Handle<Font> = asset_server.load(FONT_ASSET_PATH);

    // Init Materials
    let transparent: Handle<ColorMaterial> = materials.add(Color::NONE.into());
    let dark_gray: Handle<ColorMaterial> = materials.add(Color::DARK_GRAY.into());
    let gray: Handle<ColorMaterial> = materials.add(Color::GRAY.into());

    // Create Console window
    commands
        .spawn_bundle(create_console_window(transparent))
            .insert(ConsoleWindow)

            // Create User Input widget
            .with_children(|parent| {
                parent.spawn_bundle(create_text_input(dark_gray))
                    .insert(ConsoleWindow)
                    .insert(ConsoleTextInputBox)
                    .with_children(|parent| {
                        parent.spawn_bundle(create_text_input_box(font))
                            .insert(ConsoleWindow)
                            .insert(ConsoleTextInputBox)
                            .insert(ConsoleTextInput);
                    });
            })

            // Create Log History widget
            .with_children(|parent| {
                parent.spawn_bundle(create_log_panel(gray))
                    .insert(ConsoleWindow)
                    .insert(ConsoleHistoryBox);
            });
}


/// Toggle Console UI visibility
fn handle_toggle_console(
    mut toggle_console: EventReader<actions::ToggleConsole>,
    mut console_window: Query<&mut Visible, With<ConsoleWindow>>,
    mut commands: Commands,
    mut player: Query<Entity, With<Possessed>>
) {
    // Toggle visibility for the Console UI
    toggle_console.iter().for_each(|_| {
        let mut is_visible = false;

        console_window.iter_mut().for_each(|mut window| {
            is_visible = !window.is_visible;
            window.is_visible = is_visible;
        });

        // Block/Unblock Input consumption for the Possessed Pawn
        player.iter_mut().for_each(|entity| {
            if is_visible {
                commands.entity(entity)
                    .insert(IsFocusedOnUI);
            } else {
                commands.entity(entity)
                    .remove::<IsFocusedOnUI>();
            }
        });
    });
}


/// Populate the Console window with user input text
fn handle_key_inputs(
    keys: Res<Input<KeyCode>>,
    input_bindings: Res<InputBindings>,
    mut process_console_command: EventWriter<HandleConsoleCommand>,
    mut console_text_input: Query<&mut Text, With<ConsoleTextInput>>,
    player: Query<Entity, (With<Possessed>, With<IsFocusedOnUI>)>
) {
    player.for_each(|_| {
        let DEFAULT_CONSOLE_KEY: KeyCode = KeyCode::Grave;
        let (console_key, _): (&KeyCode, &String) = input_bindings.iter()
            .filter(|(&_k, v)| v.as_str() == "ToggleConsole")
            .next()
            .unwrap_or((&DEFAULT_CONSOLE_KEY, &"".to_string()));

        keys.get_just_pressed()
            .filter(|&it| it != console_key)
            .for_each(|key: &KeyCode| {
                console_text_input.iter_mut().for_each(|mut text_input| {
                    // Append key to text input
                    let existing_value =
                        text_input.sections.get_mut(0).unwrap().value.to_owned();

                    let mut new_value: String = "".to_string();

                    // Queue the contents for processing, if it's the Enter button
                    let should_process_command: bool = !existing_value.is_empty() && *key == KeyCode::Return;
                    if should_process_command {
                        process_console_command.send(
                            HandleConsoleCommand(existing_value));
                    } else {
                        new_value = get_adjusted_user_input(existing_value, key);
                    }

                    text_input.sections.get_mut(0).unwrap().value = new_value;
                });
            });
    });
}

/// Business Logic side effects of entering a console command
fn handle_console_command(
    mut handle_console_command: EventReader<HandleConsoleCommand>,
    mut log_to_console: EventWriter<LogToConsole>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
    mut spawn_cube_actor: EventWriter<actions::SpawnCubeActor>,
) {
    // TODO | How should I map the string to the Event? Same issue with input::InputBindings
    handle_console_command.iter().for_each(|it| {
        let as_lower: String = String::from(it.0.trim().to_lowercase());

        // Default message
        let mut log_message: String = format!("Unknown command: '{}'", as_lower);

        match as_lower.as_str() {
            "exit" | "quit" => {
                log_message = "Exiting game...".to_string();
                app_exit.send(bevy::app::AppExit);
            },
            "spawncubeactor" => {
                log_message = "Spawning Cube Actor...".to_string();
                spawn_cube_actor.send(actions::SpawnCubeActor);
            }
            _default => {}
        }

        log_to_console.send(LogToConsole(log_message));
    });
}


/// UI side effects of entering a console command
fn log_to_console(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut log_to_console: EventReader<LogToConsole>,
    console_history_box: Query<Entity, With<ConsoleHistoryBox>>,
) {
    const FONT_ASSET_PATH: &str = "fonts/FiraSans-Bold.ttf";
    let font: Handle<Font> = asset_server.load(FONT_ASSET_PATH);

    let entity: Entity = console_history_box.iter().next()
        .expect("ConsoleHistoryBox was not found in the Console UI!");

    log_to_console.iter().for_each(|it: &LogToConsole| {
        console_history_box.iter().for_each(|entity: Entity| {
            commands.entity(entity)
                .with_children(|parent| {
                    parent.spawn_bundle(create_log_line(&font, it.0.to_owned()))
                        .insert(ConsoleWindow)
                        .insert(ConsoleHistoryLogLine);
                });
        });
    });
}



/// Widget Factory methods

fn create_console_window(background_color: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect { top: Val::Px(0.0), left: Val::Px(0.0), ..Default::default() },
            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        material: background_color,
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }
}

fn create_text_input(background_color: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            position: Rect { top: Val::Px(0.0), ..Default::default() },
            size: Size::new(Val::Percent(100.0), Val::Px(25.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: Rect::all(Val::Px(2.0)),
            ..Default::default()
        },
        material: background_color,
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }
}

fn create_text_input_box(font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }
}

fn create_log_panel(background_color: Handle<ColorMaterial>) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            position: Rect { top: Val::Px(0.0), ..Default::default() },
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: Rect::all(Val::Px(4.0)),
            ..Default::default()
        },
        material: background_color,
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }
}

fn create_log_line(font: &Handle<Font>, value: String) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value,
                    style: TextStyle {
                        font: font.as_weak(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        visible: Visible { is_visible: true, ..Default::default() },
        ..Default::default()
    }
}
