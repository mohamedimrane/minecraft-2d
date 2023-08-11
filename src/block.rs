use bevy::{math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_rapier2d::prelude::*;

// CONSTANTS
const BLOCK_SPRITE_SIZE: f32 = 60.;
const BLOCK_COLLIDER_SIZE: f32 = 30.;

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
    atlas_handle: Handle<TextureAtlas>,
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
struct Block;

#[derive(Component, Default, Clone, Copy, Reflect)]
pub enum BlockKind {
    #[default]
    Dirt,
    Grass,
    Stone,
    Cobblestone,
    Deepslate,
    CobbledDeepslate,
    Bedrock,
    HayBale,
    OakLog,
    OakPlank,
    Leaves,
    OakSapling,
    CraftingTable,
    Furnace,
    FurnaceBurning,
    RedTulip,
}

impl BlockKind {
    fn to_index(&self) -> usize {
        use BlockKind::*;
        match *self {
            Dirt => 0,
            Grass => 1,
            Stone => 2,
            Cobblestone => 3,
            Deepslate => 4,
            CobbledDeepslate => 5,
            Bedrock => 6,
            HayBale => 7,
            OakLog => 8,
            OakPlank => 9,
            Leaves => 10,
            OakSapling => 11,
            CraftingTable => 12,
            Furnace => 13,
            FurnaceBurning => 14,
            RedTulip => 20,
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
        11,
        2,
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
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
}

impl BlockBundle {
    pub fn new(kind: BlockKind, translation: Vec2, blocks_graphics: &BlockGraphics) -> Self {
        Self {
            collider: Collider::cuboid(BLOCK_COLLIDER_SIZE, BLOCK_COLLIDER_SIZE),
            kind,
            sprite: TextureAtlasSprite {
                index: kind.to_index(),
                custom_size: Some(Vec2::splat(BLOCK_SPRITE_SIZE)),
                ..default()
            },
            texture_atlas: blocks_graphics.atlas_handle.clone(),
            block: Block,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(translation.x, translation.y, BLOCK_Z_INDEX),
                ..default()
            },
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}
