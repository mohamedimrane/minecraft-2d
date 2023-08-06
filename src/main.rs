use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;
use world::WorldPlugin;

mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
            PlayerPlugin,
            WorldPlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -1000.),
            ..default()
        })
        .add_systems(Startup, setup_graphics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
