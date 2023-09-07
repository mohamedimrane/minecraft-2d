use crate::{
    block::{BlockBundle, BlockGraphics, BlockKind, BLOCK_SIZE},
    player::Player,
    utils::in_bounds_y as inside,
};
use bevy::{math::vec2, prelude::*};
use bevy_editor_pls::default_windows::inspector;
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use bracket_noise::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// CONSTANTS
const WORLD_OFFSET: Vec3 = Vec3::new(0., -BLOCK_SIZE * 40., 0.);
const CHUNK_SIZE: i32 = 16;
const CHUNK_RENDER_DISTANCE: i32 = 8;

// PLUGINS

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(WorldSettings {
                seed: 0,
                octaves: 2,
                lacunarity: 5.0,

                biomes: Biomes {
                    frequency: 0.8,
                    divider: 100.,

                    plain: BiomeSettings {
                        v: 0.,

                        terrain_frequency: 1.,
                        terrain_divider: 200.,

                        // cave_frequency: 1.1,
                        // cave_divider: 10.,
                        cave_frequency: 13.,
                        cave_divider: 200.,

                        height_multiplier: 40.,
                        height_addition: 90.,

                        // air_porbality: 0.15,
                        // air_porbality: 0.008,
                        // air_porbality: 0.25,
                        air_porbality: -0.13,
                        exposed_block_top_layer_height: 1,
                        exposed_block_layer_height: 3,
                        tree_kind: TreeKind::Oak,
                        tree_chance: 6,

                        exposed_block_top: BlockKind::Grass,
                        exposed_block: BlockKind::Dirt,

                        ores_map_step: 10,

                        coal: OreSettings {
                            rarity: 0.5,
                            size: -0.18,
                            divider: 3.,
                            below: None,
                            above: Some(20),
                        },

                        copper: OreSettings {
                            rarity: 1.,
                            size: -0.18,
                            divider: 4.,
                            below: None,
                            above: Some(20),
                        },

                        iron: OreSettings {
                            rarity: 3.5,
                            size: -0.18,
                            divider: 10.,
                            below: None,
                            above: None,
                        },

                        gold: OreSettings {
                            rarity: 4.5,
                            size: -0.25,
                            divider: 8.,
                            below: Some(30),
                            above: Some(5),
                        },

                        diamond: OreSettings {
                            rarity: 3.5,
                            size: -0.25,
                            divider: 7.,
                            below: Some(15),
                            above: None,
                        },
                    },
                    // desert: BiomeSettings {
                    //     v: 0.,

                    //     terrain_frequency: 2.,
                    //     terrain_divider: 140.,

                    //     cave_frequency: 3.5,
                    //     cave_divider: 140.,

                    //     height_multiplier: 40.,
                    //     height_addition: 90.,

                    //     air_porbality: 0.18,
                    //     exposed_block_top_layer_height: 4,
                    //     exposed_block_layer_height: 2,
                    //     tree_kind: TreeKind::Cactus,
                    //     tree_chance: 20,

                    //     exposed_block_top: BlockKind::Sand,
                    //     exposed_block: BlockKind::Sandstone,

                    //     ores_map_step: 10,

                    //     coal: OreSettings {
                    //         rarity: 0.5,
                    //         size: -0.18,
                    //         divider: 3.,
                    //         below: None,
                    //         above: Some(20),
                    //     },

                    //     copper: OreSettings {
                    //         rarity: 1.,
                    //         size: -0.18,
                    //         divider: 4.,
                    //         below: None,
                    //         above: Some(20),
                    //     },

                    //     iron: OreSettings {
                    //         rarity: 3.5,
                    //         size: -0.18,
                    //         divider: 10.,
                    //         below: None,
                    //         above: None,
                    //     },

                    //     gold: OreSettings {
                    //         rarity: 4.5,
                    //         size: -0.25,
                    //         divider: 8.,
                    //         below: Some(50),
                    //         above: None,
                    //     },

                    //     diamond: OreSettings {
                    //         rarity: 3.5,
                    //         size: -0.25,
                    //         divider: 7.,
                    //         below: Some(15),
                    //         above: None,
                    //     },
                    // },
                },
            })
            .insert_resource(PlayerChunkPosition(0))
            // Systems
            // .add_systems(Startup, spawn_test_platform)
            .add_systems(Startup, spawn_world)
            .add_systems(Update, (update_player_chunk_pos, refresh_world))
            // Reflection
            // .register_type::<WorldSettings>()
            // .add_plugins(ResourceInspectorPlugin::<WorldSettings>::default())
        ;
    }
}

