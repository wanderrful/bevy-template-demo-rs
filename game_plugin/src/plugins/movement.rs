use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::plugins::actions::{MoveStrafe, MoveForward};
use crate::plugins::player::Player;

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
    mut player_query: Query<&mut Transform, With<Player>>,
    mut move_forward: EventReader<MoveForward>,
    mut move_strafe: EventReader<MoveStrafe>,
    time: Res<Time>,
) {
    const speed: f32 = 150.0;
    let mut delta = Vec3::ZERO;
    let delta_seconds: f32 = time.delta_seconds();

    for ev in move_forward.iter() {
        MovementPlugin.log_info("event=MoveForward");
        delta.y += speed * ev.scale * delta_seconds;
    }

    for ev in move_strafe.iter() {
        MovementPlugin.log_info("event=MoveStrafe");
        delta.x += speed * ev.scale * delta_seconds;
    }

    for mut transform in player_query.iter_mut() {
        transform.translation += delta;
    }
}
