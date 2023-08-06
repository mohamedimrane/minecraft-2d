use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_platform);
    }
}

fn spawn_test_platform(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1000., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -300., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(500., 25.),
    ));
}
