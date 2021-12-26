use bevy::app::{AppBuilder, Plugin};


/// This Plugin registers Game Events, so that other systems can react to them.
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<LookUp>()
            .add_event::<LookRight>()
            .add_event::<MoveForward>()
            .add_event::<StrafeRight>()
            .add_event::<Crouch>()
            .add_event::<Jump>()
            .add_event::<ToggleConsole>()
            .add_event::<SpawnCubeActor>()
            .add_event::<SpawnSpectatorCamera>()
        ;
    }
}


// Types of actions
type AxisScale = f32;
type IsEnabled = bool;


// Mouse motion actions
pub struct LookUp(pub AxisScale);
pub struct LookRight(pub AxisScale);


// Movement actions
pub struct MoveForward(pub AxisScale);
pub struct StrafeRight(pub AxisScale);


// Stance actions
pub struct Crouch(pub IsEnabled);
pub struct Jump(pub IsEnabled);

// Debug actions
pub struct SpawnCubeActor;
pub struct SpawnSpectatorCamera;

pub struct ToggleConsole;

pub struct ExitGame;
