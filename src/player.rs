use bevy::{
    math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

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

const HEAD_Z_INDEX: f32 = 2.;
const BODY_Z_INDEX: f32 = 1.;
const FRONT_ARM_Z_INDEX: f32 = 3.;
const BACK_ARM_Z_INDEX: f32 = 0.;
const FRONT_LEG_Z_INDEX: f32 = 3.;
const BACK_LEG_Z_INDEX: f32 = 0.;

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
            .add_systems(
                Update,
                (
                    player_controller_movement,
                    animate_head,
                    animate_arms,
                    animate_legs,
                ),
            )
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
    RightArm(f32),
    LeftArm(f32),
    RightLeg(f32),
    LeftLeg(f32),
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
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;

        let move_delta_x = x_axis as f32;

        rb_vel.linvel.x = move_delta_x * speed.0;
    }
}

fn animate_head(
    mut graphics_parts: Query<(
        &GlobalTransform,
        &mut Transform,
        &mut Sprite,
        &PlayerGraphicsPart,
    )>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let mut cursor_position = vec2(0., 0.);

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_position = world_position
    }

    let mut head = None;
    for p in graphics_parts.iter_mut() {
        if let PlayerGraphicsPart::Head = p.3 {
            head = Some((p.0, p.1, p.2));
            break;
        }
    }

    if let Some((head_gtr, mut head_tr, mut head_sprite)) = head {
        let cursor_vec = vec2(
            cursor_position.x - head_gtr.translation().x,
            cursor_position.y - head_gtr.translation().y,
        );

        let theta = f32::atan2(cursor_vec.y, cursor_vec.x);

        if (theta > PI / 2. && theta < PI) || (theta < -PI / 2. && theta > -PI) {
            head_tr.rotation = Quat::from_rotation_z(theta + std::f32::consts::PI);
            head_sprite.flip_x = true;
        } else {
            head_tr.rotation = Quat::from_rotation_z(theta);
            head_sprite.flip_x = false;
        }
    }
}

