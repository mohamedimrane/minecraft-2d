use bevy::prelude::*;

use crate::block::BlockKind;

// PLUGINS

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app 
            // Resources
            .insert_resource(CurrentItem::default())
            // Systems
            .add_systems(Update, manage_block_selection_inv)
            // Reflection
        ;
    }
}

// RESOURCES

#[derive(Resource, Default)]
pub struct CurrentItem(pub BlockKind);

// SYSTEMS

fn manage_block_selection_inv(
    mut current_item: ResMut<CurrentItem>,
    keys: Res<Input<KeyCode>>
) {
    for k in keys.get_pressed() {
        current_item.0 = match k {
            KeyCode::Key1 => BlockKind::Dirt, 
            KeyCode::Key2 => BlockKind::Grass, 
            KeyCode::Key3 => BlockKind::Stone, 
            KeyCode::Key4 => BlockKind::Cobblestone, 
            KeyCode::Key5 => BlockKind::Deepslate, 
            KeyCode::Key6 => BlockKind::CobbledDeepslate, 
            KeyCode::Key7 => BlockKind::Bedrock, 
            KeyCode::Key8 => BlockKind::HayBale, 
            KeyCode::Key9 => BlockKind::OakLog, 
            KeyCode::Key0 => BlockKind::OakPlank, 
            _ => current_item.0
        }
    }
}
