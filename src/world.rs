use bevy::{math::vec2, prelude::*};

use crate::block::{BlockBundle, BlockGraphics, BlockKind};

// PLUGINS

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_platform);
    }
}

// COMPONENTS
#[derive(Component)]
struct World;

// SYSTEMS

fn spawn_test_platform(mut commands: Commands, block_graphics: Res<BlockGraphics>) {
    commands
        .spawn((WorldBundle::default(), Name::new("World")))
        .with_children(|cb| {
            for i in -10..=10 {
                cb.spawn(BlockBundle::new(
                    BlockKind::Grass,
                    vec2(60. * i as f32, -300.),
                    &block_graphics,
                ));

                cb.spawn(BlockBundle::new(
                    BlockKind::Dirt,
                    vec2(60. * i as f32, -360.),
                    &block_graphics,
                ));
            }
        });

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

// BUNDLES

#[derive(Bundle)]
struct WorldBundle {
    // tags
    tag: World,

    // required
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            tag: World,
            transform_bundle: TransformBundle::default(),
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}
