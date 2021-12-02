/// Convert Keyboard inputs into Game Action events, which will in turn affect gameplay.
use std::collections::HashMap;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::plugins::actions::{Action, AxisAction, AxisActionType, ToggleAction, ToggleActionType};


/// Represents the Input handler for the Playing GameState.
pub struct InputPlugin;


impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app
            .insert_resource(InputBindings { bindings: get_input_bindings() })
            .add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(on_update.system()))
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


/// TODO | Unify these three states into one algorithm!
fn on_update(
    keys: Res<Input<KeyCode>>,
    input_bindings: Res<InputBindings>,
    mut action: EventWriter<Action>,
) {
    // Publish Toggle Action press events
    keys.get_just_pressed()
        .for_each(|&it: &KeyCode| input_bindings.bindings.iter()
            .filter(|(&k, &v)| k == it)
            .map(|(k, v): (&KeyCode, &Action)| v)
            .filter(|&v| match v { Action::Toggle(_) => true, default => false })
            .for_each(|&v: &Action| {
                match v {
                    Action::Toggle(toggle_action) => {
                        action.send(Action::Toggle(ToggleAction { enabled: true, kind: toggle_action.kind}))
                    },
                    default => {}
                }
            }));

    // Publish Toggle Action release events
    keys.get_just_released()
        .for_each(|&it: &KeyCode| input_bindings.bindings.iter()
            .filter(|(&k, &v)| k == it)
            .map(|(k, v): (&KeyCode, &Action)| v)
            .filter(|&v| match v { Action::Toggle(_) => true, default => false })
            .for_each(|&v: &Action| {
                match v {
                    Action::Toggle(toggle_action) => {
                        action.send(Action::Toggle(ToggleAction { enabled: false, kind: toggle_action.kind}))
                    },
                    default => {}
                }
            }));

    // Publish Axis Action events
    keys.get_pressed()
        .for_each(|&it: &KeyCode| input_bindings.bindings.iter()
            .filter(|(&k, &v)| k == it)
            .map(|(k, v): (&KeyCode, &Action)| v)
            .filter(|&v| match v { Action::Axis(_) => true, default => false })
            .for_each(|&v: &Action| action.send(v)));
}


/// Wrapper struct for the game's Input Bindings.
struct InputBindings {
    pub bindings: HashMap<KeyCode, Action>
}

// TODO | Promote this to some kind of Configuration resource
fn get_input_bindings() -> HashMap<KeyCode, Action> {
    [
        (KeyCode::W, Action::Axis(AxisAction { scale: 1.0, kind: AxisActionType::MOVE_FORWARD })),
        (KeyCode::S, Action::Axis(AxisAction { scale: -1.0, kind: AxisActionType::MOVE_FORWARD })),
        (KeyCode::A, Action::Axis(AxisAction { scale: -1.0, kind: AxisActionType::MOVE_STRAFE })),
        (KeyCode::D, Action::Axis(AxisAction { scale: 1.0, kind: AxisActionType::MOVE_STRAFE })),
        (KeyCode::C, Action::Toggle(ToggleAction { enabled: false, kind: ToggleActionType::CROUCH }))
    ]
        .iter()
        .cloned()
        .collect()
}
