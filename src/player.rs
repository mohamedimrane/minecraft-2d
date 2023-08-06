use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor};
use bevy_rapier2d::prelude::*;

// PLUGINS

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_controller_movement, animate_player));
    }
}

// COMPONENTS

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerGraphicsHolder;

#[derive(Component)]
enum PlayerGraphics {
    Head,
    Body,
    RightArm,
    LeftArm,
    RightLeg,
    LeftLeg,
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Jump(f32);

// SYSTEMS

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default()).with_children(|cb| {
        cb.spawn(PlayerGraphicsHolderBundle::default())
            .with_children(|cb| {
                cb.spawn(PlayerGraphicsBundle::new_head());
                cb.spawn(PlayerGraphicsBundle::new_body());
                cb.spawn(PlayerGraphicsBundle::new_right_arm());
                cb.spawn(PlayerGraphicsBundle::new_left_arm());
                cb.spawn(PlayerGraphicsBundle::new_right_leg());
                cb.spawn(PlayerGraphicsBundle::new_left_leg());
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

    // rendering
    // sprite: Sprite,
    // texture: Handle<Image>,

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
    fn new(
        speed: f32,
        jump_force: f32,
        collider: Collider,
        sprite: Sprite,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            speed: Speed(speed),
            jump: Jump(jump_force),
            collider,
            // sprite,
            // texture,
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
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(10.)),
                anchor: Anchor::BottomCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::Head,
            transform_bundle: TransformBundle::default(),
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_body() -> Self {
        Self {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 60.)),
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
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 50.)),
                anchor: Anchor::TopCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::RightArm,
            transform_bundle: TransformBundle::default(),
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_left_arm() -> Self {
        Self {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 50.)),
                anchor: Anchor::TopCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::LeftArm,
            transform_bundle: TransformBundle::default(),
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_right_leg() -> Self {
        Self {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 50.)),
                anchor: Anchor::BottomCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::RightLeg,
            transform_bundle: TransformBundle::default(),
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_left_leg() -> Self {
        Self {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(10., 50.)),
                anchor: Anchor::BottomCenter,
                // rect: Some(Rect::new(, , , )),
                ..default()
            },
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            tag: PlayerGraphics::LeftLeg,
            transform_bundle: TransformBundle::default(),
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
            // sprite: Sprite {
            //     color: Color::WHITE,
            //     custom_size: Some(Vec2::splat(100.)),
            //     ..default()
            // },
            // texture: DEFAULT_IMAGE_HANDLE.typed(),
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
