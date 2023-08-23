use crate::block::{BlockBundle, BlockGraphics, BlockKind, BLOCK_SIZE};
use bevy::{math::vec2, prelude::*};
use bracket_noise::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// CONSTANTS
const CHUNK_SIZE: i32 = 16;

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
                tree_chance: 8,
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
    /// The greater the rarer
    tree_chance: i32,
    height_multiplier: f32,
    height_addition: f32,
    divider: f32,
}

// COMPONENTS
#[derive(Component)]
pub struct World;

#[derive(Component)]
pub struct Chunk;

// SYSTEMS

fn generate_world(
    mut commands: Commands,
    stgs: Res<WorldSettings>,
    block_graphics: Res<BlockGraphics>,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(stgs.seed);

    let mut noise = FastNoise::seeded(stgs.seed);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_octaves(stgs.octaves);
    noise.set_fractal_lacunarity(stgs.lacunarity);

    commands
        .spawn((WorldBundle::default(), Name::new("World")))
        .with_children(|cb| {
            generate_chunk(0, cb, &mut noise, &mut rng, &stgs, &block_graphics);
        });
}

fn generate_chunk(
    chunk_x: i32,
    cb: &mut ChildBuilder,
    noise: &mut FastNoise,
    rng: &mut ChaCha8Rng,
    stgs: &Res<WorldSettings>,
    block_graphics: &Res<BlockGraphics>,
) {
    cb.spawn((
        ChunkBundle::default(),
        Name::new(format!("Chunk x{}", chunk_x)),
    ))
    .with_children(|cb| {
        for x in (CHUNK_SIZE * chunk_x) + 1..=(CHUNK_SIZE * (chunk_x) + CHUNK_SIZE) {
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
                        kind = BlockKind::Grass;

                        if rng.gen_bool(1. / stgs.tree_chance as f64) {
                            spawn_tree(x as f32, y as f32 + 1., cb, &block_graphics);
                        }
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

fn spawn_tree(x: f32, y: f32, commands: &mut ChildBuilder, block_graphics: &Res<BlockGraphics>) {
    for i in 0..=5 {
        let kind = if i >= 3 {
            BlockKind::LeafedOakLog
        } else {
            BlockKind::OakLog
        };

        commands.spawn(BlockBundle::new(
            kind,
            vec2(x * BLOCK_SIZE, (y + i as f32) * BLOCK_SIZE),
            block_graphics,
        ));
    }

    for i in -2..=2 {
        for j in 0..=3 {
            if i == 0 {
                continue;
            }

            commands.spawn(BlockBundle::new(
                BlockKind::Leaves,
                vec2(
                    (x + i as f32) * BLOCK_SIZE,
                    (y + j as f32 + 2.) * BLOCK_SIZE,
                ),
                block_graphics,
            ));
        }
    }

    for i in -1..=1 {
        commands.spawn(BlockBundle::new(
            BlockKind::Leaves,
            vec2((x + i as f32) * BLOCK_SIZE, (y + 6.) * BLOCK_SIZE),
            block_graphics,
        ));
    }
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

#[derive(Bundle)]
struct ChunkBundle {
    // tags
    tag: Chunk,

    // required
    spatial_bundle: SpatialBundle,
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            tag: World,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(
                    BLOCK_SIZE * -CHUNK_SIZE as f32 / 2.,
                    -BLOCK_SIZE * 40.,
                    0.,
                ),
                ..default()
            },
        }
    }
}

impl Default for ChunkBundle {
    fn default() -> Self {
        Self {
            tag: Chunk,
            spatial_bundle: default(),
        }
    }
}
