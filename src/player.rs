use bevy::{
    math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use crate::{
    block::{self, Block, BlockBundle, BlockGraphics, BlockKind, BLOCK_SIZE},
    camera::MainCamera,
    inventory::CurrentItem,
    utils::{in_reach, leans_to_left, leans_to_right, map},
    world::World,
};

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

const PLAYER_REACH: f32 = 3.;

const PLAYER_Z_INDEX: f32 = 0.;

// PLUGINS

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(PlayerGraphics::default())
            .insert_resource(SelectedBlock::default())
            .insert_resource(LastCursorPosition::default())
            .insert_resource(LastPlayerPosition::default())
            // Systems
            .add_systems(PreStartup, load_player_graphics)
            .add_systems(Startup, (spawn_player, spawn_block_selector))
            .add_systems(FixedUpdate, player_controller_movement)
            .add_systems(
                Update,
                (
                    animate_head,
                    animate_arms,
                    animate_legs,
                    change_direction,
                    change_graphics_with_direction,
                    select_block,
                    highlight_selected_block,
                    place_block,
                    break_block,
                ),
            )
            // Reflection
            .register_type::<Player>()
            .register_type::<PlayerGraphicsHolder>()
            .register_type::<PlayerGraphicsPart>()
            .register_type::<PlayerGraphicsPartHead>()
            .register_type::<PlayerGraphicsPartBody>()
            .register_type::<PlayerGraphicsPartRightArm>()
            .register_type::<PlayerGraphicsPartLeftArm>()
            .register_type::<PlayerGraphicsPartRightLeg>()
            .register_type::<PlayerGraphicsPartLeftLeg>()
            .register_type::<Speed>()
            .register_type::<Jump>()
            .register_type::<Direction>()
            .register_type::<WaveIndex>()
            .register_type::<BlockSelector>();
    }
}

// RESOURCES

#[derive(Resource)]
struct PlayerGraphics {
    tex: Handle<Image>,
    head: Rect,
    body_front: Rect,
    body_back: Rect,
    right_arm_front: Rect,
    right_arm_back: Rect,
    left_arm_front: Rect,
    left_arm_back: Rect,
    right_leg_front: Rect,
    right_leg_back: Rect,
    left_leg_front: Rect,
    left_leg_back: Rect,
}

#[derive(Resource, Default)]
struct SelectedBlock(Option<Entity>);

#[derive(Resource, Default)]
struct LastCursorPosition(Vec2);

#[derive(Resource, Default)]
struct LastPlayerPosition(Vec2);

impl Default for PlayerGraphics {
    fn default() -> Self {
        Self {
            tex: DEFAULT_IMAGE_HANDLE.typed(),
            head: Rect::new(0., 0., 8., 8.),
            body_front: Rect::new(8., 0., 12., 12.),
            body_back: Rect::new(8., 12., 12., 24.),
            right_arm_front: Rect::new(12., 0., 16., 12.),
            right_arm_back: Rect::new(12., 12., 16., 24.),
            left_arm_front: Rect::new(16., 0., 20., 12.),
            left_arm_back: Rect::new(16., 12., 20., 23.),
            right_leg_front: Rect::new(20., 0., 24., 12.),
            right_leg_back: Rect::new(20., 12., 24., 24.),
            left_leg_front: Rect::new(24., 0., 28., 12.),
            left_leg_back: Rect::new(24., 12., 28., 24.),
        }
    }
}

// COMPONENTS

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
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
struct PlayerGraphicsPartHead;
#[derive(Component, Reflect)]
struct PlayerGraphicsPartBody;
#[derive(Component, Reflect)]
struct PlayerGraphicsPartRightArm;
#[derive(Component, Reflect)]
struct PlayerGraphicsPartLeftArm;
#[derive(Component, Reflect)]
struct PlayerGraphicsPartRightLeg;
#[derive(Component, Reflect)]
struct PlayerGraphicsPartLeftLeg;

/// [`0`] is the walking speed
/// [`1`] is the runnign speed
#[derive(Component, Reflect)]
struct Speed(f32, f32);

#[derive(Component, Reflect)]
struct Jump(f32);

#[derive(Component, Default, Reflect)]
enum Direction {
    #[default]
    Right,
    Left,
}

#[derive(Component, Reflect)]
struct WaveIndex(f32);

#[derive(Component, Reflect)]
struct BlockSelector;

// SYSTEMS

