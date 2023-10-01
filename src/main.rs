use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use block::BlockPlugin;
use camera::CamPlugin;
use gamemode::GameModePlugin;
use inventory::InventoryPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod block;
mod camera;
mod gamemode;
mod inventory;
mod item;
mod item_kind;
mod player;
mod utils;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(
                    ImagePlugin::default_nearest(), // Pixel perfect camera
                )
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            AudioPlugin,
            WorldInspectorPlugin::new(),
            // EditorPlugin::default(),
            PlayerPlugin,
            WorldPlugin,
            BlockPlugin,
            InventoryPlugin,
            GameModePlugin,
            CamPlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -1000.),
            ..default()
        })
        .insert_resource(ClearColor(Color::rgba(126. / 255., 200. / 255., 1., 1.)))
        .run();
}