// RESOURCES

#[derive(Resource, Reflect, InspectorOptions)]
struct WorldSettings {
    seed: u64,
    octaves: i32,
    lacunarity: f32,

    biomes: Biomes,
}

#[derive(Resource)]
pub struct PlayerChunkPosition(pub i32);

// STRUCTS
#[derive(Reflect, InspectorOptions)]
struct Biomes {
    frequency: f32,
    divider: f32,
    plain: BiomeSettings,
    // desert: BiomeSettings,
}

/// Frequency compresses values in the x axis
/// The greater the frequency, the more compressed the values are
/// freq = 1 -> 0000001111110000000000011110000
/// freq = 2 -> 000111000001100

/// Divider impacts variation in the y axis
/// The greater the divider, the less varied the heights are
/// div = 1 -> 013420035335601
/// div = 2 -> 001234443332100
#[derive(Reflect, InspectorOptions)]
struct BiomeSettings {
    v: f32,

    terrain_frequency: f32,
    terrain_divider: f32,
    cave_frequency: f32,
    cave_divider: f32,

    height_multiplier: f32,
    height_addition: f32,

    air_porbality: f32,
    exposed_block_top_layer_height: i32,
    exposed_block_layer_height: i32,
    tree_kind: TreeKind,
    /// The greater the rarer
    tree_chance: i32,

    exposed_block_top: BlockKind,
    exposed_block: BlockKind,

    ores_map_step: i32,
    coal: OreSettings,
    copper: OreSettings,
    iron: OreSettings,
    gold: OreSettings,
    // lapis: OreSettings,
    // redstone: OreSettings,
    // emrald: OreSettings,
    diamond: OreSettings,
}

#[derive(Reflect, InspectorOptions)]
struct OreSettings {
    rarity: f32,
    size: f32,
    divider: f32,
    below: Option<i32>,
    above: Option<i32>,
}

#[derive(Clone, Copy, Reflect)]
enum TreeKind {
    Oak,
    Cactus,
}

// COMPONENTS

#[derive(Component)]
pub struct World;

#[derive(Component)]
pub struct Chunk;

#[derive(Component)]
pub struct ChunkPosition(pub i32);

#[derive(Component)]
struct BlockPosition(Vec2);

// SYSTEMS

fn spawn_world(mut commands: Commands) {
    commands.spawn((WorldBundle::default(), Name::new("World")));
}

fn update_player_chunk_pos(
    player_transform: Query<&GlobalTransform, With<Player>>,
    mut player_chunk_pos: ResMut<PlayerChunkPosition>,
) {
    let player_transform = player_transform.single().translation();

    let chunk_x = (player_transform.x / (CHUNK_SIZE as f32 * BLOCK_SIZE)).round();
    if chunk_x != player_chunk_pos.0 as f32 {
        player_chunk_pos.0 = chunk_x as i32;
    }
}

