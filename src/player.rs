use bevy::{
    math::vec2, prelude::*, render::texture::DEFAULT_IMAGE_HANDLE, sprite::Anchor,
    window::PrimaryWindow,
};
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use std::f32::consts::PI;

use crate::{
    block::{Block, BlockBundle, BlockGraphics, BLOCK_SIZE},
    camera::MainCamera,
    inventory::{CurrentItem, Inv},
    item::{spawn_item, ItemSensor},
    item_kind::ItemKind,
    utils::{in_reach, leans_to_left, leans_to_right, map},
    world::{Chunk, ChunkPosition, PlayerChunkPosition, World},
};

// CONSTANTS

const PLAYER_Z_INDEX: f32 = 0.;

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
                    pick_up_item,
                ),
            )
            // Reflection
            .register_type::<Player>()
            .register_type::<PlayerGraphicsHolder>()
            .register_type::<PlayerGraphicsPart>()
            .register_type::<PlayerGraphicsHead>()
            .register_type::<PlayerGraphicsBody>()
            .register_type::<PlayerGraphicsRightArm>()
            .register_type::<PlayerGraphicsLeftArm>()
            .register_type::<PlayerGraphicsRightLeg>()
            .register_type::<PlayerGraphicsLeftLeg>()
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
struct PlayerGraphicsHead;
#[derive(Component, Reflect)]
struct PlayerGraphicsBody;
#[derive(Component, Reflect)]
struct PlayerGraphicsRightArm;
#[derive(Component, Reflect)]
struct PlayerGraphicsLeftArm;
#[derive(Component, Reflect)]
struct PlayerGraphicsRightLeg;
#[derive(Component, Reflect)]
struct PlayerGraphicsLeftLeg;

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
    mut head: Query<(&GlobalTransform, &mut Transform, &mut Sprite), With<PlayerGraphicsHead>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let (head_gtransform, mut head_transform, mut head_sprite) = head.single_mut();
    let head_gtransform = head_gtransform.translation();

    let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) else { return };

    let cursor_vec = vec2(
        cursor_position.x - head_gtransform.x,
        cursor_position.y - head_gtransform.y,
    );

    let theta = f32::atan2(cursor_vec.y, cursor_vec.x);

    if (theta > PI / 2. && theta < PI) || (theta < -PI / 2. && theta > -PI) {
        head_transform.rotation = Quat::from_rotation_z(theta + std::f32::consts::PI);
        head_sprite.flip_x = true;
    } else {
        head_transform.rotation = Quat::from_rotation_z(theta);
        head_sprite.flip_x = false;
    }
}

fn animate_arms(
    mut right_arm: Query<
        (&mut Transform, &mut WaveIndex),
        (With<PlayerGraphicsRightArm>, Without<PlayerGraphicsLeftArm>),
    >,
    mut left_arm: Query<
        (&mut Transform, &mut WaveIndex),
        (With<PlayerGraphicsLeftArm>, Without<PlayerGraphicsRightArm>),
    >,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let running = keys.pressed(KeyCode::ShiftLeft);

    let right_arm = right_arm.single_mut();
    let left_arm = left_arm.single_mut();

    let step = match running {
        true => 9.,
        false => 4.5,
    } * time.delta_seconds();

    let rad_map = match running {
        true => (5. * PI / 4., 7. * PI / 4.),
        false => (4. * PI / 3., 5. * PI / 3.),
    };

    let moving = keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]);

    let (mut transform, mut wave_index) = right_arm;

    handle_member_animation(
        moving,
        &mut transform,
        &mut wave_index,
        Direction::Right,
        rad_map,
        step,
    );

    let (mut transform, mut wave_index) = left_arm;

    handle_member_animation(
        moving,
        &mut transform,
        &mut wave_index,
        Direction::Left,
        rad_map,
        step,
    );
}

