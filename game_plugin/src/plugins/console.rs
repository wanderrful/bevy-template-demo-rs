use bevy::prelude::*;

use crate::GameState;
use crate::plugins::actions;
use crate::plugins::input::InputBindings;


/// Labels to uniquely identify the Console Window
///     (e.g. `With<ConsoleWindow>`) and its child widgets
struct ConsoleWindow;
struct ConsoleHistoryBox;
struct ConsoleTextInputBox;
struct ConsoleTextInput;


pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(on_update.system())
                .with_system(process_console_command.system()));
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


fn on_update(
    // Console visibility toggling
    mut toggle_console: EventReader<actions::ToggleConsole>,
    mut console_window: Query<&mut Visible, With<ConsoleWindow>>,

    // Text Input handling
    mut console_text_input: Query<&mut Text, With<ConsoleTextInput>>,
    keys: Res<Input<KeyCode>>,
    input_bindings: Res<InputBindings>,
    mut process_console_command: EventWriter<actions::ProcessConsoleCommand>,
) {
    // Toggle visibility if the button was pressed
    toggle_console.iter().for_each(|it| {
        console_window.iter_mut().for_each(|mut window| {
            window.is_visible = !window.is_visible;
        })
    });

    // TODO | Populate Console text content appropriately

    // TODO | Add "get key for this command" method into the Inputbindings struct(?)
    // TODO | Promote Action Binding names to enum!
    let (console_key, _): (&KeyCode, &String) = input_bindings.iter()
        .filter(|(&_k, v)| v.as_str() == "ToggleConsole")
        .next()
        .unwrap();

    keys.get_just_pressed().for_each(|it: &KeyCode| {
        if it == console_key { return; }

        console_text_input.iter_mut().for_each(|mut text_input| {
            // Append key to text input
            let existing_value =
                text_input.sections.get_mut(0).unwrap().value.to_owned();

            let mut new_value: String = "".to_string();

            // Queue the contents for processing, if it's the Enter button
            let should_process_command: bool = !existing_value.is_empty() && *it == KeyCode::Return;
            if should_process_command {
                process_console_command.send(
                    actions::ProcessConsoleCommand(existing_value));
            } else {
                new_value = existing_value + format!("{:?}", it).as_str();
            }

            text_input.sections.get_mut(0).unwrap().value = new_value;
        });
    });

    // TODO | Execute Console command if one has been entered!
}

fn process_console_command(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut process_console_command: EventReader<actions::ProcessConsoleCommand>,
    console_history_box: Query<Entity, With<ConsoleHistoryBox>>,
) {
    const FONT_ASSET_PATH: &str = "fonts/FiraSans-Bold.ttf";
    let font: Handle<Font> = asset_server.load(FONT_ASSET_PATH);

    let entity: Entity = console_history_box.iter().next().unwrap();

    process_console_command.iter().for_each(|it: &actions::ProcessConsoleCommand| {
        // Log the command to the Console History widget
        // TODO | Something is wrong with the Query?
        console_history_box.iter().for_each(|entity: Entity| {
            commands.entity(entity)
                .with_children(|parent| {
                    parent.spawn_bundle(create_log_line(&font, it.0.to_owned()));
                });
        });

        // TODO | Process the command
        info!("Processing! command={}", it.0);
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
