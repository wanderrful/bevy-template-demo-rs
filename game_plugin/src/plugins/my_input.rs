use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::plugins::actions::{MoveForward, MoveStrafe};

/// Represents the Input handler for the Playing GameState.
pub struct MyInputPlugin;

impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(on_update.system()))
            .add_system_set(SystemSet::on_exit(GAME_STATE)
                .with_system(on_exit.system()));
    }
}

fn on_enter() {
    MyInputPlugin.log_debug("on_enter");
}

fn on_exit() {
    MyInputPlugin.log_debug("on_exit");
}

fn on_update(mut keyboard_input: EventReader<KeyboardInput>,
             mut move_forward: EventWriter<MoveForward>,
             mut move_strafe: EventWriter<MoveStrafe>) {
    keyboard_input.iter().for_each(|key: &KeyboardInput| {
        let key_code: KeyCode = key.key_code.unwrap();
        let is_pressed: bool = key.state.is_pressed();

        match key_code {
            KeyCode::W => {
                move_forward.send(MoveForward {
                    scale: if is_pressed { 1.0 } else { 0.0 }
                })
            },
            KeyCode::S => {
                move_forward.send(MoveForward {
                    scale: if is_pressed { -1.0 } else { 0.0 }
                })
            },
            KeyCode::A => {
                move_strafe.send(MoveStrafe {
                    scale: if is_pressed { -1.0 } else { 0.0 }
                })
            },
            KeyCode::D => {
                move_strafe.send(MoveStrafe {
                    scale: if is_pressed { 1.0 } else { 0.0 }
                })
            }
            default => {
                MyInputPlugin.log_info(format!(
                    "event={} input={:?}",
                    if is_pressed {"keyPressed"} else {"keyReleased"},
                    key_code
                ).as_str());
            }
        }
    });
}
