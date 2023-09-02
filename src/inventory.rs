use bevy::prelude::*;

use crate::block::BlockKind;

// CONSTANTS

const INVENTORY_SIZE: usize = 36;
const SLOT_SIZE: usize = 64;

// PLUGINS

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app 
            // Resources
            .insert_resource(CurrentItem::default())
            .insert_resource(Inventory::<INVENTORY_SIZE, SLOT_SIZE>::default())
            // Systems
            .add_systems(Update, manage_block_selection_inv)
            // Reflection
        ;
    }
}

// RESOURCES

#[derive(Resource, Default)]
pub struct CurrentItem(pub BlockKind);

#[derive(Resource)]
struct Inventory<const N: usize, const M: usize>([Option<InventorySlot>; N]);

#[derive(Clone, Copy, Default)]
struct InventorySlot {
    kind: BlockKind,
    quantity: usize,
}

#[derive(Clone, Copy)]
enum InventoryOpError {
    None,
    InvalidRetrieval,
    Overflow,
}

impl<const N: usize, const M: usize> Inventory<N, M> {
    fn add(&mut self, kind: BlockKind, quantity: usize) -> InventoryOpError {
        for slot in self.0.clone().iter_mut() {
            let Some(slot) = slot else { continue };

            if slot.kind != kind || slot.quantity == M {
                continue;
            }

            if slot.quantity + quantity > M {
                slot.quantity += M;
                return self.add(kind, quantity - M);
            }

            slot.quantity += quantity;
            return InventoryOpError::None;
        }

        for slot in self.0.iter_mut() {
            let None = slot else { continue };

            if quantity > M {
                *slot = Some(InventorySlot { kind, quantity: M });
                return self.add(kind, quantity - M);
            }

            *slot = Some(InventorySlot { kind, quantity });
            return InventoryOpError::None;
        }

        InventoryOpError::Overflow
    }

    fn retrieve(&mut self, kind: BlockKind, quantity: usize) -> InventoryOpError {
        for slot in self.0.clone().iter_mut() {
            let Some(mut some_slot) = slot else { continue };

            if some_slot.kind != kind {
                continue;
            }

            if some_slot.quantity < quantity {
                *slot = None;
                return self.retrieve(kind, quantity - M);
            }

            if some_slot.quantity == quantity {
                *slot = None;
                return InventoryOpError::None;
            }

            some_slot.quantity -= quantity;
            return InventoryOpError::None;
        }

        return InventoryOpError::InvalidRetrieval;
    }
}

impl<const N: usize, const M: usize> Default for Inventory<N, M> {
    fn default() -> Self {
        Self([Some(default()); N])
    }
}

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
