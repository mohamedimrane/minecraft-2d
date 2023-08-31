use bevy::{math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_rapier2d::prelude::*;

// CONSTANTS
pub const BLOCK_SIZE: f32 = 70.;
const BLOCK_COLLIDER_SIZE: f32 = 35.;

const BLOCK_Z_INDEX: f32 = 0.;

// PLUGINS

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(BlockGraphics::default())
            // Systems
            .add_systems(PreStartup, load_block_graphics)
            // Reflection
            .register_type::<Block>()
            .register_type::<BlockKind>();
    }
}

// RESOURCES

#[derive(Resource)]
pub struct BlockGraphics {
    tex: Handle<Image>,
    pub atlas_handle: Handle<TextureAtlas>,
}

impl Default for BlockGraphics {
    fn default() -> Self {
        Self {
            tex: DEFAULT_IMAGE_HANDLE.typed(),
            atlas_handle: Handle::<TextureAtlas>::default(),
        }
    }
}

// COMPONENTS
#[derive(Component, Reflect)]
pub struct Block;

#[derive(Component, Default, Clone, Copy, Reflect)]
pub enum BlockKind {
    #[default]
    // row 1
    Dirt,
    Grass,
    Stone,
    Cobblestone,
    Deepslate,
    CobbledDeepslate,
    Bedrock,
    HayBale,
    OakLog,
    LeafedOakLog,
    OakPlank,
    Leaves,
    OakSapling,
    CraftingTable,
    Furnace,
    FurnaceBurning,
    // row 2
    Sand,
    Sandstone,
    RedSand,
    RedSandstone,
    Cactus,
    RedTulip,
    // row 3
    CoalOre,
    CoalOreDeepslate,
    CopperOre,
    CopperOreDeepslate,
    IronOre,
    IronOreDeepslate,
    GoldOre,
    GoldOreDeepslate,
    LapisOre,
    LapisOreDeepslate,
    RedstoneOre,
    RedstoneOreDeepslate,
    EmraldOre,
    EmraldOreDeepslate,
    DiamondOre,
    DiamondOreDeepslate,
}

impl BlockKind {
    pub fn to_index(&self) -> usize {
        use BlockKind::*;
        match *self {
            // row 1
            Dirt => 0,
            Grass => 1,
            Stone => 2,
            Cobblestone => 3,
            Deepslate => 4,
            CobbledDeepslate => 5,
            Bedrock => 6,
            HayBale => 7,
            OakLog => 8,
            LeafedOakLog => 9,
            OakPlank => 10,
            Leaves => 11,
            OakSapling => 12,
            CraftingTable => 13,
            Furnace => 14,
            FurnaceBurning => 15,
            // row 2
            Sand => 16,
            Sandstone => 17,
            RedSand => 18,
            RedSandstone => 19,
            Cactus => 20,
            RedTulip => 21,
            // row 3
            CoalOre => 32,
            CoalOreDeepslate => 33,
            CopperOre => 34,
            CopperOreDeepslate => 35,
            IronOre => 36,
            IronOreDeepslate => 37,
            GoldOre => 38,
            GoldOreDeepslate => 39,
            LapisOre => 40,
            LapisOreDeepslate => 41,
            RedstoneOre => 42,
            RedstoneOreDeepslate => 43,
            EmraldOre => 44,
            EmraldOreDeepslate => 45,
            DiamondOre => 46,
            DiamondOreDeepslate => 47,
            _ => 0,
        }
    }
}

// SYSTEMS

fn load_block_graphics(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut block_graphics: ResMut<BlockGraphics>,
) {
    block_graphics.tex = asset_server.load("blocks.png");
    let atlas = TextureAtlas::from_grid(
        block_graphics.tex.clone(),
        vec2(16., 16.),
        16,
        5,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(atlas);
    block_graphics.atlas_handle = atlas_handle;
}

// BUNDLES

#[derive(Bundle)]
pub struct BlockBundle {
    // colliders
    collider: Collider,

    // rendering
    sprite: TextureAtlasSprite,
    texture_atlas: Handle<TextureAtlas>,

    // tags
    kind: BlockKind,
    block: Block,

    // required
    spatial_bundle: SpatialBundle,
}

impl BlockBundle {
    pub fn new(kind: BlockKind, translation: Vec2, blocks_graphics: &BlockGraphics) -> Self {
        Self {
            collider: Collider::cuboid(BLOCK_COLLIDER_SIZE, BLOCK_COLLIDER_SIZE),
            kind,
            sprite: TextureAtlasSprite {
                index: kind.to_index(),
                custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                ..default()
            },
            texture_atlas: blocks_graphics.atlas_handle.clone(),
            block: Block,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(translation.x, translation.y, BLOCK_Z_INDEX),
                ..default()
            },
        }
    }
}
