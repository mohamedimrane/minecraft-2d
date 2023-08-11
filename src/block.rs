use bevy::{math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

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
            ;
    }
}

// RESOURCES

#[derive(Resource)]
struct BlockGraphics {
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

// SYSTEMS

fn load_block_graphics(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut block_graphics: ResMut<BlockGraphics>,
) {
    block_graphics.tex = asset_server.load("blocks.png");
    let atlas =
        TextureAtlas::from_grid(block_graphics.tex.clone(), vec2(16., 16.), 4, 1, None, None);
    let atlas_handle = texture_atlases.add(atlas);
    block_graphics.atlas_handle = atlas_handle;
}
