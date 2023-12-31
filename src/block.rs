use bevy::{math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_rapier2d::prelude::*;

use crate::item_kind::ItemKind;

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
            .register_type::<ItemKind>();
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

// BUNDLES

#[derive(Bundle)]
pub struct BlockBundle {
    // rendering
    sprite: TextureAtlasSprite,
    texture_atlas: Handle<TextureAtlas>,

    // tags
    kind: ItemKind,
    block: Block,

    // required
    spatial_bundle: SpatialBundle,
}

impl BlockBundle {
    pub fn new(
        kind: ItemKind,
        translation: Vec2,
        blocks_graphics: &BlockGraphics,
    ) -> (Self, Collider) {
        if !kind.is_block() {
            panic!("Cannot spawn block of non block item {:?}", kind);
        }

        (
            Self {
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
            },
            Collider::cuboid(BLOCK_COLLIDER_SIZE, BLOCK_COLLIDER_SIZE),
        )
    }

    pub fn non_collidable(
        kind: ItemKind,
        translation: Vec2,
        blocks_graphics: &BlockGraphics,
    ) -> Self {
        if !kind.is_block() {
            panic!("Cannot spawn block of non block item {:?}", kind);
        }

        Self {
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
