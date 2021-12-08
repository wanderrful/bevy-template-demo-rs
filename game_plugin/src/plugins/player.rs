use bevy::prelude::*;
use heron::prelude::*;

use crate::GameState;
use crate::utils::Loggable;
use crate::utils::random_color;
use super::actions::{Action, ToggleAction, ToggleActionType};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_camera.system())
                    .with_system(spawn_light.system())
                    .with_system(spawn_floor.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_cube_actor_listener.system())
                    .with_system(bump_cube_actors.system()));
    }
}


/// Cube Mesh Actor
pub struct CubeActor;

fn spawn_cube_actor_listener(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_cube_actor_event: EventReader<Action>
) {
    spawn_cube_actor_event.iter()
        .filter(|&it| {
            match it {
                Action::Toggle(ToggleAction { enabled, kind: ToggleActionType::SPAWN_CUBE_ACTOR }) => {
                    return *enabled
                },
                default => { false }
            }
        })
        .for_each(|it| {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
                material: materials.add(random_color::get_random_color().into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            })
                .insert(CollisionShape::Sphere { radius: 1.0 })
                .insert(RigidBody::Dynamic)
                .insert(CubeActor);
        });
}


/// Player's 3D Camera
pub struct Camera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-10.0, 25.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Camera);
}


/// To query lights, use `bevy_pbr::Light`
fn spawn_light(mut commands: Commands) {
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        });
}


fn spawn_floor(mut commands: Commands) {
    commands
        .spawn_bundle((Transform::identity(), GlobalTransform::identity()))
        .insert(RigidBody::Static)
        .insert(CollisionShape::HeightField {
            size: Vec2::new(50., 50.),
            heights: vec![
                vec![1.5, 0.8, 0., 0., 3.0],
                vec![0.8, 0.2, 0., 0., 3.0],
                vec![0., 0.5, 0., 0., 3.0],
                vec![0., 0., 0.6, 0., 3.0],
                vec![3., 3., 3., 3., 3.0],
            ],
        });
}

fn bump_cube_actors(
    mut transforms: Query<&mut Transform, With<CubeActor>>,
    time: Res<Time>
) {
    if time.seconds_since_startup() as i64 % 5 == 0 {
        PlayerPlugin.log_debug(format!("hello! seconds_since_startup={}",
                    time.seconds_since_startup()).as_str());
        transforms.iter_mut()
            .for_each(|(mut it)| {
                let q: Vec3 = it.rotation.mul_vec3(-Vec3::Z) * 0.1;
                it.translation += q;
            });
    }
}
