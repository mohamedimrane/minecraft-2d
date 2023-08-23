use crate::block::{BlockBundle, BlockGraphics, BlockKind, BLOCK_SIZE};
use bevy::{math::vec2, prelude::*};
use bracket_noise::prelude::*;

// CONSTANTS
const WIDTH: i32 = 200;

// PLUGINS

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(WorldSettings {
                seed: 59900,
                // seed: 4332575,
                // seed: 43325075,
                octaves: 2,
                lacunarity: 5.0,
                terrain_frequency: 4.,
                cave_frequency: 3.,
                air_porbality: 0.26,
                dirt_layer_height: 3,
                height_multiplier: 40.,
                height_addition: 40.,
                divider: 140.,
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
    terrain_frequency: f32,
    cave_frequency: f32,
    air_porbality: f32,
    dirt_layer_height: i32,
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
    stgs: Res<WorldSettings>,
    block_graphics: Res<BlockGraphics>,
) {
    let mut noise = FastNoise::seeded(stgs.seed);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_octaves(stgs.octaves);
    // noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(stgs.lacunarity);
    // noise.set_frequency(stgs.frequency);

    commands
        .spawn((WorldBundle::default(), Name::new("World")))
        .with_children(|cb| {
            for x in 0..WIDTH {
                noise.set_frequency(stgs.terrain_frequency);

                let height = noise.get_noise(x as f32 / stgs.divider, 1. * stgs.cave_frequency)
                    * stgs.height_multiplier
                    + stgs.height_addition;

                for y in 0..=height as i32 {
                    noise.set_frequency(stgs.cave_frequency);

                    let v = noise.get_noise(x as f32 / stgs.divider, y as f32 / stgs.divider);

                    if v < stgs.air_porbality {
                        let mut kind = BlockKind::Stone;

                        if height as i32 - y <= stgs.dirt_layer_height {
                            kind = BlockKind::Dirt;
                        }

                        if y == height as i32 {
                            kind = BlockKind::Grass
                        }

                        cb.spawn(BlockBundle::new(
                            kind,
                            vec2(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE),
                            &block_graphics,
                        ));
                    }
                }
            }
        });
}

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
    spatial_bundle: SpatialBundle,
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            tag: World,
            // transform_bundle: TransformBundle {
            //     local: Transform::from_xyz(
            //         -WIDTH as f32 * BLOCK_SIZE / 2.,
            //         -WIDTH as f32 * BLOCK_SIZE / 8.,
            //         0.,
            //     ),
            //     ..default()
            // },
            spatial_bundle: default(),
        }
    }
}
