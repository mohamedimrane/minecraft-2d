use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};
use bevy_rapier2d::prelude::*;

// CONSTANTS

const RATIO: f32 = 4.75;
const HEAD_SIZE: f32 = 8. * RATIO;
const BODY_W_SIZE: f32 = 4. * RATIO;
const BODY_H_SIZE: f32 = 12. * RATIO;
const ARM_W_SIZE: f32 = 4. * RATIO;
const ARM_H_SIZE: f32 = 12. * RATIO;
const LEG_W_SIZE: f32 = 4. * RATIO;
const LEG_H_SIZE: f32 = 12. * RATIO;

const HEAD_OFFSET: f32 = 28.5;
const ARM_OFFSET: f32 = 28.5;
const LEG_OFFSET: f32 = -28.5;

// PLUGINS

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_controller_movement, animate_player))
            // Reflection
            .register_type::<PlayerGraphics>()
            .register_type::<Speed>()
            .register_type::<Jump>();
    }
}

// COMPONENTS

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerGraphicsHolder;

#[derive(Component, Reflect)]
enum PlayerGraphics {
    Head,
    Body,
    RightArm,
    LeftArm,
    RightLeg,
    LeftLeg,
}

#[derive(Component, Reflect)]
struct Speed(f32);

#[derive(Component, Reflect)]
struct Jump(f32);

// SYSTEMS

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((PlayerBundle::default(), Name::new("Player")))
        .with_children(|cb| {
            cb.spawn((
                PlayerGraphicsHolderBundle::default(),
                Name::new("Graphics Holder"),
            ))
            .with_children(|cb| {
                cb.spawn((PlayerGraphicsBundle::new_head(), Name::new("Head")));
                cb.spawn((PlayerGraphicsBundle::new_body(), Name::new("Body")));
                cb.spawn((
                    PlayerGraphicsBundle::new_right_arm(),
                    Name::new("Right Arm"),
                ));
                cb.spawn((PlayerGraphicsBundle::new_left_arm(), Name::new("Left Arm")));
                cb.spawn((
                    PlayerGraphicsBundle::new_right_leg(),
                    Name::new("Right Leg"),
                ));
                cb.spawn((PlayerGraphicsBundle::new_left_leg(), Name::new("Left Leg")));
            });
        });
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

fn animate_player(mut player_query: Query<(), With<Player>>) {}

// BUNDLES

#[derive(Bundle)]
struct PlayerBundle {
    // gameplay settings
    speed: Speed,
    jump: Jump,

    // colliders
    collider: Collider,

    // tags
    player: Player,

    // physics required
    rigid_body: RigidBody,
    friction: Friction,
    velocity: Velocity,
    ext_impulse: ExternalImpulse,
    locked_axes: LockedAxes,

    // required
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
}

#[derive(Bundle)]
struct PlayerGraphicsHolderBundle {
    // tags
    tag: PlayerGraphicsHolder,

    // required
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
}

#[derive(Bundle)]
struct PlayerGraphicsBundle {
    // rendering
    sprite: Sprite,
    texture: Handle<Image>,

    // tags
    tag: PlayerGraphics,

    // required
    transform_bundle: TransformBundle,
    visibility_bunde: VisibilityBundle,
}

impl PlayerBundle {
    fn new(speed: f32, jump_force: f32, collider: Collider) -> Self {
        Self {
            speed: Speed(speed),
            jump: Jump(jump_force),
            collider,
            player: Player,
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            velocity: Velocity::default(),
            ext_impulse: ExternalImpulse::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            transform_bundle: TransformBundle::default(),
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}

impl PlayerGraphicsBundle {
    fn new_head() -> Self {
        Self {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(HEAD_SIZE)),
                anchor: Anchor::BottomCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::Head,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., HEAD_OFFSET, 0.0),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_body() -> Self {
        Self {
            sprite: Sprite {
                color: Color::SEA_GREEN,
                custom_size: Some(Vec2::new(BODY_W_SIZE, BODY_H_SIZE)),
                anchor: Anchor::Center,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::Body,
            transform_bundle: TransformBundle::default(),
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_right_arm() -> Self {
        Self {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(ARM_W_SIZE, ARM_H_SIZE)),
                anchor: Anchor::TopCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::RightArm,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., ARM_OFFSET, 0.0),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_left_arm() -> Self {
        Self {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(ARM_W_SIZE, ARM_H_SIZE)),
                anchor: Anchor::TopCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::LeftArm,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., ARM_OFFSET, 0.0),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_right_leg() -> Self {
        Self {
            sprite: Sprite {
                color: Color::ORANGE_RED,
                custom_size: Some(Vec2::new(LEG_W_SIZE, LEG_H_SIZE)),
                anchor: Anchor::TopCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::RightLeg,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., LEG_OFFSET, 0.0),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_left_leg() -> Self {
        Self {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(LEG_W_SIZE, LEG_H_SIZE)),
                anchor: Anchor::TopCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::LeftLeg,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., LEG_OFFSET, 0.0),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            speed: Speed(300.),
            jump: Jump(100.),
            collider: Collider::cuboid(50., 50.),
            player: Player,
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            velocity: Velocity::default(),
            ext_impulse: ExternalImpulse::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            transform_bundle: default(),
            visibility_bundle: default(),
        }
    }
}

impl Default for PlayerGraphicsHolderBundle {
    fn default() -> Self {
        Self {
            tag: PlayerGraphicsHolder,
            transform_bundle: TransformBundle::default(),
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}
