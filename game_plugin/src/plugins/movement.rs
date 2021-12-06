use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::plugins::actions::{Action, AxisAction, AxisActionType, ToggleActionType};
use crate::plugins::player::CubeActor;

pub struct MovementPlugin;

pub const MOVEMENT_SYSTEM: &str = "MovementPlugin::on_move";

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(on_update.system().label(MOVEMENT_SYSTEM)))
            .add_system_set(SystemSet::on_exit(GAME_STATE)
                .with_system(on_exit.system()));
    }
}

fn on_enter() {
    MovementPlugin.log_debug("on_enter");
}

fn on_exit() {
    MovementPlugin.log_debug("on_exit");
}

fn on_update(
    mut player_query: Query<&mut Transform, With<CubeActor>>,
    mut actions: EventReader<Action>,
    time: Res<Time>,
) {
    const speed: f32 = 5.0;
    let mut delta = Vec3::ZERO;
    let delta_seconds: f32 = time.delta_seconds();

    for given_action in actions.iter() {
        match given_action {
            Action::Axis(axis_action) => {
                let modifier: f32 = speed * axis_action.scale * delta_seconds;

                match axis_action.kind {
                    AxisActionType::MOVE_FORWARD => {
                        delta.z += modifier;
                    },
                    AxisActionType::MOVE_STRAFE => {
                        delta.x += modifier;
                    }
                }
            },

            Action::Toggle(toggle_action) => {
                match toggle_action.kind {
                    ToggleActionType::CROUCH => {
                        MovementPlugin.log_error(format!("TODO Crouch! enabled={}", toggle_action.enabled).as_str())
                    },
                    default => {}
                }
            },
            default => {}
        }
    }

    for mut transform in player_query.iter_mut() {
        transform.translation += delta;
    }
}
