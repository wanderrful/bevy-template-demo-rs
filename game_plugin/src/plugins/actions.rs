/// Process Actions and Send corresponding Events
use bevy::prelude::*;

use crate::GameState;

/// This Plugin registers Game Events, so that other systems can react to them.
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<Action>();
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Action {
    Axis(AxisAction),
    Toggle(ToggleAction)
}


#[derive(Debug, Clone, Copy)]
pub enum AxisActionType {
    MOVE_FORWARD,
    MOVE_STRAFE
}

#[derive(Debug, Clone, Copy)]
pub struct AxisAction {
    pub scale: f32,
    pub kind: AxisActionType
}


#[derive(Debug, Clone, Copy)]
pub enum ToggleActionType {
    /// Use this to Crouch
    CROUCH,

    /// Use this to Spawn a Cube Actor
    SPAWN_CUBE_ACTOR
}

#[derive(Debug, Clone, Copy)]
pub struct ToggleAction {
    pub enabled: bool,
    pub kind: ToggleActionType
}