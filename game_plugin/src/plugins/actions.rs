/// Process Actions and Send corresponding Events

use bevy::prelude::*;

use crate::GameState;

/// This Plugin registers Game Events, so that other systems can react to them.
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MoveForward>()
            .add_event::<MoveStrafe>();
    }
}

/// GAME ACTIONS

/// Scale should be between -1.0 (move backward) and 1.0 (move forward).
pub struct MoveForward {
    pub scale: f32
}

/// Scale should be between -1.0 (strafe left) and 1.0 (strafe right).
pub struct MoveStrafe {
    pub scale: f32
}
