use crate::plugins::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use crate::plugins::actions::{MoveForward, MoveStrafe};

pub struct PlayerPlugin;

pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_camera.system()),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(textures.texture_bevy.clone().into()),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..Default::default()
    })
        .insert(Player);
}