fn refresh_world(
    mut commands: Commands,
    world: Query<Entity, With<World>>,
    chunks_pos: Query<(Entity, &ChunkPosition), With<Chunk>>,
    player_chunk_pos: Res<PlayerChunkPosition>,
    settings: Res<WorldSettings>,
    block_graphics: Res<BlockGraphics>,
    // mut first_time_not: Local<bool>,
) {
    // let first_time = !*first_time_not;

    // if !first_time && !settings.is_changed() {
    //     return;
    // }

    // *first_time_not = !false;

    // let Ok(world_ent) = world.get_single() else { return };
    // commands.entity(world_ent).despawn_recursive();

    // let mut rng = ChaCha8Rng::seed_from_u64(settings.seed);

    // let mut noise = FastNoise::seeded(settings.seed);
    // noise.set_noise_type(NoiseType::PerlinFractal);
    // noise.set_fractal_octaves(settings.octaves);
    // noise.set_fractal_lacunarity(settings.lacunarity);

    // commands
    //     .spawn((WorldBundle::default(), Name::new("World")))
    //     .with_children(|cb| {
    //         for i in (player_chunk_pos.0 - CHUNK_RENDER_DISTANCE)
    //             ..=(player_chunk_pos.0 + CHUNK_RENDER_DISTANCE)
    //         {
    //             if !chunks_pos.iter().any(|x| i == x.1 .0) {
    //                 generate_chunk(i, cb, &mut noise, &mut rng, &settings, &block_graphics);
    //             }
    //         }
    //     });

    if !player_chunk_pos.is_changed() {
        return;
    }

    let world_ent = world.single();

    let mut rng = ChaCha8Rng::seed_from_u64(settings.seed);

    let mut noise = FastNoise::seeded(settings.seed);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_octaves(settings.octaves);
    noise.set_fractal_lacunarity(settings.lacunarity);

    for chunk_pos in chunks_pos.iter() {
        if player_chunk_pos.0 < chunk_pos.1 .0 - CHUNK_RENDER_DISTANCE
            || player_chunk_pos.0 > chunk_pos.1 .0 + CHUNK_RENDER_DISTANCE
        {
            commands.entity(chunk_pos.0).despawn_recursive();
        }
    }

    commands.entity(world_ent).with_children(|cb| {
        for i in (player_chunk_pos.0 - CHUNK_RENDER_DISTANCE)
            ..=(player_chunk_pos.0 + CHUNK_RENDER_DISTANCE)
        {
            if !chunks_pos.iter().any(|x| i == x.1 .0) {
                generate_chunk(i, cb, &mut noise, &mut rng, &settings, &block_graphics);
            }
        }
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
        ChunkBundle::new(chunk_x),
        Name::new(format!("Chunk {}", chunk_x)),
    ))
    .with_children(|cb| {
        let mut blocks = Vec::<Vec2>::new();

        noise.set_frequency(stgs.biomes.frequency);
        for x in (CHUNK_SIZE * chunk_x) + 1..=(CHUNK_SIZE * (chunk_x) + CHUNK_SIZE) {
            noise.set_frequency(stgs.biomes.frequency);
            let biome_v = noise.get_noise(x as f32 / stgs.biomes.divider, 1.);
            // let bstgs = match biome_v {
            //     _ if biome_v <= stgs.biomes.desert.v => &stgs.biomes.desert,
            //     _ if biome_v <= stgs.biomes.plain.v => &stgs.biomes.plain,
            //     _ => return,
            // };

            let bstgs = &stgs.biomes.plain;

            noise.set_frequency(bstgs.terrain_frequency);

            let height = noise
                .get_noise(x as f32 / bstgs.terrain_divider, 1. * bstgs.cave_frequency)
                * bstgs.height_multiplier
                + bstgs.height_addition;

            for y in 0..=height as i32 {
                let name = Name::new(format!("Block {}:{}", x, y));

                if y == 0 {
                    cb.spawn((
                        BlockBundle::new(
                            BlockKind::Bedrock,
                            vec2(x as f32 * BLOCK_SIZE, 0.),
                            &block_graphics,
                        ),
                        name,
                    ));

                    continue;
                }

                noise.set_frequency(bstgs.cave_frequency);

                let v =
                    noise.get_noise(x as f32 / bstgs.cave_divider, y as f32 / bstgs.cave_divider);

                if v > bstgs.air_porbality
                    && !blocks.iter().any(|&b| b.x == x as f32 && b.y == y as f32)
                {
                    let mut kind = BlockKind::Stone;

                    noise.set_frequency(bstgs.coal.rarity);
                    let coal_v = noise
                        .get_noise(x as f32 / bstgs.coal.divider, y as f32 / bstgs.coal.divider);

                    noise.set_frequency(bstgs.copper.rarity);
                    let copper_v = noise.get_noise(
                        (x + bstgs.ores_map_step) as f32 / bstgs.copper.divider,
                        (y + bstgs.ores_map_step) as f32 / bstgs.copper.divider,
                    );

                    noise.set_frequency(bstgs.iron.rarity);
                    let iron_v = noise.get_noise(
                        (x + bstgs.ores_map_step * 2) as f32 / bstgs.iron.divider,
                        (y + bstgs.ores_map_step * 2) as f32 / bstgs.iron.divider,
                    );

                    noise.set_frequency(bstgs.gold.rarity);
                    let gold_v = noise.get_noise(
                        (x + bstgs.ores_map_step * 3) as f32 / bstgs.gold.divider,
                        (y + bstgs.ores_map_step * 3) as f32 / bstgs.gold.divider,
                    );

                    noise.set_frequency(bstgs.diamond.rarity);
                    let diamond_v = noise.get_noise(
                        (x + bstgs.ores_map_step * 4) as f32 / bstgs.diamond.divider,
                        (y + bstgs.ores_map_step * 4) as f32 / bstgs.diamond.divider,
                    );

                    match true {
                        _ if is_ore(diamond_v, y, &bstgs.diamond) => kind = BlockKind::DiamondOre,
                        _ if is_ore(gold_v, y, &bstgs.gold) => kind = BlockKind::GoldOre,
                        _ if is_ore(iron_v, y, &bstgs.iron) => kind = BlockKind::IronOre,
                        _ if is_ore(copper_v, y, &bstgs.copper) => kind = BlockKind::CopperOre,
                        _ if is_ore(coal_v, y, &bstgs.coal) => kind = BlockKind::CoalOre,
                        _ => {}
                    }

                    if height as i32 - y
                        < bstgs.exposed_block_layer_height + bstgs.exposed_block_top_layer_height
                    {
                        kind = bstgs.exposed_block;
                    }

                    if height as i32 - y < bstgs.exposed_block_top_layer_height {
                        kind = bstgs.exposed_block_top;
                    }

                    if y == height as i32 {
                        if rng.gen_bool(1. / bstgs.tree_chance as f64) {
                            spawn_tree(
                                bstgs.tree_kind,
                                x as f32,
                                y as f32 + 1.,
                                &mut blocks,
                                cb,
                                &block_graphics,
                            );
                        }
                    }

                    cb.spawn((
                        BlockBundle::new(
                            kind,
                            vec2(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE),
                            &block_graphics,
                        ),
                        name,
                    ));

                    blocks.push(vec2(x as f32, y as f32));
                }
            }
        }
    });
}

