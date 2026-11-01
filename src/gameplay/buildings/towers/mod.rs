pub mod crossbow;

use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::RigidBody,
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::prelude::*;

use crate::{
    GameState,
    gameplay::{
        buildings::{base_building, towers::crossbow::perform_crossbow_attack},
        collisions::GameLayer::Enemy,
    },
};

pub fn towers_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        perform_crossbow_attack.run_if(in_state(GameState::Playing)),
    );
}

#[derive(Debug, Default, Clone, SceneComponent)]
pub struct Tower {}

impl Tower {
    fn scene() -> impl Scene {
        bsn! {
            #Tower
            RigidBody::Static
            @base_building()
        }
    }
}

pub fn find_all_targets_in_radius(
    origin: Entity,
    radius: f32,
    position: Vec2,
    enemies: bool,
    spatial_query: &SpatialQuery,
) -> Vec<Entity> {
    let mut filter = SpatialQueryFilter::from_excluded_entities([origin]);
    if enemies {
        filter = filter.with_mask([Enemy]);
    }
    spatial_query.shape_intersections(&Collider::circle(radius), position, Rot2::IDENTITY, &filter)
}
