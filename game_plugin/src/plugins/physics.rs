use bevy::prelude::*;
use heron::prelude::*;

use crate::GameState;

pub struct MyPhysicsPlugin;

impl Plugin for MyPhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Heron's PhysicsPlugin, which is a wrapper for Rapier's PhysicsPlugin
            .add_plugin(PhysicsPlugin::default())

            // When in the Playing GameState, gravity should be enabled!
            // TODO | Make gravity customizable via Event!
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(init_gravity.system())
            );
    }
}

/// Insert the Gravity constraint for the Physics engine
fn init_gravity(mut commands: Commands) {
    commands
        .insert_resource(Gravity::from(Vec3::new(0., -9.81, 0.)));
}
