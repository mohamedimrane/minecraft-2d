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
    mut camera_tr: Query<&mut Transform, With<MainCamera>>,
    player_tr: Query<&GlobalTransform, With<Player>>,
) {
    let mut camera_tr = camera_tr.single_mut();
    let player_tr = player_tr.single().translation();

    camera_tr.translation.x = player_tr.x;
    camera_tr.translation.y = player_tr.y;
}

// COMPONENTS

#[derive(Component)]
pub struct MainCamera;
