use avian2d::{collision::collider::Collider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::gameplay::{buildings::base_building, game_object::GameObject, utility::FollowCursor};

#[derive(Debug, Component, Clone, Default)]
pub struct MainBuilding;

pub fn main_building() -> impl Scene {
    bsn! {
        @base_building()
        GameObject
        MainBuilding
        Sprite{custom_size: vec2(100.0, 100.0)}
        RigidBody::Static
        Collider
        Pickable
        on(|on: On<PointerOver>| {
            warn!("Main building was pressed!")
        })
    }
}