fn load_player_graphics(
    asset_server: Res<AssetServer>,
    mut player_graphics: ResMut<PlayerGraphics>,
) {
    *player_graphics = PlayerGraphics {
        tex: asset_server.load("player.png"),
        ..default()
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

fn spawn_block_selector(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                ..default()
            },
            texture: asset_server.load("block_selector.png"),
            visibility: Visibility::Hidden,
            ..default()
        },
        BlockSelector,
        Name::new("Block Selector"),
    ));
}

fn player_controller_movement(
    mut player_query: Query<(&Speed, &mut Velocity, &GlobalTransform)>,
    keys: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
) {
    for (speed, mut rb_vel, gtr) in player_query.iter_mut() {
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);
        let jump = keys.just_pressed(KeyCode::Space);
        let running = keys.pressed(KeyCode::ShiftLeft);

        let x_axis = -(left as i8) + right as i8;
        let move_delta_x = x_axis as f32;
        let sp = if running { speed.1 } else { speed.0 };

        rb_vel.linvel.x = move_delta_x * sp;

        if jump {
            let ray_pos = vec2(gtr.translation().x, gtr.translation().y - 79.);
            let ray_dir = Vec2::new(0., -20.);
            let max_toi = 1.;
            let solid = true;
            let filter = QueryFilter::default();

            if let Some(_) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
                rb_vel.linvel.y = 400.;
            }
        }
    }
}

fn animate_head(
    mut head: Query<(&GlobalTransform, &mut Transform, &mut Sprite), With<PlayerGraphicsPartHead>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let (head_gtr, mut head_tr, mut head_sprite) = head.single_mut();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
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
    mut right_arm: Query<
        (&mut Transform, &mut WaveIndex),
        (
            With<PlayerGraphicsPartRightArm>,
            Without<PlayerGraphicsPartLeftArm>,
        ),
    >,
    mut left_arm: Query<
        (&mut Transform, &mut WaveIndex),
        (
            With<PlayerGraphicsPartLeftArm>,
            Without<PlayerGraphicsPartRightArm>,
        ),
    >,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let running = keys.pressed(KeyCode::ShiftLeft);

    let right_arm = right_arm.single_mut();
    let left_arm = left_arm.single_mut();

    let step = if running { 9. } else { 4.5 } * time.delta_seconds();

    let rad_map = if running {
        (5. * PI / 4., 7. * PI / 4.)
    } else {
        (4. * PI / 3., 5. * PI / 3.)
    };

    let moving = keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]);

    let (mut tr, mut wave_index) = right_arm;

    handle_member_animation(
        moving,
        &mut tr,
        &mut wave_index,
        Direction::Right,
        rad_map,
        step,
    );

    let (mut tr, mut wave_index) = left_arm;

    handle_member_animation(
        moving,
        &mut tr,
        &mut wave_index,
        Direction::Left,
        rad_map,
        step,
    );
}

fn animate_legs(
    mut right_leg: Query<
        (&mut Transform, &mut WaveIndex),
        (
            With<PlayerGraphicsPartRightLeg>,
            Without<PlayerGraphicsPartLeftLeg>,
        ),
    >,
    mut left_leg: Query<
        (&mut Transform, &mut WaveIndex),
        (
            With<PlayerGraphicsPartLeftLeg>,
            Without<PlayerGraphicsPartRightLeg>,
        ),
    >,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let running = keys.pressed(KeyCode::ShiftLeft);

    let right_leg = right_leg.single_mut();
    let left_leg = left_leg.single_mut();

    let step = if keys.pressed(KeyCode::ShiftLeft) {
        9.
    } else {
        4.5
    } * time.delta_seconds();

    let rad_map = if running {
        (5. * PI / 4., 7. * PI / 4.)
    } else {
        (4. * PI / 3., 5. * PI / 3.)
    };

    let moving = keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]);

    let (mut tr, mut wave_index) = right_leg;

    handle_member_animation(
        moving,
        &mut tr,
        &mut wave_index,
        Direction::Left,
        rad_map,
        step,
    );

    let (mut tr, mut wave_index) = left_leg;
    handle_member_animation(
        moving,
        &mut tr,
        &mut wave_index,
        Direction::Right,
        rad_map,
        step,
    );
}

