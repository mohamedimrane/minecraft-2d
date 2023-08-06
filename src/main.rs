use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            PlayerPlugin,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0., -1000.),
            ..default()
        })
        .add_systems(Startup, (setup_graphics, setup_physics))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(1000., 50.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -300., 0.),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(500., 25.),
    ));
}
