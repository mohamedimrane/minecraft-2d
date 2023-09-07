use bevy::prelude::*;

use crate::item_kind::ItemKind;

// CONSTANTS

const INVENTORY_SIZE: usize = 10;
const HOTBAR_SIZE: usize = 5;
const STACK_SIZE: usize = 3;

// PLUGINS

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app 
            // Resources
            .insert_resource(CurrentItem::default())
            .insert_resource(Inventory::<INVENTORY_SIZE, HOTBAR_SIZE, STACK_SIZE>::default())
            // Systems
            .add_systems(Update, manage_block_selection_inv)
            // Reflection
        ;
    }
}

// RESOURCES

#[derive(Resource, Default)]
pub struct CurrentItem(pub ItemKind);

#[derive(Resource)]
struct Inventory<const INVENTORY_SIZE: usize, const HOTBAR_SIZE: usize, const STACK_SIZE: usize>([Option<InventorySlot>; INVENTORY_SIZE]);

#[derive(Clone, Copy, Default)]
struct InventorySlot {
    kind: ItemKind,
    quantity: usize,
}

impl<const I: usize, const H: usize, const S: usize> Inventory<I, H, S> {
}

impl<const I: usize, const H: usize, const S: usize> Default for Inventory<I, H, S> {
    fn default() -> Self {
        Self([Some(default()); I])
    }
}

// SYSTEMS

fn manage_block_selection_inv(
    mut current_item: ResMut<CurrentItem>,
    keys: Res<Input<KeyCode>>
) {
    for k in keys.get_pressed() {
        current_item.0 = match k {
            KeyCode::Key1 => ItemKind::Dirt, 
            KeyCode::Key2 => ItemKind::Grass, 
            KeyCode::Key3 => ItemKind::Stone, 
            KeyCode::Key4 => ItemKind::Cobblestone, 
            KeyCode::Key5 => ItemKind::Deepslate, 
            KeyCode::Key6 => ItemKind::CobbledDeepslate, 
            KeyCode::Key7 => ItemKind::Bedrock, 
            KeyCode::Key8 => ItemKind::HayBale, 
            KeyCode::Key9 => ItemKind::OakLog, 
            KeyCode::Key0 => ItemKind::OakPlank, 
            _ => current_item.0
        }
    }
}
