use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::block::{BlockBundle, BlockGraphics, BlockKind};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_platform);
    }
}

fn spawn_test_platform(mut commands: Commands, block_graphics: Res<BlockGraphics>) {
    for i in -10..=10 {
        commands.spawn(BlockBundle::new(
            BlockKind::Grass,
            vec2(60. * i as f32, -300.),
            &block_graphics,
        ));
        commands.spawn(BlockBundle::new(
            BlockKind::Dirt,
            vec2(60. * i as f32, -360.),
            &block_graphics,
        ));
    }

    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::WHITE,
    //             custom_size: Some(Vec2::new(1000., 50.)),
    //             ..default()
    //         },
    //         transform: Transform::from_xyz(0., -300., 0.),
    //         ..default()
    //     },
    //     RigidBody::Fixed,
    //     Collider::cuboid(500., 25.),
    // ));
}