fn animate_legs(
    mut right_leg: Query<
        (&mut Transform, &mut WaveIndex),
        (With<PlayerGraphicsRightLeg>, Without<PlayerGraphicsLeftLeg>),
    >,
    mut left_leg: Query<
        (&mut Transform, &mut WaveIndex),
        (With<PlayerGraphicsLeftLeg>, Without<PlayerGraphicsRightLeg>),
    >,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let running = keys.pressed(KeyCode::ShiftLeft);

    let right_leg = right_leg.single_mut();
    let left_leg = left_leg.single_mut();

    let step = match running {
        true => 9.,
        false => 4.5,
    } * time.delta_seconds();

    let rad_map = match running {
        true => (5. * PI / 4., 7. * PI / 4.),
        false => (4. * PI / 3., 5. * PI / 3.),
    };

    let moving = keys.any_pressed([KeyCode::A, KeyCode::D, KeyCode::Left, KeyCode::Right]);

    let (mut transform, mut wave_index) = right_leg;

    handle_member_animation(
        moving,
        &mut transform,
        &mut wave_index,
        Direction::Left,
        rad_map,
        step,
    );

    let (mut transform, mut wave_index) = left_leg;
    handle_member_animation(
        moving,
        &mut transform,
        &mut wave_index,
        Direction::Right,
        rad_map,
        step,
    );
}

fn handle_member_animation(
    moving: bool,
    transform: &mut Transform,
    wave_index: &mut WaveIndex,
    direction: Direction,
    rad_map: (f32, f32),
    step: f32,
) {
    if moving {
        let sin = wave_index.0.sin();
        let theta = map(sin, -1., 1., rad_map.0, rad_map.1) + PI / 2.;

        transform.rotation = Quat::from_rotation_z(match direction {
            Direction::Right => theta,
            Direction::Left => -theta,
        });

        wave_index.0 += step;
        if wave_index.0 > 360. {
            wave_index.0 = 0.;
        }
    } else {
        // Put arm back in place after stopping movement
        let angle = transform.rotation.to_euler(EulerRot::ZYX).0;

        if leans_to_left(angle + 3. * PI / 2.) {
            let angle = angle - step;

            transform.rotation = Quat::from_rotation_z(angle);

            if leans_to_right(angle + 3. * PI / 2.) {
                transform.rotation = Quat::from_rotation_z(2. * PI);
                wave_index.0 = 0.;
            }
        } else if leans_to_right(angle + 3. * PI / 2.) {
            let angle = angle + step;

            transform.rotation = Quat::from_rotation_z(angle);

            if leans_to_left(angle + 3. * PI / 2.) {
                transform.rotation = Quat::from_rotation_z(2. * PI);
                wave_index.0 = 0.;
            }
        }
    }
}

fn change_direction(
    mut direction: Query<&mut Direction, With<Player>>,
    head_gtransform: Query<&GlobalTransform, With<PlayerGraphicsHead>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let head_gtransform = head_gtransform.single();

    let mut direction = direction.single_mut();

    let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) else { return };

    let cursor_vec = vec2(
        cursor_position.x - head_gtransform.translation().x,
        cursor_position.y - head_gtransform.translation().y,
    );

    let theta = f32::atan2(cursor_vec.y, cursor_vec.x);

    if (theta > PI / 2. && theta < PI) || (theta < -PI / 2. && theta > -PI) {
        *direction = Direction::Left;
    } else {
        *direction = Direction::Right;
    }
}