fn animate_arms(
    mut graphics_parts: Query<(&mut Transform, &mut Sprite, &mut PlayerGraphicsPart)>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    // Get graphics parts
    let mut right_arm = None;
    let mut left_arm = None;
    for p in graphics_parts.iter_mut() {
        match *p.2 {
            PlayerGraphicsPart::RightArm(_) => {
                right_arm = Some((p.0, p.1, p.2));
                continue;
            }
            PlayerGraphicsPart::LeftArm(_) => {
                left_arm = Some((p.0, p.1, p.2));
                continue;
            }
            _ => (),
        }
    }

    // Extract right arm
    if let Some((mut right_arm_tr, mut _right_arm_sprite, mut right_arm_grpart)) = right_arm {
        match *right_arm_grpart {
            PlayerGraphicsPart::RightArm(ref mut wave_index) => {
                // Handle animation
                let step = 4.5 * time.delta_seconds();
                if keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]) {
                    let sin = wave_index.sin();
                    let theta =
                        (sin - (-1.)) / (1. - (-1.)) * (5. * PI / 3. - 4. * PI / 3.) + 4. * PI / 3.;

                    right_arm_tr.rotation = Quat::from_rotation_z(theta + PI / 2.);

                    *wave_index += step;
                    if *wave_index > 360. {
                        *wave_index = 0.;
                    }
                } else {
                    let angle = right_arm_tr.rotation.to_euler(EulerRot::ZYX).0;

                    if angle + 3. * PI / 2. < 5. * PI / 3. && angle + 3. * PI / 2. > 3. * PI / 2. {
                        // condition 1
                        let angle = angle - step;

                        right_arm_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. > 4. * PI / 3.
                            && angle + 3. * PI / 2. < 3. * PI / 2.
                        // condition 2
                        {
                            right_arm_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    } else if angle + 3. * PI / 2. > 4. * PI / 3.
                        && angle + 3. * PI / 2. < 3. * PI / 2.
                    // condition 2
                    {
                        let angle = angle + step;

                        right_arm_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. < 5. * PI / 3.
                            && angle + 3. * PI / 2. > 3. * PI / 2.
                        // condition 1
                        {
                            right_arm_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    // Extract left arm
    if let Some((mut left_arm_tr, mut _left_arm_sprite, mut left_arm_grpart)) = left_arm {
        match *left_arm_grpart {
            PlayerGraphicsPart::LeftArm(ref mut wave_index) => {
                let step = 4.5 * time.delta_seconds();
                if keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]) {
                    // Handle animation
                    let sin = wave_index.sin();
                    let theta =
                        (sin - (-1.)) / (1. - (-1.)) * (5. * PI / 3. - 4. * PI / 3.) + 4. * PI / 3.;

                    left_arm_tr.rotation = Quat::from_rotation_z(-(theta + PI / 2.));

                    *wave_index += 4.5 * time.delta_seconds();
                    if *wave_index > 360. {
                        *wave_index = 0.;
                    }
                } else {
                    let angle = left_arm_tr.rotation.to_euler(EulerRot::ZYX).0;

                    if angle + 3. * PI / 2. < 5. * PI / 3. && angle + 3. * PI / 2. > 3. * PI / 2. {
                        // condition 1
                        let angle = angle - step;

                        left_arm_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. > 4. * PI / 3.
                            && angle + 3. * PI / 2. < 3. * PI / 2.
                        // condition 2
                        {
                            left_arm_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    } else if angle + 3. * PI / 2. > 4. * PI / 3.
                        && angle + 3. * PI / 2. < 3. * PI / 2.
                    // condition 2
                    {
                        let angle = angle + step;

                        left_arm_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. < 5. * PI / 3.
                            && angle + 3. * PI / 2. > 3. * PI / 2.
                        // condition 1
                        {
                            left_arm_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn animate_legs(
    mut graphics_parts: Query<(&mut Transform, &mut Sprite, &mut PlayerGraphicsPart)>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    // Get graphics parts
    let mut right_leg = None;
    let mut left_leg = None;
    for p in graphics_parts.iter_mut() {
        match *p.2 {
            PlayerGraphicsPart::RightLeg(_) => {
                right_leg = Some((p.0, p.1, p.2));
                continue;
            }
            PlayerGraphicsPart::LeftLeg(_) => {
                left_leg = Some((p.0, p.1, p.2));
                continue;
            }
            _ => (),
        }
    }

    // Extract right leg
    if let Some((mut right_leg_tr, mut _right_leg_sprite, mut right_leg_grpart)) = right_leg {
        match *right_leg_grpart {
            PlayerGraphicsPart::RightLeg(ref mut wave_index) => {
                // Handle animation
                let step = 4.5 * time.delta_seconds();
                if keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]) {
                    let sin = wave_index.sin();
                    let theta =
                        (sin - (-1.)) / (1. - (-1.)) * (5. * PI / 3. - 4. * PI / 3.) + 4. * PI / 3.;

                    right_leg_tr.rotation = Quat::from_rotation_z(-(theta + PI / 2.));

                    *wave_index += step;
                    if *wave_index > 360. {
                        *wave_index = 0.;
                    }
                } else {
                    let angle = right_leg_tr.rotation.to_euler(EulerRot::ZYX).0;

                    if angle + 3. * PI / 2. < 5. * PI / 3. && angle + 3. * PI / 2. > 3. * PI / 2. {
                        // condition 1
                        let angle = angle - step;

                        right_leg_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. > 4. * PI / 3.
                            && angle + 3. * PI / 2. < 3. * PI / 2.
                        // condition 2
                        {
                            right_leg_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    } else if angle + 3. * PI / 2. > 4. * PI / 3.
                        && angle + 3. * PI / 2. < 3. * PI / 2.
                    // condition 2
                    {
                        let angle = angle + step;

                        right_leg_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. < 5. * PI / 3.
                            && angle + 3. * PI / 2. > 3. * PI / 2.
                        // condition 1
                        {
                            right_leg_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    // Extract left leg
    if let Some((mut left_leg_tr, mut _left_leg_sprite, mut left_leg_grpart)) = left_leg {
        match *left_leg_grpart {
            PlayerGraphicsPart::LeftLeg(ref mut wave_index) => {
                // Handle animation
                let step = 4.5 * time.delta_seconds();
                if keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]) {
                    let sin = wave_index.sin();
                    let theta =
                        (sin - (-1.)) / (1. - (-1.)) * (5. * PI / 3. - 4. * PI / 3.) + 4. * PI / 3.;

                    left_leg_tr.rotation = Quat::from_rotation_z(theta + PI / 2.);

                    *wave_index += step;
                    if *wave_index > 360. {
                        *wave_index = 0.;
                    }
                } else {
                    let angle = left_leg_tr.rotation.to_euler(EulerRot::ZYX).0;

                    if angle + 3. * PI / 2. < 5. * PI / 3. && angle + 3. * PI / 2. > 3. * PI / 2. {
                        // condition 1
                        let angle = angle - step;

                        left_leg_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. > 4. * PI / 3.
                            && angle + 3. * PI / 2. < 3. * PI / 2.
                        // condition 2
                        {
                            left_leg_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    } else if angle + 3. * PI / 2. > 4. * PI / 3.
                        && angle + 3. * PI / 2. < 3. * PI / 2.
                    // condition 2
                    {
                        let angle = angle + step;

                        left_leg_tr.rotation = Quat::from_rotation_z(angle);

                        if angle + 3. * PI / 2. < 5. * PI / 3.
                            && angle + 3. * PI / 2. > 3. * PI / 2.
                        // condition 1
                        {
                            left_leg_tr.rotation = Quat::from_rotation_z(2. * PI);
                            *wave_index = 0.;
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

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
            tag: PlayerGraphicsPart::RightArm(0.),
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
            tag: PlayerGraphicsPart::LeftArm(0.),
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
            tag: PlayerGraphicsPart::RightLeg(0.),
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
            tag: PlayerGraphicsPart::LeftLeg(0.),
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
