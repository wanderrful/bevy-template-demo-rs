use bevy::prelude::*;
use bevy::ecs::system::QuerySingleError;
use bevy::app::PluginGroupBuilder;

use crate::GameState;
use crate::plugins::actions::{
    MoveForward, StrafeRight, LookUp, LookRight, Crouch, Jump, SpawnSpectatorCamera
};
use crate::plugins::player::Possessed;


/// External-facing Plugin. Use this to add to your project!
pub struct SpectatorCameraPlugin;

impl PluginGroup for SpectatorCameraPlugin {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(SpectatorPlugin);
    }
}


/// This MovementPlugin will read inputs for any Entity that has a `Mobile` component.
struct SpectatorPlugin;

impl Plugin for SpectatorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        const GAME_STATE: GameState = GameState::Playing;

        app.add_system_set(SystemSet::on_enter(GAME_STATE)
                .with_system(on_enter.system()))

            .add_system_set(SystemSet::on_update(GAME_STATE)
                // Spawn when event broadcast is received
                .with_system(on_spawn_spectator_camera_listener.system())

                // Handle Player inputs
                .with_system(on_update_move_forward.system())
                .with_system(on_update_move_strafe.system())
                .with_system(on_update_crouch.system())
                .with_system(on_update_jump.system())
                .with_system(on_update_look_up.system())
                .with_system(on_update_look_right.system()))

            .add_system_set(SystemSet::on_exit(GAME_STATE)
                .with_system(on_exit.system()));
    }
}

fn on_enter() {
    debug!("on_enter");
}

fn on_exit() {
    debug!("on_exit");
}


/// FPS Spectator Flying Camera (Possessed)
pub struct SpectatorCamera;

fn on_spawn_spectator_camera_listener(
    mut commands: Commands,
    mut spawn_spectator_camera: EventReader<SpawnSpectatorCamera>,
    existing_cameras: Query<Entity, (With<Possessed>, With<SpectatorCamera>)>
) {
    spawn_spectator_camera.iter()
        .for_each(|it| {
            // Spawn if there is no already-possessed SpectatorCamera
            if let Err(QuerySingleError::NoEntities(_)) = existing_cameras.single() {
                info!("Spawning Spectator Camera...");
                commands
                    .spawn_bundle(PerspectiveCameraBundle {
                        transform: Transform::from_xyz(-10.0, 25.0, 25.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        ..Default::default()
                    })
                    .insert(Possessed)
                    .insert(SpectatorCamera);
            } else {
                info!("Possessed Spectator Camera already exists! Ignoring spawn attempt...");
            }
        });
}



// TODO | Promote these to configurations of some kind!
//  Maybe it can be a proprety of Possessed? Or Character?
const SPEED: f32 = 15.0;
const SENSITIVITY: f32 = 10.0;

fn on_update_move_forward(
    mut player_query: Query<&mut Transform, With<Possessed>>,
    mut actions: EventReader<MoveForward>,
    time: Res<Time>
) {
    actions.iter().for_each(|action: &MoveForward| {
        player_query.iter_mut().for_each(|mut transform| {
            let delta = -1.0 * transform.local_z() * SPEED * action.0 * time.delta_seconds();
            let new_position = transform.translation + delta;
            transform.translation = new_position;
        });
    });
}

fn on_update_move_strafe(
    mut player_query: Query<&mut Transform, With<Possessed>>,
    mut actions: EventReader<StrafeRight>,
    time: Res<Time>
) {
    actions.iter().for_each(|action: &StrafeRight| {
        player_query.iter_mut().for_each(|mut transform| {
            let delta = transform.local_x() * SPEED * action.0 * time.delta_seconds();
            let new_position = transform.translation + delta;
            transform.translation = new_position;
        });
    });
}

fn on_update_crouch(
    mut player_query: Query<&mut Transform, With<Possessed>>,
    mut actions: EventReader<Crouch>,
    time: Res<Time>
) {
    actions.iter()
        .filter(|action| action.0)
        .for_each(|action: &Crouch| {
            player_query.iter_mut().for_each(|mut transform| {
                let delta = -Vec3::Y * SPEED * time.delta_seconds();
                let new_position = transform.translation + delta;
                transform.translation = new_position;
            });
        });
}

fn on_update_jump(
    mut player_query: Query<&mut Transform, With<Possessed>>,
    mut actions: EventReader<Jump>,
    time: Res<Time>
) {
    actions.iter()
        .filter(|action| action.0)
        .for_each(|action: &Jump| {
            player_query.iter_mut().for_each(|mut transform| {
                let delta = Vec3::Y * SPEED * time.delta_seconds();
                let new_position = transform.translation + delta;
                transform.translation = new_position;
            });
        });
}

fn on_update_look_up(
    mut player_query: Query<&mut Transform, With<Possessed>>,
    mut actions: EventReader<LookUp>,
    time: Res<Time>
) {
    actions.iter().for_each(|action: &LookUp| {
        let delta: f32 = (action.0 * time.delta_seconds() * SENSITIVITY).to_radians();
        player_query.iter_mut().for_each(|mut transform| {
            let new_rotation = transform.rotation * Quat::from_axis_angle(
                -transform.local_x(),
                delta
            );

            transform.rotation = new_rotation.normalize();
        });
    });
}

fn on_update_look_right(
    mut player_query: Query<&mut Transform, With<Possessed>>,
    mut actions: EventReader<LookRight>,
    time: Res<Time>
) {
    actions.iter().for_each(|action: &LookRight| {
        let delta: f32 = (action.0 * time.delta_seconds() * SENSITIVITY).to_radians();
        player_query.iter_mut().for_each(|mut transform| {
            let new_rotation = Quat::from_axis_angle(
                -Vec3::Y,
                delta
            ) * transform.rotation;

            transform.rotation = new_rotation.normalize();
        });
    });
}