fn handle_member_animation(
    moving: bool,
    tr: &mut Transform,
    wave_index: &mut WaveIndex,
    direction: Direction,
    rad_map: (f32, f32),
    step: f32,
) {
    if moving {
        let sin = wave_index.0.sin();
        let theta = map(sin, -1., 1., rad_map.0, rad_map.1) + PI / 2.;

        tr.rotation = Quat::from_rotation_z(match direction {
            Direction::Right => theta,
            Direction::Left => -theta,
        });

        wave_index.0 += step;
        if wave_index.0 > 360. {
            wave_index.0 = 0.;
        }
    } else {
        // Put arm back in place after stopping movement
        let angle = tr.rotation.to_euler(EulerRot::ZYX).0;

        if leans_to_left(angle + 3. * PI / 2.) {
            let angle = angle - step;

            tr.rotation = Quat::from_rotation_z(angle);

            if leans_to_right(angle + 3. * PI / 2.) {
                tr.rotation = Quat::from_rotation_z(2. * PI);
                wave_index.0 = 0.;
            }
        } else if leans_to_right(angle + 3. * PI / 2.) {
            let angle = angle + step;

            tr.rotation = Quat::from_rotation_z(angle);

            if leans_to_left(angle + 3. * PI / 2.) {
                tr.rotation = Quat::from_rotation_z(2. * PI);
                wave_index.0 = 0.;
            }
        }
    }
}

fn change_direction(
    mut direction: Query<&mut Direction, With<Player>>,
    head_gtr: Query<&GlobalTransform, With<PlayerGraphicsPartHead>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let head_gtr = head_gtr.single();

    let mut direction = direction.single_mut();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let cursor_vec = vec2(
            cursor_position.x - head_gtr.translation().x,
            cursor_position.y - head_gtr.translation().y,
        );

        let theta = f32::atan2(cursor_vec.y, cursor_vec.x);

        if (theta > PI / 2. && theta < PI) || (theta < -PI / 2. && theta > -PI) {
            *direction = Direction::Left;
        } else {
            *direction = Direction::Right;
        }
    }
}

fn change_graphics_with_direction(
    mut graphics_parts: Query<(&mut Sprite, &mut Transform, &PlayerGraphicsPart)>,
    player_graphics: Res<PlayerGraphics>,
    direction: Query<&Direction, (With<Player>, Changed<Direction>)>,
) {
    let direction = direction.get_single();
    let direction = match direction {
        Ok(e) => e,
        Err(_) => return,
    };
    match *direction {
        Direction::Right => {
            use PlayerGraphicsPart::*;
            for (mut grp_sprite, mut grp_tr, grp) in graphics_parts.iter_mut() {
                match *grp {
                    Body => grp_sprite.rect = Some(player_graphics.body_front),
                    RightArm => {
                        grp_sprite.rect = Some(player_graphics.right_arm_front);
                        grp_tr.translation.z = FRONT_ARM_Z_INDEX;
                    }
                    LeftArm => {
                        grp_sprite.rect = Some(player_graphics.left_arm_back);
                        grp_tr.translation.z = BACK_ARM_Z_INDEX;
                    }
                    RightLeg => {
                        grp_sprite.rect = Some(player_graphics.right_leg_front);
                        grp_tr.translation.z = FRONT_LEG_Z_INDEX;
                    }
                    LeftLeg => {
                        grp_sprite.rect = Some(player_graphics.left_leg_back);
                        grp_tr.translation.z = BACK_LEG_Z_INDEX;
                    }
                    _ => (),
                }
            }
        }
        Direction::Left => {
            use PlayerGraphicsPart::*;
            for (mut grp_sprite, mut grp_tr, grp) in graphics_parts.iter_mut() {
                match *grp {
                    Body => grp_sprite.rect = Some(player_graphics.body_back),
                    RightArm => {
                        grp_sprite.rect = Some(player_graphics.right_arm_back);
                        grp_tr.translation.z = BACK_ARM_Z_INDEX;
                    }
                    LeftArm => {
                        grp_sprite.rect = Some(player_graphics.left_arm_front);
                        grp_tr.translation.z = FRONT_ARM_Z_INDEX;
                    }
                    RightLeg => {
                        grp_sprite.rect = Some(player_graphics.right_leg_back);
                        grp_tr.translation.z = BACK_LEG_Z_INDEX;
                    }
                    LeftLeg => {
                        grp_sprite.rect = Some(player_graphics.left_leg_front);
                        grp_tr.translation.z = FRONT_LEG_Z_INDEX;
                    }
                    _ => (),
                }
            }
        }
    }
}

