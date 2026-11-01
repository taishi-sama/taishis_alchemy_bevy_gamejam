use avian2d::{collision::collider::CollisionLayers, prelude::PhysicsLayer};
use bevy::prelude::*;

#[derive(Debug, Component, Clone, Default)]
pub struct Obstacle;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Player, // Layer 1
    PlayerProjectiles,
    Enemy, // Layer 2
    EnemyProjectiles,
    Ground, // Layer 3
}

pub fn player_collisions() -> impl Scene {
    bsn! {
        CollisionLayers::new(
            GameLayer::Player,
            [GameLayer::Default, GameLayer::Player, GameLayer::Enemy, GameLayer::Ground, GameLayer::EnemyProjectiles],
        )
    }
}

pub fn player_projectiles_collisions() -> impl Scene {
    bsn! {
        CollisionLayers::new(
            GameLayer::PlayerProjectiles,
            [GameLayer::Default, GameLayer::Enemy, GameLayer::Ground],
        )
    }
}

pub fn enemies_collisions() -> impl Scene {
    bsn! {
        CollisionLayers::new(
            GameLayer::Enemy,
            [GameLayer::Default, GameLayer::Enemy, GameLayer::Player, GameLayer::PlayerProjectiles, GameLayer::Ground],
        )
    }
}

pub fn enemy_projectiles_collisions() -> impl Scene {
    bsn! {
        CollisionLayers::new(
            GameLayer::EnemyProjectiles,
            [GameLayer::Default, GameLayer::Player, GameLayer::Ground],
        )
    }
}
