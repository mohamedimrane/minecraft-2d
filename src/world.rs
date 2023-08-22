use crate::block::{BlockBundle, BlockGraphics, BlockKind, BLOCK_SIZE};
use bevy::{math::vec2, prelude::*};
use bracket_noise::prelude::*;

// CONSTANTNS
const WIDTH: i32 = 200;

// PLUGINS

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(WorldSettings {
                seed: 59900,
                octaves: 3,
                lacunarity: 5.0,
                frequency: 3.,
                height_multiplier: 40.,
                height_addition: 40.,
                divider: 150.,
            })
            // Systems
            // .add_systems(Startup, spawn_test_platform)
            .add_systems(Startup, generate_world);
    }
}

// RESOURCES
#[derive(Resource)]
struct WorldSettings {
    seed: u64,
    octaves: i32,
    lacunarity: f32,
    frequency: f32,
    height_multiplier: f32,
    height_addition: f32,
    divider: f32,
}

// COMPONENTS
#[derive(Component)]
pub struct World;

// SYSTEMS

fn generate_world(
    mut commands: Commands,
    settings: Res<WorldSettings>,
    block_graphics: Res<BlockGraphics>,
) {
    let mut noise = FastNoise::seeded(settings.seed);
    noise.set_noise_type(NoiseType::Perlin);
    noise.set_fractal_octaves(settings.octaves);
    // noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(settings.lacunarity);
    noise.set_frequency(settings.frequency);

    commands
        .spawn((WorldBundle::default(), Name::new("World")))
        .with_children(|cb| {
            for x in 0..WIDTH {
                let height = noise.get_noise(x as f32 / settings.divider / 2., 0.)
                    * settings.height_multiplier
                    + settings.height_addition;

                for y in 0..height as i32 {
                    let v =
                        noise.get_noise(x as f32 / settings.divider, y as f32 / settings.divider);

                    if v < 40. / settings.divider {
                        let kind = if y == height as i32 - 1 {
                            BlockKind::Grass
                        } else {
                            BlockKind::Dirt
                        };

                        cb.spawn(BlockBundle::new(
                            kind,
                            vec2(BLOCK_SIZE * x as f32, BLOCK_SIZE * y as f32),
                            &block_graphics,
                        ));
                    }
                }
            }
        });
}

// fn spawn_test_platform(mut commands: Commands, block_graphics: Res<BlockGraphics>) {
//     commands
//         .spawn((WorldBundle::default(), Name::new("World")))
//         .with_children(|cb| {
//             for i in -10..=10 {
//                 cb.spawn(BlockBundle::new(
//                     BlockKind::Grass,
//                     vec2(60. * i as f32, -300.),
//                     &block_graphics,
//                 ));

//                 cb.spawn(BlockBundle::new(
//                     BlockKind::Dirt,
//                     vec2(60. * i as f32, -360.),
//                     &block_graphics,
//                 ));
//             }
//         });

//     // commands.spawn((
//     //     SpriteBundle {
//     //         sprite: Sprite {
//     //             color: Color::WHITE,
//     //             custom_size: Some(Vec2::new(1000., 50.)),
//     //             ..default()
//     //         },
//     //         transform: Transform::from_xyz(0., -300., 0.),
//     //         ..default()
//     //     },
//     //     RigidBody::Fixed,
//     //     Collider::cuboid(500., 25.),
//     // ));
// }

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
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(
                    -WIDTH as f32 * BLOCK_SIZE / 2.,
                    -WIDTH as f32 * BLOCK_SIZE / 8.,
                    0.,
                ),
                ..default()
            },
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}