fn is_ore(v: f32, y: i32, ore_stgs: &OreSettings) -> bool {
    v < ore_stgs.size && inside(ore_stgs.below, ore_stgs.above, y)
}

fn spawn_tree(
    kind: TreeKind,
    x: f32,
    y: f32,
    blocks: &mut Vec<Vec2>,
    commands: &mut ChildBuilder,
    block_graphics: &Res<BlockGraphics>,
) {
    use TreeKind::*;
    match kind {
        Oak => {
            for i in 0..=5 {
                if blocks
                    .iter()
                    .any(|&b| b.x == x as f32 && b.y == y + i as f32)
                {
                    continue;
                }

                let kind = if i >= 3 {
                    BlockKind::LeafedOakLog
                } else {
                    BlockKind::OakLog
                };

                commands.spawn((
                    BlockBundle::non_collidable(
                        kind,
                        vec2(x * BLOCK_SIZE, (y + i as f32) * BLOCK_SIZE),
                        block_graphics,
                    ),
                    Name::new(format!("Block {}:{}", x, y + i as f32)),
                ));

                blocks.push(vec2(x, y + i as f32));
            }

            for i in -2..=2 {
                for j in 0..=3 {
                    if i == 0
                        || blocks
                            .iter()
                            .any(|&b| b.x == x + i as f32 && b.y == y + j as f32 + 2.)
                    {
                        continue;
                    }

                    commands.spawn((
                        BlockBundle::non_collidable(
                            BlockKind::Leaves,
                            vec2(
                                (x + i as f32) * BLOCK_SIZE,
                                (y + j as f32 + 2.) * BLOCK_SIZE,
                            ),
                            block_graphics,
                        ),
                        Name::new(format!("Block {}:{}", x + i as f32, y + j as f32)),
                    ));

                    blocks.push(vec2(x + i as f32, y + j as f32 + 2.));
                }
            }

            for i in -1..=1 {
                if blocks.iter().any(|&b| b.x == x + i as f32 && b.y == y + 6.) {
                    continue;
                }

                commands.spawn((
                    BlockBundle::non_collidable(
                        BlockKind::Leaves,
                        vec2((x + i as f32) * BLOCK_SIZE, (y + 6.) * BLOCK_SIZE),
                        block_graphics,
                    ),
                    Name::new(format!("Block {}:{}", x + i as f32, y + 6.)),
                ));

                blocks.push(vec2(x + i as f32, y + 6.));
            }
        }
        Cactus => {
            for i in 0..=2 {
                commands.spawn((
                    BlockBundle::non_collidable(
                        BlockKind::Cactus,
                        vec2(x * BLOCK_SIZE, (y + i as f32) * BLOCK_SIZE),
                        block_graphics,
                    ),
                    Name::new(format!("Block {}:{}", x, y + i as f32)),
                ));
            }
        }
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
    // game related
    chunk_position: ChunkPosition,

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
                transform: Transform::from_translation(WORLD_OFFSET),
                ..default()
            },
        }
    }
}

impl ChunkBundle {
    fn new(x: i32) -> Self {
        Self {
            chunk_position: ChunkPosition(x),
            tag: Chunk,
            spatial_bundle: default(),
        }
    }
}

impl Default for ChunkBundle {
    fn default() -> Self {
        Self {
            chunk_position: ChunkPosition(0),
            tag: Chunk,
            spatial_bundle: default(),
        }
    }
}
