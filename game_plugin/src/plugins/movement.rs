use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;

pub struct MovementPlugin;

/// Broadcast a MovementEvent to mutate an Actor's Transform!
pub struct MovementEvent(Entity);

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_event::<MovementEvent>()
            .add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))
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

fn on_tick(mut movement_events: EventReader<MovementEvent>) {
    for e in movement_events.iter() {
        MovementPlugin.log_info("Movement event detected!");
    }
}
