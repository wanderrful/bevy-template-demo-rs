/// Process Actions and Send corresponding Events
use bevy::prelude::*;

use crate::GameState;

/// This Plugin registers Game Events, so that other systems can react to them.
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AxisAction>();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AxisActionType {
    MOVE_FORWARD,
    MOVE_STRAFE
}

#[derive(Clone, Copy)]
pub struct AxisAction {
    pub scale: f32,
    pub kind: AxisActionType
}