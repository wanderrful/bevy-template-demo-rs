/// Process Actions and Send corresponding Events
use bevy::prelude::*;

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
    /// Used for forward/backward movement
    MOVE_FORWARD,

    /// Used for sideways movement
    MOVE_STRAFE,

    /// Used for left-right mouse movement
    MOUSE_MOTION_X,

    /// Used for up-down mouse movement
    MOUSE_MOTION_Y
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