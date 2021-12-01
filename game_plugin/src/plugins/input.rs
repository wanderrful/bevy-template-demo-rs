/// Convert Keyboard inputs into Game Action events, which will in turn affect gameplay.
use std::collections::HashMap;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::plugins::actions::{AxisAction, AxisActionType};


/// Represents the Input handler for the Playing GameState.
pub struct InputPlugin;


impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(on_update2.system()))
            .add_system_set(SystemSet::on_exit(GAME_STATE)
                .with_system(on_exit.system()));
    }
}


fn on_enter() {
    InputPlugin.log_debug("on_enter");
}


fn on_exit() {
    InputPlugin.log_debug("on_exit");
}

fn on_update2(
    keys: Res<Input<KeyCode>>,
    mut axis_action: EventWriter<AxisAction>,
) {
    let input_bindings = get_input_bindings();

    for it in keys.get_just_pressed() {
        if input_bindings.contains_key(&it) {
            InputPlugin.log_info(format!("event=justPressed key={:?}", it).as_str());
        }
    }

    for it in keys.get_just_released() {
        if input_bindings.contains_key(&it) {
            InputPlugin.log_info(format!("event=justReleased key={:?}", it).as_str());
        }
    }

    for it in keys.get_pressed() {
        if input_bindings.contains_key(&it) {
            axis_action.send(*input_bindings.get(&it).unwrap());
        }
    }
}

// TODO | Promote this to some kind of Configuration file
fn get_input_bindings() -> HashMap<KeyCode, AxisAction> {
    let mut out = HashMap::new();

    out.insert(KeyCode::W, AxisAction { scale: 1.0, kind: AxisActionType::MOVE_FORWARD });
    out.insert(KeyCode::S, AxisAction { scale: -1.0, kind: AxisActionType::MOVE_FORWARD });
    out.insert(KeyCode::A, AxisAction { scale: -1.0, kind: AxisActionType::MOVE_STRAFE });
    out.insert(KeyCode::D, AxisAction { scale: 1.0, kind: AxisActionType::MOVE_STRAFE });

    out
}
