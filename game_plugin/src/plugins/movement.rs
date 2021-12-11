use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::plugins::actions::{Action, AxisAction, AxisActionType};
use crate::plugins::player::CubeActor;

/// This MovementPlugin will read inputs for any Entity that has a `Mobile` component.
pub struct MobileCamera;

pub struct MovementPlugin;

pub const MOVEMENT_SYSTEM: &str = "MovementPlugin::on_move";

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
            .add_system_set(SystemSet::on_update(GAME_STATE)
                .with_system(on_update_movement.system().label(MOVEMENT_SYSTEM))
                .with_system(on_update_rotation.system().label(MOVEMENT_SYSTEM)))
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

// TODO | Combine the action and transform updates so that we can press multiple buttons together
//          ^ Every action type should have its own listener system!
// TODO | Promote the speed constant to a Component field so that we can customize it!
fn on_update_movement(
    mut player_query: Query<&mut Transform, With<MobileCamera>>,
    mut actions: EventReader<Action>,
    time: Res<Time>,
) {
    const SPEED: f32 = 15.0;
    let mut delta = 0.0;
    let mut movement_type: Option<AxisActionType> = None;

    for given_action in actions.iter() {
        match given_action {
            Action::Axis(axis_action) => {
                let modifier: f32 = SPEED * axis_action.scale * time.delta_seconds();

                movement_type = Some(axis_action.kind);

                delta = modifier;
            },

            _default => {}
        }
    }

    for mut transform in player_query.iter_mut() {
        if let Some(blah) = movement_type {
            transform.translation = match blah {
                // Get Forward Vector
                AxisActionType::MOVE_FORWARD =>
                    transform.translation + (-1.0 * transform.local_z() * delta),

                // Get Right Vector
                AxisActionType::MOVE_STRAFE =>
                    transform.translation + (1.0 * transform.local_x() * delta),

                // Failure case (no-op)
                _default => transform.translation
            };
        }
    }
}

/// Provide for FPS mouse rotation
fn on_update_rotation(
    mut player_query: Query<&mut Transform, With<MobileCamera>>,
    mut actions: EventReader<Action>,
    time: Res<Time>,
) {
    actions.iter().for_each(|it| {
        const SENSITIVITY: f32 = 10.0;

        match it {
            Action::Axis(AxisAction { scale, kind }) => {
                match kind {
                    AxisActionType::MOUSE_MOTION_X => {
                        let delta: f32 = (scale * time.delta_seconds() * SENSITIVITY).to_radians();
                        player_query.iter_mut().for_each(|mut transform| {
                            let new_rotation = Quat::from_axis_angle(
                                -Vec3::Y,
                                delta
                            ) * transform.rotation;

                            transform.rotation = new_rotation.normalize();
                        });
                    },
                    AxisActionType::MOUSE_MOTION_Y => {
                        let delta: f32 = (scale * time.delta_seconds() * SENSITIVITY).to_radians();
                        player_query.iter_mut().for_each(|mut transform| {
                            let new_rotation = Quat::from_axis_angle(
                                -transform.local_x(),
                                delta
                            ) * transform.rotation;

                            transform.rotation = new_rotation.normalize();
                        });
                    },
                    _default => { }
                }
            },
            _default => { }
        }
    });
}
