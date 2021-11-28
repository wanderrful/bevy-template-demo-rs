use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;

/// Represents the Input handler for the Playing GameState.
pub struct MyInputPlugin;

impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(on_tick.system()))
            .add_system_set(SystemSet::on_exit(GAME_STATE)
                .with_system(on_exit.system()));
    }
}

fn on_enter() {
    MyInputPlugin.log_debug("on_enter");
    MyInputPlugin.log_info("on_enter");
    MyInputPlugin.log_warn("on_enter");
    MyInputPlugin.log_error("on_enter");
}

fn on_exit() {
    MyInputPlugin.log_debug("on_exit");
}

fn on_tick(mut input_events: EventReader<KeyboardInput>) {
    input_events.iter().for_each(|key: &KeyboardInput|
        match key.state.is_pressed() {
            true => { on_key_pressed(key) },
            false => { }
        });
}

fn on_key_pressed(key: &KeyboardInput) {
    MyInputPlugin.log_info(format!("keyPressed={:?}", key.key_code.unwrap()).as_str());
}