fn change_graphics_with_direction(
    mut graphics_parts: Query<(&mut Sprite, &mut Transform, &PlayerGraphicsPart)>,
    player_graphics: Res<PlayerGraphics>,
    direction: Query<&Direction, (With<Player>, Changed<Direction>)>,
) {
    let Ok(direction) = direction.get_single() else { return };

    match *direction {
        Direction::Right => {
            use PlayerGraphicsPart::*;
            for (mut grp_sprite, mut grp_transform, grpart) in graphics_parts.iter_mut() {
                match *grpart {
                    Body => grp_sprite.rect = Some(player_graphics.body_front),
                    RightArm => {
                        grp_sprite.rect = Some(player_graphics.right_arm_front);
                        grp_transform.translation.z = FRONT_ARM_Z_INDEX;
                    }
                    LeftArm => {
                        grp_sprite.rect = Some(player_graphics.left_arm_back);
                        grp_transform.translation.z = BACK_ARM_Z_INDEX;
                    }
                    RightLeg => {
                        grp_sprite.rect = Some(player_graphics.right_leg_front);
                        grp_transform.translation.z = FRONT_LEG_Z_INDEX;
                    }
                    LeftLeg => {
                        grp_sprite.rect = Some(player_graphics.left_leg_back);
                        grp_transform.translation.z = BACK_LEG_Z_INDEX;
                    }
                    _ => (),
                }
            }
        }
        Direction::Left => {
            use PlayerGraphicsPart::*;
            for (mut grp_sprite, mut grp_transform, grpart) in graphics_parts.iter_mut() {
                match *grpart {
                    Body => grp_sprite.rect = Some(player_graphics.body_back),
                    RightArm => {
                        grp_sprite.rect = Some(player_graphics.right_arm_back);
                        grp_transform.translation.z = BACK_ARM_Z_INDEX;
                    }
                    LeftArm => {
                        grp_sprite.rect = Some(player_graphics.left_arm_front);
                        grp_transform.translation.z = FRONT_ARM_Z_INDEX;
                    }
                    RightLeg => {
                        grp_sprite.rect = Some(player_graphics.right_leg_back);
                        grp_transform.translation.z = BACK_LEG_Z_INDEX;
                    }
                    LeftLeg => {
                        grp_sprite.rect = Some(player_graphics.left_leg_front);
                        grp_transform.translation.z = FRONT_LEG_Z_INDEX;
                    }
                    _ => (),
                }
            }
        }
    }
}

fn select_block(
    blocks: Query<(&GlobalTransform, Entity), (With<Block>, Without<BlockSelector>)>,
    player: Query<&GlobalTransform, With<Player>>,
    mut selected_block: ResMut<SelectedBlock>,
    mut last_cur_pos: ResMut<LastCursorPosition>,
    mut last_pl_pos: ResMut<LastPlayerPosition>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) else { return };

    let pos = vec2(
        (cursor_position.x / BLOCK_SIZE).round() * BLOCK_SIZE,
        (cursor_position.y / BLOCK_SIZE).round() * BLOCK_SIZE,
    );

    let player_transform = player.single().translation();
    let player_transform = vec2(player_transform.x, player_transform.y);

    if last_cur_pos.0 == pos && last_pl_pos.0 == player_transform {
        return;
    }

    last_cur_pos.0 = pos;
    last_pl_pos.0 = vec2(player_transform.x, player_transform.y);

    selected_block.0 = None;

    if !in_reach(player_transform, pos, PLAYER_REACH, BLOCK_SIZE) {
        return;
    }

    for (block_transform, block_ent) in blocks.iter() {
        if block_transform.translation().x == pos.x && block_transform.translation().y == pos.y {
            selected_block.0 = Some(block_ent);
            break;
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
    world: Query<&GlobalTransform, With<World>>,
    chunks: Query<(Entity, &ChunkPosition), With<Chunk>>,
    blocks: Query<&GlobalTransform, With<Block>>,
    player_transform: Query<&GlobalTransform, With<Player>>,
    current_chunk: Res<PlayerChunkPosition>,
    inventory: Res<Inv>,
    blocks_graphics: Res<BlockGraphics>,
    mouse: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) else { return };

    let player_transform = player_transform.single().translation();
    let player_transform = vec2(player_transform.x, player_transform.y);
    let block_pos = (cursor_position / BLOCK_SIZE).round() * BLOCK_SIZE;

    if !(mouse.just_pressed(MouseButton::Right)
        && in_reach(player_transform, block_pos, PLAYER_REACH, BLOCK_SIZE))
    {
        return;
    }

    let Some(current_inv_slot) = inventory.items[inventory.hotbar_cursor] else { return };

    if blocks
        .iter()
        .any(|&b| b.translation().x == block_pos.x && b.translation().y == block_pos.y)
    {
        return;
    }

    let world_transform = world.single().translation();

    let chunk_ent = {
        let mut ent = None;
        for (chunk_ent, chunk_pos) in chunks.iter() {
            if chunk_pos.0 == current_chunk.0 {
                ent = Some(chunk_ent);
                break;
            }
        }
        ent.unwrap()
    };

    let spawn_pos = vec2(
        block_pos.x - world_transform.x,
        block_pos.y - world_transform.y,
    );

    let block_ent = commands
        .spawn(BlockBundle::new(
            current_inv_slot.kind,
            spawn_pos,
            &blocks_graphics,
        ))
        .id();

    commands.entity(chunk_ent).add_child(block_ent);
}