fn select_block(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut selected_block: ResMut<SelectedBlock>,
    mut last_cur_pos: ResMut<LastCursorPosition>,
    mut last_pl_pos: ResMut<LastPlayerPosition>,
    blocks: Query<(&GlobalTransform, Entity), (With<Block>, Without<BlockSelector>)>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let pos = vec2(
            (cursor_position.x / BLOCK_SIZE).round() * BLOCK_SIZE,
            (cursor_position.y / BLOCK_SIZE).round() * BLOCK_SIZE,
        );

        let player_tr = player.single().translation();
        let player_tr = vec2(player_tr.x, player_tr.y);

        if last_cur_pos.0 == pos && last_pl_pos.0 == player_tr {
            return;
        }

        last_cur_pos.0 = pos;
        last_pl_pos.0 = vec2(player_tr.x, player_tr.y);

        selected_block.0 = None;
        if in_reach(player_tr, pos, PLAYER_REACH, BLOCK_SIZE) {
            for (btr, bent) in blocks.iter() {
                if btr.translation().x == pos.x && btr.translation().y == pos.y {
                    selected_block.0 = Some(bent);
                    break;
                }
            }
        }
    }
}

fn highlight_selected_block(
    last_cur_pos: Res<LastCursorPosition>,
    selected_block: Res<SelectedBlock>,
    mut block_selector: Query<
        (&mut Transform, &mut Visibility),
        (With<BlockSelector>, Without<Block>),
    >,
) {
    let mut block_selector = block_selector.single_mut();

    match selected_block.0 {
        Some(_) => {
            block_selector.0.translation = last_cur_pos.0.extend(10.);
            *block_selector.1 = Visibility::Visible;
        }
        None => *block_selector.1 = Visibility::Hidden,
    }
}

fn place_block(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    blocks_graphics: Res<BlockGraphics>,
    world: Query<Entity, With<World>>,
    blocks: Query<&GlobalTransform, With<Block>>,
    player_tr: Query<&GlobalTransform, With<Player>>,
    mouse: Res<Input<MouseButton>>,
    current_item: Res<CurrentItem>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let player_tr = player_tr.single().translation();
        let player_tr = vec2(player_tr.x, player_tr.y);
        let spawn_pos = (cursor_position / BLOCK_SIZE).round() * BLOCK_SIZE;

        if mouse.just_pressed(MouseButton::Right)
            && in_reach(player_tr, spawn_pos, PLAYER_REACH, BLOCK_SIZE)
        {
            if blocks
                .iter()
                .any(|&b| b.translation().x == spawn_pos.x && b.translation().y == spawn_pos.y)
            {
                return;
            }

            let chent = commands
                .spawn(BlockBundle::new(
                    current_item.0,
                    spawn_pos,
                    &blocks_graphics,
                ))
                .id();
            commands.entity(world.single()).add_child(chent);
        }
    }
}

fn break_block(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    blocks: Query<(&GlobalTransform, Entity), With<Block>>,
    player_tr: Query<&GlobalTransform, With<Player>>,
    mouse: Res<Input<MouseButton>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let player_tr = player_tr.single().translation();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        if mouse.just_pressed(MouseButton::Left) {
            let block_pos = (cursor_position / BLOCK_SIZE).round() * BLOCK_SIZE;

            for (btr, bent) in blocks.iter() {
                if btr.translation().x == block_pos.x
                    && btr.translation().y == block_pos.y
                    && in_reach(
                        vec2(player_tr.x, player_tr.y),
                        block_pos,
                        PLAYER_REACH,
                        BLOCK_SIZE,
                    )
                {
                    commands.entity(bent).despawn_recursive();
                    return;
                }
            }
        }
    }
}

// BUNDLES

#[derive(Bundle)]
struct PlayerBundle {
    // gameplay settings
    speed: Speed,
    jump: Jump,
    direction: Direction,

    // colliders
    collider: Collider,
    collider_mass: ColliderMassProperties,

    // tags
    player: Player,

    // physics required
    rigid_body: RigidBody,
    friction: Friction,
    velocity: Velocity,
    ext_impulse: ExternalImpulse,
    locked_axes: LockedAxes,

    // required
    spatial_bundle: SpatialBundle,
}

#[derive(Bundle)]
struct PlayerGraphicsHolderBundle {
    // tags
    tag: PlayerGraphicsHolder,

    // required
    spatial_bundle: SpatialBundle,
}

/// Need to insert tags manually (as well as WaveIndex if needed)
#[derive(Bundle)]
struct PlayerGraphicsPartBundle {
    // rendering
    sprite: Sprite,
    texture: Handle<Image>,

    // tags
    tag: PlayerGraphicsPart,

    // required
    spatial_bundle: SpatialBundle,
}

impl PlayerBundle {
    /// Need to insert tags manually (as well as WaveIndex if needed)
    fn new(
        walking_speed: f32,
        runnign_speed: f32,
        jump_force: f32,
        collider: Collider,
        mass: f32,
    ) -> Self {
        Self {
            speed: Speed(walking_speed, runnign_speed),
            jump: Jump(jump_force),
            direction: default(),
            collider,
            collider_mass: ColliderMassProperties::Mass(mass),
            player: Player,
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            velocity: default(),
            ext_impulse: default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            spatial_bundle: default(),
        }
    }
}

