use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::block::{BlockGraphics, BlockKind};

// CONSTANTS

const ITEM_SIZE: f32 = 30.;
const ITEM_COLLIDER_SIZE: f32 = 15.;
const ITEM_SENSOR_SIZE: f32 = 30.;

// COMPONENTS

#[derive(Component)]
struct Item;

#[derive(Component)]
pub struct ItemSensor;

// FUNCTIONS

pub fn spawn_item(
    commands: &mut Commands,
    kind: BlockKind,
    translation: Vec2,
    ext_impulse: ExternalImpulse,
    block_graphics: &Res<BlockGraphics>,
) {
    commands
        .spawn((
            Item,
            Name::new("Item"),
            Collider::cuboid(ITEM_COLLIDER_SIZE, ITEM_COLLIDER_SIZE),
            RigidBody::Dynamic,
            Friction {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Min,
            },
            Velocity::default(),
            ext_impulse,
            LockedAxes::ROTATION_LOCKED,
            SpatialBundle {
                transform: Transform::from_xyz(translation.x, translation.y, 0.),
                ..default()
            },
        ))
        .with_children(|cb| {
            cb.spawn((
                block_graphics.atlas_handle.clone(),
                TextureAtlasSprite {
                    index: kind.to_index(),
                    custom_size: Some(Vec2::splat(ITEM_SIZE)),
                    ..default()
                },
                SpatialBundle::default(),
            ));

            cb.spawn((
                ItemSensor,
                Sensor,
                Collider::cuboid(ITEM_SENSOR_SIZE, ITEM_SENSOR_SIZE),
                ActiveEvents::COLLISION_EVENTS,
            ));
        });
}
