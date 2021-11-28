use bevy::prelude::*;

use crate::GameState;
use crate::utils::Loggable;

pub struct MovementPlugin;
pub struct MovementEvent;

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
    MovementPlugin.debug("hello");
}

fn on_exit() {
    MovementPlugin.debug("goodbye");
}

fn on_tick(mut movement_events: EventReader<MovementEvent>) {

}