impl PlayerGraphicsPartBundle {
    fn new_head(gr: &PlayerGraphics) -> (Self, PlayerGraphicsPartHead) {
        (
            Self {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(HEAD_SIZE)),
                    anchor: Anchor::BottomCenter,
                    rect: Some(gr.head),
                    ..default()
                },
                texture: gr.tex.clone(),
                tag: PlayerGraphicsPart::Head,
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(0., HEAD_OFFSET, HEAD_Z_INDEX),
                    ..default()
                },
            },
            PlayerGraphicsPartHead,
        )
    }

    fn new_body(gr: &PlayerGraphics) -> (Self, PlayerGraphicsPartBody) {
        (
            Self {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BODY_W_SIZE, BODY_H_SIZE)),
                    anchor: Anchor::Center,
                    rect: Some(gr.body_front),
                    ..default()
                },
                texture: gr.tex.clone(),
                tag: PlayerGraphicsPart::Body,
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(0., 0., BODY_Z_INDEX),
                    ..default()
                },
            },
            PlayerGraphicsPartBody,
        )
    }

    fn new_right_arm(gr: &PlayerGraphics) -> (Self, PlayerGraphicsPartRightArm, WaveIndex) {
        (
            Self {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ARM_W_SIZE, ARM_H_SIZE)),
                    anchor: Anchor::TopCenter,
                    rect: Some(gr.right_arm_front),
                    ..default()
                },
                texture: gr.tex.clone(),
                tag: PlayerGraphicsPart::RightArm,
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(0., ARM_OFFSET, FRONT_ARM_Z_INDEX),
                    ..default()
                },
            },
            PlayerGraphicsPartRightArm,
            WaveIndex(0.),
        )
    }

    fn new_left_arm(gr: &PlayerGraphics) -> (Self, PlayerGraphicsPartLeftArm, WaveIndex) {
        (
            Self {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ARM_W_SIZE, ARM_H_SIZE)),
                    anchor: Anchor::TopCenter,
                    rect: Some(gr.left_arm_front),
                    ..default()
                },
                texture: gr.tex.clone(),
                tag: PlayerGraphicsPart::LeftArm,
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(0., ARM_OFFSET, BACK_ARM_Z_INDEX),
                    ..default()
                },
            },
            PlayerGraphicsPartLeftArm,
            WaveIndex(0.),
        )
    }

    fn new_right_leg(gr: &PlayerGraphics) -> (Self, PlayerGraphicsPartRightLeg, WaveIndex) {
        (
            Self {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(LEG_W_SIZE, LEG_H_SIZE)),
                    anchor: Anchor::TopCenter,
                    rect: Some(gr.right_leg_front),
                    ..default()
                },
                texture: gr.tex.clone(),
                tag: PlayerGraphicsPart::RightLeg,
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(0., LEG_OFFSET, FRONT_LEG_Z_INDEX),
                    ..default()
                },
            },
            PlayerGraphicsPartRightLeg,
            WaveIndex(0.),
        )
    }

    fn new_left_leg(gr: &PlayerGraphics) -> (Self, PlayerGraphicsPartLeftLeg, WaveIndex) {
        (
            Self {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(LEG_W_SIZE, LEG_H_SIZE)),
                    anchor: Anchor::TopCenter,
                    rect: Some(gr.left_leg_front),
                    ..default()
                },
                texture: gr.tex.clone(),
                tag: PlayerGraphicsPart::LeftLeg,
                spatial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(0., LEG_OFFSET, BACK_LEG_Z_INDEX),
                    ..default()
                },
            },
            PlayerGraphicsPartLeftLeg,
            WaveIndex(0.),
        )
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            speed: Speed(300., 500.),
            jump: Jump(100.),
            direction: default(),
            collider: Collider::capsule_y(70., 8.),
            // collider: Collider::cuboid(10., 76.),
            // collider: Collider::round_cuboid(10., 76., 0.03),
            collider_mass: ColliderMassProperties::Mass(91.),
            player: Player,
            rigid_body: RigidBody::Dynamic,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            velocity: default(),
            ext_impulse: default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(BLOCK_SIZE, 0., PLAYER_Z_INDEX),
                ..default()
            },
        }
    }
}

impl Default for PlayerGraphicsHolderBundle {
    fn default() -> Self {
        Self {
            tag: PlayerGraphicsHolder,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_xyz(0., 9.5, 0.),
                ..default()
            },
        }
    }
}