fn break_block(
    mut commands: Commands,
    blocks: Query<(&GlobalTransform, Entity, &ItemKind), With<Block>>,
    player_transform: Query<&GlobalTransform, With<Player>>,
    mouse: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    blocks_graphics: Res<BlockGraphics>,
) {
    let window = window.single();
    let (camera, camera_transform) = camera.single();

    let player_transform = player_transform.single().translation();

    let Some(cursor_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) else { return };

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let block_pos = (cursor_position / BLOCK_SIZE).round() * BLOCK_SIZE;

    for (block_transform, block_ent, block_kind) in blocks.iter() {
        let block_transform = block_transform.translation();

        if block_transform.x == block_pos.x
            && block_transform.y == block_pos.y
            && in_reach(
                vec2(player_transform.x, player_transform.y),
                block_pos,
                PLAYER_REACH,
                BLOCK_SIZE,
            )
        {
            let translation = vec2(block_transform.x, block_transform.y);
            let ext_impulse = ExternalImpulse {
                impulse: vec2(0., 50.),
                ..default()
            };

            spawn_item(
                &mut commands,
                *block_kind,
                translation,
                ext_impulse,
                &blocks_graphics,
            );
            commands.entity(block_ent).despawn_recursive();
            return;
        }
    }
}

fn pick_up_item(
    mut commands: Commands,
    player_ent: Query<Entity, With<Player>>,
    items_ent: Query<(Entity, &Parent), With<ItemSensor>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let player_ent = player_ent.single();
    // let items_ent = items_ent.iter().map(|item| (item.0, item.1.get()));

    for collision_event in collision_events.iter() {
        let CollisionEvent::Started(ent0, ent1, CollisionEventFlags::SENSOR) = collision_event else { return };
        if !(player_ent == *ent0) {
            return;
        }

        for item_ent in items_ent.iter() {
            if item_ent.0 != *ent1 {
                continue;
            }

            commands.entity(item_ent.1.get()).despawn_recursive();
            println!("Received collision event: {:?} ! {:?}", ent0, ent1);
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
    fn new_head(gr: &PlayerGraphics) -> (Self, PlayerGraphicsHead) {
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
            PlayerGraphicsHead,
        )
    }

    fn new_body(gr: &PlayerGraphics) -> (Self, PlayerGraphicsBody) {
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
            PlayerGraphicsBody,
        )
    }

    fn new_right_arm(gr: &PlayerGraphics) -> (Self, PlayerGraphicsRightArm, WaveIndex) {
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
            PlayerGraphicsRightArm,
            WaveIndex(0.),
        )
    }

    fn new_left_arm(gr: &PlayerGraphics) -> (Self, PlayerGraphicsLeftArm, WaveIndex) {
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
            PlayerGraphicsLeftArm,
            WaveIndex(0.),
        )
    }

    fn new_right_leg(gr: &PlayerGraphics) -> (Self, PlayerGraphicsRightLeg, WaveIndex) {
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
            PlayerGraphicsRightLeg,
            WaveIndex(0.),
        )
    }

    fn new_left_leg(gr: &PlayerGraphics) -> (Self, PlayerGraphicsLeftLeg, WaveIndex) {
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
            PlayerGraphicsLeftLeg,
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
            collider: Collider::capsule_y(60., 8.),
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
                transform: Transform::from_xyz(BLOCK_SIZE, 60. * BLOCK_SIZE, PLAYER_Z_INDEX),
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
