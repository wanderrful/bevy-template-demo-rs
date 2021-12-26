/// Convert Keyboard inputs into Game Action events, which will in turn affect gameplay.
use std::collections::HashMap;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::GameState;
use crate::plugins::actions;
use crate::plugins::console::IsFocusedOnUI;
use crate::plugins::player::Possessed;


/// Represents the Input handler for the Playing GameState.
///     Input -> &str -> Action -> (Implementations)
/// The above flow allows us to decouple gameplay inputs from the actions themselves,
///     so that we can re-bind keys by updating a configuration file.
pub struct InputPlugin;


impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app
            .insert_resource(get_input_bindings())
            .add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(handle_game_input.system())
                .with_system(handle_debug_input.system())
                .with_system(on_update_mouse_movement.system()))
            .add_system_set(SystemSet::on_exit(GAME_STATE)
                .with_system(on_exit.system()));
    }
}


fn on_enter() {
    debug!("on_enter");
}


fn on_exit() {
    debug!("on_exit");
}


/// Game Input Handler
// TODO | _Should_ I replace std::String with an enum? Or can I somehow consolidate this?
fn handle_game_input(
    player: Query<Entity, (With<Possessed>, Without<IsFocusedOnUI>)>,
    keys: Res<Input<KeyCode>>,
    input_bindings: Res<InputBindings>,
    mut move_forward: EventWriter<actions::MoveForward>,
    mut move_strafe: EventWriter<actions::StrafeRight>,
    mut crouch: EventWriter<actions::Crouch>,
    mut jump: EventWriter<actions::Jump>,
) {
    player.for_each(|_| {
        keys.get_just_pressed().for_each(|&it| {
            input_bindings.iter()
                .filter(|(&k, _v)| k == it)
                .map(|(_k, v)| v)
                .for_each(|action| {
                    match action.as_str() {
                        default => {}
                    }
                });
        });

        keys.get_just_released().for_each(|&it| {
            input_bindings.iter()
                .filter(|(&k, _v)| k == it)
                .map(|(_k, v)| v)
                .for_each(|action| {
                    match action.as_str() {
                        default => {}
                    }
                });
        });

        keys.get_pressed().for_each(|&it| {
            input_bindings.iter()
                .filter(|(&k, _v)| k == it)
                .map(|(_k, v)| v)
                .for_each(|action| {
                    match action.as_str() {
                        "MoveForward" => move_forward.send(actions::MoveForward(1.0)),
                        "MoveBackward" => move_forward.send(actions::MoveForward(-1.0)),
                        "StrafeLeft" => move_strafe.send(actions::StrafeRight(-1.0)),
                        "StrafeRight" => move_strafe.send(actions::StrafeRight(1.0)),
                        "Crouch" => crouch.send(actions::Crouch(true)),
                        "Jump" => jump.send(actions::Jump(true)),
                        default => {}
                    }
                });
        });
    });
}

/// Handle key inputs that are independent of InputMode
fn handle_debug_input(
    keys: Res<Input<KeyCode>>,
    input_bindings: Res<InputBindings>,
    mut toggle_console: EventWriter<actions::ToggleConsole>,
    mut spawn_cube_actor: EventWriter<actions::SpawnCubeActor>,
    mut spawn_spectator_camera: EventWriter<actions::SpawnSpectatorCamera>,
) {
    keys.get_just_pressed().for_each(|&it| {
        input_bindings.iter()
            .filter(|(&k, _v)| k == it)
            .map(|(_k, v)| v)
            .for_each(|action| {
                match action.as_str() {
                    "SpawnCubeActor" => spawn_cube_actor.send(actions::SpawnCubeActor),
                    "SpawnSpectatorCamera" => spawn_spectator_camera.send(actions::SpawnSpectatorCamera),
                    "ToggleConsole" => toggle_console.send(actions::ToggleConsole),
                    default => {}
                }
            });
    });
}


/// Map mouse movement to game Actions!
fn on_update_mouse_movement(
    input_bindings: Res<InputBindings>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut look_up: EventWriter<actions::LookUp>,
    mut look_right: EventWriter<actions::LookRight>
) {
    mouse_motion.iter().for_each(|it: &MouseMotion| {
        look_up.send(actions::LookUp(it.delta.y));
        look_right.send(actions::LookRight(it.delta.x));
    });
}


/// Wrapper struct for the game's Input Bindings.
pub type InputBindings = HashMap<KeyCode, String>;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct InputBinding {
    key: KeyCode,
    binding: String
}

fn get_input_bindings() -> InputBindings {
    const INPUT_FILE_LOCATION: &str = "assets/inputs.yaml";
    let error_input_file_not_found: String =
        format!("Input file not found at '{}'!", INPUT_FILE_LOCATION);
    let error_input_file_formatting: String =
        format!("Input file at '{}' is not formatted properly!", INPUT_FILE_LOCATION);

    let input_bindings: Vec<InputBinding> = serde_yaml::from_reader(
        std::io::BufReader::new(std::fs::File::open(INPUT_FILE_LOCATION)
            .expect(error_input_file_not_found.as_str()))
    ).expect(error_input_file_formatting.as_str());

    input_bindings.iter()
        .map(|it| (it.key, String::from(it.binding.as_str())))
        .collect()
}
