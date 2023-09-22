use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{block::BlockGraphics, item_kind::ItemKind};

// CONSTANTS

const FONT_NAME: &str = "Monocraft.ttf";

const INVENTORY_SIZE: usize = 36;
const HOTBAR_SIZE: usize = 9;
const STACK_SIZE: usize = 5;

const UI_HOTBAR_RATIO: f32 = 91. / 11.;
const UI_HOTBAR_BOTTOM_SPACING: f32 = 10.;

const UI_SLOT_ITEM_SIZE_RATIO: f32 = 10. / 13.;
const UI_SLOT_SIZE: f32 = 52.;
const UI_ITEM_SIZE: f32 = UI_SLOT_SIZE * UI_SLOT_ITEM_SIZE_RATIO;
const UI_ITEM_MARGIN: f32 = 5.;
// const UI_SLOT_SPACING: f32 = 4.;
const UI_SLOT_FONT_SIZE: f32 = 18.;
const UI_SLOT_FONT_SPACING: f32 = 2.;

// PLUGINS

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(UiAssets::default())
            .insert_resource(Inv::default())
            // Systems
            .add_systems(PreStartup, load_assets)
            .add_systems(Startup, spawn_ui)
            .add_systems(
                Update,
                (
                    manage_hotbar_cursor,
                    update_hotbar,
                    update_hotbar_selected_slot,
                ),
            )
            // Reflection
            .register_type::<Inv>()
            .add_plugins(ResourceInspectorPlugin::<Inv>::default());
    }
}

// SYSTEMS

fn load_assets(mut assets: ResMut<UiAssets>, asset_server: Res<AssetServer>) {
    assets.hotbar_tex = asset_server.load("hotbar.png");
    assets.hotbar_selected_slot_tex = asset_server.load("hotbar_selected_slot.png");
    assets.font = asset_server.load(FONT_NAME);
}

fn spawn_ui(mut commands: Commands, ui_assets: Res<UiAssets>, block_graphics: Res<BlockGraphics>) {
    commands
        .spawn((
            Name::new("Hotbar Holder"),
            NodeBundle {
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
            },
        ))
        .with_children(|cb| spawn_hotbar(cb, &ui_assets, &block_graphics));
}

fn update_hotbar(
    mut slot_texts: Query<
        (&mut Text, &mut Visibility, &SlotNumber),
        (With<SlotText>, Without<SlotImage>),
    >,
    mut slot_images: Query<
        (&mut UiTextureAtlasImage, &mut Visibility, &SlotNumber),
        (With<SlotImage>, Without<SlotText>),
    >,
    inventory: Res<Inv>,
) {
    for (mut slot_text, mut slot_visibility, slot_number) in slot_texts.iter_mut() {
        match inventory.items[slot_number.0 as usize] {
            Some(inventory_slot) => {
                *slot_visibility = Visibility::Inherited;
                slot_text.sections[0].value = inventory_slot.quantity.to_string();
            }
            None => *slot_visibility = Visibility::Hidden,
        }
    }

    for (mut slot_image, mut slot_visibility, slot_number) in slot_images.iter_mut() {
        match inventory.items[slot_number.0 as usize] {
            Some(inventory_image) => {
                *slot_visibility = Visibility::Inherited;
                slot_image.index = inventory_image.kind.to_index();
            }
            None => *slot_visibility = Visibility::Hidden,
        }
    }
}

fn update_hotbar_selected_slot(
    mut slot_selector: Query<&mut Style, With<SlotSelector>>,
    inventory: Res<Inv>,
) {
    if !inventory.is_changed() {
        return;
    }

    let cursor = inventory.hotbar_cursor as f32;
    slot_selector.single_mut().left = Val::Px(UI_SLOT_SIZE * cursor - cursor * UI_ITEM_MARGIN);
}

fn spawn_hotbar(
    cb: &mut ChildBuilder,
    ui_assets: &Res<UiAssets>,
    block_graphics: &Res<BlockGraphics>,
) {
    cb.spawn((
        Name::new("Hotbar"),
        HotbarUi,
        ImageBundle {
            // background_color: Color::RED.into(),
            image: UiImage {
                texture: ui_assets.hotbar_tex.clone(),
                ..default()
            },
            style: Style {
                width: Val::Px(
                    UI_SLOT_SIZE * UI_HOTBAR_RATIO, // HOTBAR_SIZE as f32 * UI_SLOT_SIZE + (HOTBAR_SIZE as f32 - 1.) * UI_SLOT_SPACING,
                ),
                height: Val::Px(UI_SLOT_SIZE),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
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
        spawn_slot_selector(cb, ui_assets);

        for i in 0..HOTBAR_SIZE {
            spawn_slot(cb, i as u8, ui_assets, block_graphics);
        }
    });
}

fn spawn_slot_selector(cb: &mut ChildBuilder, ui_assets: &Res<UiAssets>) {
    cb.spawn((
        Name::new("Slot Selector"),
        SlotSelector,
        ImageBundle {
            image: UiImage {
                texture: ui_assets.hotbar_selected_slot_tex.clone(),
                ..default()
            },
            style: Style {
                width: Val::Px(UI_SLOT_SIZE),
                height: Val::Px(UI_SLOT_SIZE),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_slot(
    cb: &mut ChildBuilder,
    number: u8,
    ui_assets: &Res<UiAssets>,
    block_graphics: &Res<BlockGraphics>,
) {
    cb.spawn((
        Name::new("Slot ".to_string() + &number.to_string()),
        Slot,
        SlotNumber(number),
        NodeBundle {
            // background_color: Color::BLUE.into(),
            style: Style {
                width: Val::Px(UI_ITEM_SIZE),
                height: Val::Px(UI_ITEM_SIZE),
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|cb| {
        spawn_slot_image(cb, number, block_graphics);
        spawn_slot_text(cb, number, ui_assets)
    });
}

fn spawn_slot_image(cb: &mut ChildBuilder, number: u8, block_graphics: &Res<BlockGraphics>) {
    cb.spawn((
        Name::new("Slot Image"),
        SlotImage,
        SlotNumber(number),
        AtlasImageBundle {
            texture_atlas: block_graphics.atlas_handle.clone(),
            texture_atlas_image: UiTextureAtlasImage {
                index: 0,
                ..default()
            },
            style: Style {
                margin: UiRect::all(Val::Px(UI_ITEM_MARGIN)),
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_slot_text(cb: &mut ChildBuilder, number: u8, ui_assets: &Res<UiAssets>) {
    cb.spawn((
        Name::new("Slot Text"),
        SlotText,
        SlotNumber(number),
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "64",
                    TextStyle {
                        font_size: UI_SLOT_FONT_SIZE,
                        font: ui_assets.font.clone(),
                        ..default()
                    },
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(UI_SLOT_FONT_SPACING),
                bottom: Val::Px(UI_SLOT_FONT_SPACING),
                ..default()
            },
            ..default()
        },
    ));
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
            KeyCode::Key6 => 5,
            KeyCode::Key7 => 6,
            KeyCode::Key8 => 7,
            KeyCode::Key9 => 8,
            _ => inventory.hotbar_cursor,
        };

        break;
    }
}

// RESOURCES

#[derive(Resource, Default)]
struct UiAssets {
    hotbar_tex: Handle<Image>,
    hotbar_selected_slot_tex: Handle<Image>,
    font: Handle<Font>,
}

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
struct Slot;

#[derive(Component)]
struct SlotSelector;

#[derive(Component)]
struct SlotImage;

#[derive(Component)]
struct SlotText;

#[derive(Component)]
struct SlotNumber(u8);
