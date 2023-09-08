use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

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
            .insert_resource(Inv::default())
            // Systems
            .add_systems(Update, (manage_block_selection_inv, manage_hotbar_cursor))
            // Reflection
            .register_type::<Inv>()
            .add_plugins(ResourceInspectorPlugin::<Inv>::default());
    }
}

// RESOURCES

pub type Inv = Inventory<INVENTORY_SIZE, HOTBAR_SIZE, STACK_SIZE>;

#[derive(Resource, Default)]
pub struct CurrentItem(pub ItemKind);

#[derive(Resource, Reflect)]
pub struct Inventory<const INVENTORY_SIZE: usize, const HOTBAR_SIZE: usize, const STACK_SIZE: usize>
{
    pub items: [Option<InventorySlot>; INVENTORY_SIZE],
    pub hotbar_cursor: usize,
}

#[derive(Clone, Copy, Default, Reflect)]
pub struct InventorySlot {
    pub kind: ItemKind,
    pub quantity: usize,
}

impl<const I: usize, const H: usize, const S: usize> Inventory<I, H, S> {
    pub fn add<T>(&mut self, kind: ItemKind, if_accepted_callback: T)
    where
        T: FnOnce(),
    {
        for slot in self.items.iter_mut() {
            let Some(ref mut slot) = slot else { continue };
            if !(slot.kind == kind && slot.quantity < 4) {
                continue;
            }
            slot.quantity += 1;

            if_accepted_callback();
            // commands.entity(parent_ent).despawn_recursive();

            return;
        }

        for slot in self.items.iter_mut() {
            let None = slot else { continue };
            *slot = Some(InventorySlot { kind, quantity: 1 });

            if_accepted_callback();
            // commands.entity(parent_ent).despawn_recursive();

            return;
        }
    }

    pub fn remove_at_cursor(&mut self) {
        let Some(ref mut current_slot) = self.items[self.hotbar_cursor] else { return };

        current_slot.quantity -= 1;

        if current_slot.quantity == 0 {
            self.items[self.hotbar_cursor] = None;
        }
    }

    pub fn current_hotbar_slot(&self) -> &Option<InventorySlot> {
        &self.items[self.hotbar_cursor]
    }

    pub fn current_hotbar_slot_mut(&mut self) -> &mut Option<InventorySlot> {
        &mut self.items[self.hotbar_cursor]
    }

    pub fn shift_cursor_right(&mut self) {
        if self.hotbar_cursor == H - 1 {
            self.hotbar_cursor = 1;
            return;
        }

        self.hotbar_cursor = 0;
    }

    pub fn shift_cursor_left(&mut self) {
        if self.hotbar_cursor == 0 {
            self.hotbar_cursor = H;
            return;
        }

        self.hotbar_cursor -= 1;
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        if cursor > H - 1 {
            panic!("cannot set hotbar cursor out of bounds");
        }

        self.hotbar_cursor = cursor;
    }
}

impl<const I: usize, const H: usize, const S: usize> Default for Inventory<I, H, S> {
    fn default() -> Self {
        Self {
            items: [None; I],
            hotbar_cursor: 0,
        }
    }
}

// SYSTEMS

fn manage_block_selection_inv(mut current_item: ResMut<CurrentItem>, keys: Res<Input<KeyCode>>) {
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
            _ => current_item.0,
        }
    }
}

fn manage_hotbar_cursor(mut inventory: ResMut<Inv>, keys: Res<Input<KeyCode>>) {
    for k in keys.get_pressed() {
        inventory.hotbar_cursor = match k {
            KeyCode::Key1 => 0,
            KeyCode::Key2 => 1,
            KeyCode::Key3 => 2,
            KeyCode::Key4 => 3,
            KeyCode::Key5 => 4,
            // KeyCode::Key6 => 5,
            // KeyCode::Key7 => 6,
            // KeyCode::Key8 => 7,
            // KeyCode::Key9 => 8,
            _ => inventory.hotbar_cursor,
        };

        break;
    }
}
