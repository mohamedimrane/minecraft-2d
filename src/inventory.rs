use bevy::prelude::*;

use crate::item_kind::ItemKind;

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
pub struct CurrentItem(pub ItemKind);

#[derive(Resource)]
struct Inventory<const N: usize, const M: usize>([Option<InventorySlot>; N]);

#[derive(Clone, Copy, Default)]
struct InventorySlot {
    kind: ItemKind,
    quantity: usize,
}

#[derive(Clone, Copy)]
enum InventoryOpError {
    None,
    InvalidRetrieval,
    Overflow,
}

impl<const N: usize, const M: usize> Inventory<N, M> {
    fn add(&mut self, kind: ItemKind, quantity: usize) -> InventoryOpError {
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

    fn retrieve(&mut self, kind: ItemKind, quantity: usize) -> InventoryOpError {
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
