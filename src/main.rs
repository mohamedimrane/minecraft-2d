use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use block::BlockPlugin;
use camera::CamPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod block;
mod camera;
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
            WorldInspectorPlugin::new(),
            PlayerPlugin,
            WorldPlugin,
            BlockPlugin,
            CamPlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -1000.),
            ..default()
        })
        .run();
}
