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

const HEAD_Z_INDEX: f32 = 1.;
const BODY_Z_INDEX: f32 = 0.;
const FRONT_ARM_Z_INDEX: f32 = 2.;
const BACK_ARM_Z_INDEX: f32 = -1.;
const FRONT_LEG_Z_INDEX: f32 = 2.;
const BACK_LEG_Z_INDEX: f32 = -1.;

// PLUGINS

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(PlayerGraphics::default())
            // Systems
            .add_systems(PreStartup, load_player_graphics)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_controller_movement, animate_player))
            // Reflection
            .register_type::<PlayerGraphicsPart>()
            .register_type::<Speed>()
            .register_type::<Jump>();
    }
}

// RESOURCES

#[derive(Resource)]
struct PlayerGraphics {
    tex: Handle<Image>,
    head: Rect,
    body: Rect,
    right_arm: Rect,
    left_arm: Rect,
    right_leg: Rect,
    left_leg: Rect,
}

impl Default for PlayerGraphics {
    fn default() -> Self {
        Self {
            tex: DEFAULT_IMAGE_HANDLE.typed(),
            head: Rect::new(0., 0., 8., 8.),
            body: Rect::new(8., 0., 12., 12.),
            right_arm: Rect::new(12., 0., 16., 12.),
            left_arm: Rect::new(16., 0., 20., 12.),
            right_leg: Rect::new(20., 0., 24., 12.),
            left_leg: Rect::new(24., 0., 28., 12.),
        }
    }
}

// COMPONENTS

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerGraphicsHolder;

#[derive(Component, Reflect)]
enum PlayerGraphicsPart {
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

fn load_player_graphics(
    asset_server: Res<AssetServer>,
    mut player_graphics: ResMut<PlayerGraphics>,
) {
    *player_graphics = PlayerGraphics {
        tex: asset_server.load("player.png"),
        head: Rect::new(0., 0., 8., 8.),
        body: Rect::new(8., 0., 12., 12.),
        right_arm: Rect::new(12., 0., 16., 12.),
        left_arm: Rect::new(16., 0., 20., 12.),
        right_leg: Rect::new(20., 0., 24., 12.),
        left_leg: Rect::new(24., 0., 28., 12.),
    }
}

fn spawn_player(mut commands: Commands, graphics: Res<PlayerGraphics>) {
    commands
        .spawn((PlayerBundle::default(), Name::new("Player")))
        .with_children(|cb| {
            cb.spawn((
                PlayerGraphicsHolderBundle::default(),
                Name::new("Graphics Holder"),
            ))
            .with_children(|cb| {
                cb.spawn((
                    PlayerGraphicsPartBundle::new_head(&graphics),
                    Name::new("Head"),
                ));
                cb.spawn((
                    PlayerGraphicsPartBundle::new_body(&graphics),
                    Name::new("Body"),
                ));
                cb.spawn((
                    PlayerGraphicsPartBundle::new_right_arm(&graphics),
                    Name::new("Right Arm"),
                ));
                cb.spawn((
                    PlayerGraphicsPartBundle::new_left_arm(&graphics),
                    Name::new("Left Arm"),
                ));
                cb.spawn((
                    PlayerGraphicsPartBundle::new_right_leg(&graphics),
                    Name::new("Right Leg"),
                ));
                cb.spawn((
                    PlayerGraphicsPartBundle::new_left_leg(&graphics),
                    Name::new("Left Leg"),
                ));
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
struct PlayerGraphicsPartBundle {
    // rendering
    sprite: Sprite,
    texture: Handle<Image>,

    // tags
    tag: PlayerGraphicsPart,

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

impl PlayerGraphicsPartBundle {
    fn new_head(gr: &PlayerGraphics) -> Self {
        Self {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(HEAD_SIZE)),
                anchor: Anchor::BottomCenter,
                rect: Some(gr.head),
                ..default()
            },
            texture: gr.tex.clone(),
            tag: PlayerGraphicsPart::Head,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., HEAD_OFFSET, HEAD_Z_INDEX),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_body(gr: &PlayerGraphics) -> Self {
        Self {
            sprite: Sprite {
                custom_size: Some(Vec2::new(BODY_W_SIZE, BODY_H_SIZE)),
                anchor: Anchor::Center,
                rect: Some(gr.body),
                ..default()
            },
            texture: gr.tex.clone(),
            tag: PlayerGraphicsPart::Body,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., 0., BODY_Z_INDEX),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_right_arm(gr: &PlayerGraphics) -> Self {
        Self {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ARM_W_SIZE, ARM_H_SIZE)),
                anchor: Anchor::TopCenter,
                rect: Some(gr.right_arm),
                ..default()
            },
            texture: gr.tex.clone(),
            tag: PlayerGraphicsPart::RightArm,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., ARM_OFFSET, FRONT_ARM_Z_INDEX),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_left_arm(gr: &PlayerGraphics) -> Self {
        Self {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ARM_W_SIZE, ARM_H_SIZE)),
                anchor: Anchor::TopCenter,
                rect: Some(gr.left_arm),
                ..default()
            },
            texture: gr.tex.clone(),
            tag: PlayerGraphicsPart::LeftArm,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., ARM_OFFSET, BACK_ARM_Z_INDEX),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_right_leg(gr: &PlayerGraphics) -> Self {
        Self {
            sprite: Sprite {
                custom_size: Some(Vec2::new(LEG_W_SIZE, LEG_H_SIZE)),
                anchor: Anchor::TopCenter,
                rect: Some(gr.right_leg),
                ..default()
            },
            texture: gr.tex.clone(),
            tag: PlayerGraphicsPart::RightLeg,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., LEG_OFFSET, FRONT_LEG_Z_INDEX),
                ..default()
            },
            visibility_bunde: VisibilityBundle::default(),
        }
    }

    fn new_left_leg(gr: &PlayerGraphics) -> Self {
        Self {
            sprite: Sprite {
                custom_size: Some(Vec2::new(LEG_W_SIZE, LEG_H_SIZE)),
                anchor: Anchor::TopCenter,
                rect: Some(gr.left_leg),
                ..default()
            },
            texture: gr.tex.clone(),
            tag: PlayerGraphicsPart::LeftLeg,
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., LEG_OFFSET, BACK_LEG_Z_INDEX),
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
            collider: Collider::cuboid(10., 76.),
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
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(0., 9.5, 0.),
                ..default()
            },
            visibility_bundle: VisibilityBundle::default(),
        }
    }
}
