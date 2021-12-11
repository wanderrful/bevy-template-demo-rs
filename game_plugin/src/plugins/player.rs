use bevy::prelude::*;
use heron::prelude::*;

use crate::GameState;
use crate::utils::random_color;
use super::actions;



/// An Entity with this Possessed Component means that the Player's inputs are being handled here.
///     TODO | How do I combine this with reading inputs for UI purposes?
pub struct Possessed;


// TODO | Refactor this PlayerPlugin to be a generic "spawn something" plugin... where we optionally
//  Possess the thing we're spawning?
/// PlayerPlugin allows us to spawn Characters such that the Player can possess them.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_light.system())
                    .with_system(spawn_floor.system())
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_cube_actor_listener.system())
                    // .with_system(bump_cube_actors.system())
            );
    }
}


/// Cube Mesh Actor
pub struct CubeActor;

fn spawn_cube_actor_listener(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_cube_actor_event: EventReader<actions::SpawnCubeActor>
) {
    spawn_cube_actor_event.iter()
        .for_each(|_it| {
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())),
                material: materials.add(random_color::get_random_color().into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            })
                .insert(CollisionShape::Sphere { radius: 1.0 })
                .insert(RigidBody::Dynamic)
                .insert(CubeActor);
        });
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


// TODO | Remove this test system
fn bump_cube_actors(
    mut transforms: Query<&mut Transform, With<CubeActor>>,
    time: Res<Time>
) {
    if time.seconds_since_startup() as i64 % 5 == 0 {
        transforms.iter_mut()
            .for_each(|mut it| {
                let q: Vec3 = it.rotation.mul_vec3(-Vec3::Z) * 0.1;
                it.translation += q;
            });
    }
}
