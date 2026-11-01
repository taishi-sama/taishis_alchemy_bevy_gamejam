use std::f32::consts;

use crate::gameplay::{
    buildings::main_building::MainBuilding, enemies::Enemy, game_ai::path::Path,
    generic_stats::Health,
};
use avian2d::{
    collision::collider::{Collider, ColliderConstructor},
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;

pub fn spawn_zombies_around(
    mut commands: Commands,
    main_building: Query<&Transform, With<MainBuilding>>,
) {
    // return;
    let t = main_building.single().unwrap();
    commands.spawn_scene(bsn! {
        @spawn_zombie(vec2(400., 400.).rotate_angle(rand::random_range(0.0f32..consts::TAU)) + t.translation.xy())
        Path{ target: { t.translation.xy() } }
    });
}

pub fn spawn_zombie(location: Vec2) -> impl Scene {
    bsn! {
        @Enemy
        Transform::from_translation(location.extend(0.0))
        #Zombie
        Health(100.0)
        Sprite{custom_size: vec2(10.0, 10.0), color: Color::srgba_u8(10, 170, 10, 255)}
        Collider::circle(5.0)
        RigidBody::Dynamic

    }
}
