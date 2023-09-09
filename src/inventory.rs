use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{block::BlockGraphics, item_kind::ItemKind};

// CONSTANTS

const INVENTORY_SIZE: usize = 36;
const HOTBAR_SIZE: usize = 9;
const STACK_SIZE: usize = 5;

const UI_HOTBAR_BOTTOM_SPACING: f32 = 10.;
const UI_ITEM_SIZE: f32 = 40.;
const UI_SLOT_SIZE: f32 = UI_ITEM_SIZE * 1.4;
const UI_SLOT_SPACING: f32 = 5.;
const UI_SLOT_FONT_SIZE: f32 = 23.;

// PLUGINS

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(Inv::default())
            // Systems
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, manage_hotbar_cursor)
            // Reflection
            .register_type::<Inv>()
            .add_plugins(ResourceInspectorPlugin::<Inv>::default());
    }
}

// SYSTEMS

fn spawn_ui(mut commands: Commands, block_graphics: Res<BlockGraphics>) {
    commands
        .spawn((NodeBundle {
            // background_color: Color::GREEN.into(),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|cb| spawn_hotbar(cb, &block_graphics));
}

fn spawn_hotbar(cb: &mut ChildBuilder, block_graphics: &Res<BlockGraphics>) {
    cb.spawn((
        HotbarUi,
        NodeBundle {
            // background_color: Color::RED.into(),
            style: Style {
                width: Val::Px(
                    HOTBAR_SIZE as f32 * UI_SLOT_SIZE + (HOTBAR_SIZE as f32 - 1.) * UI_SLOT_SPACING,
                ),
                height: Val::Px(UI_SLOT_SIZE),
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect {
                    bottom: Val::Px(UI_HOTBAR_BOTTOM_SPACING),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|cb| {
        for i in 0..HOTBAR_SIZE {
            spawn_slot(cb, block_graphics);
        }
    });
}

fn spawn_slot(cb: &mut ChildBuilder, block_graphics: &Res<BlockGraphics>) {
    cb.spawn((
        SlotUi,
        NodeBundle {
            // background_color: Color::BLUE.into(),
            style: Style {
                width: Val::Px(UI_SLOT_SIZE),
                height: Val::Px(UI_SLOT_SIZE),
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|cb| {
        spawn_slot_image(cb, block_graphics);
        spawn_slot_text(cb)
    });
}

fn spawn_slot_image(cb: &mut ChildBuilder, block_graphics: &Res<BlockGraphics>) {
    cb.spawn(AtlasImageBundle {
        texture_atlas: block_graphics.atlas_handle.clone(),
        texture_atlas_image: UiTextureAtlasImage {
            index: 0,
            ..default()
        },
        style: Style {
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            ..default()
        },
        ..default()
    });
}

fn spawn_slot_text(cb: &mut ChildBuilder) {
    cb.spawn(TextBundle {
        text: Text {
            sections: vec![TextSection::new(
                "1",
                TextStyle {
                    font_size: UI_SLOT_FONT_SIZE,
                    ..default()
                },
            )],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(UI_SLOT_SPACING + 3.),
            bottom: Val::Px(3.),
            ..default()
        },
        ..default()
    });
}

// fn toggle_ui(mut ui: Query<&mut Visibility, With<InventoryUi>>, keys: Res<Input<KeyCode>>) {
//     if !keys.just_pressed(KeyCode::O) {
//         return;
//     }

//     let mut ui = ui.single_mut();
//     *ui = match *ui {
//         Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
//         Visibility::Hidden => Visibility::Inherited,
//     };
// }

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

// RESOURCES

pub type Inv = Inventory<INVENTORY_SIZE, HOTBAR_SIZE, STACK_SIZE>;

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

// COMPONENTS

#[derive(Component)]
struct HotbarUi;

#[derive(Component)]
struct SlotUi;
