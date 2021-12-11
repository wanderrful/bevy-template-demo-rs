mod plugins;
mod ui;
mod utils;

use bevy::prelude::*;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use crate::plugins::loading::LoadingPlugin;
use crate::plugins::menu::MenuPlugin;
use crate::plugins::input::InputPlugin;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::actions::ActionsPlugin;
use crate::plugins::physics::MyPhysicsPlugin;
use crate::plugins::spectator::SpectatorCameraPlugin;


// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // GameState
            .add_state(GameState::Loading)

            //Game-specific Plugins
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugins(SpectatorCameraPlugin)

            // My Physics Plugin
            .add_plugin(MyPhysicsPlugin)
        ;

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}
