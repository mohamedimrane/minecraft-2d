use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_controller_movement);
    }
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Jump(f32);

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(100.)),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Velocity::default(),
        ExternalImpulse::default(),
        Collider::cuboid(50., 50.),
        LockedAxes::ROTATION_LOCKED,
        Speed(300.),
        Jump(100.),
    ));
}

fn player_controller_movement(
    mut player_query: Query<(&Speed, &mut Velocity)>,
    keys: Res<Input<KeyCode>>,
) {
    for (speed, mut rb_vel) in player_query.iter_mut() {
        // let up = keys.any_pressed([KeyCode::W, KeyCode::Up]);
        // let down = keys.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;
        // let y_axis = -(down as i8) + up as i8;

        let move_delta_x = x_axis as f32;

        rb_vel.linvel.x = move_delta_x * speed.0;
    }
}
