use bevy::prelude::*;

use crate::player::Player;

// PLUGINS

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(PostUpdate, camera_follow_player);
    }
}

// SYSTEMS

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn camera_follow_player(
    mut camera_transfrom: Query<&mut Transform, With<MainCamera>>,
    player_transform: Query<&GlobalTransform, With<Player>>,
) {
    let mut camera_transform = camera_transfrom.single_mut();
    let player_transform = player_transform.single().translation();

    camera_transform.translation.x = player_transform.x;
    camera_transform.translation.y = player_transform.y;
}

// COMPONENTS

#[derive(Component)]
pub struct MainCamera;
