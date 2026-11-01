use avian2d::{
    collision::collider::{Collider, CollidingEntities, Sensor},
    dynamics::rigid_body::{LinearVelocity, RigidBody, mass_properties::components::Mass},
};
use bevy::{color::palettes::css, prelude::*};

use crate::gameplay::{
    collisions::player_projectiles_collisions,
    enemies::Enemy,
    game_object::GameObject,
    generic_stats::{Health, ProjectileDamage},
};

#[derive(Debug, Default, Clone, SceneComponent)]
#[scene(PlayerProjectileProps)]
pub struct PlayerProjectile {}

#[derive(Debug)]
pub struct PlayerProjectileProps {
    pub position: Vec2,
    pub direction: Dir2,
    pub speed: f32,
}
impl Default for PlayerProjectileProps {
    fn default() -> Self {
        Self {
            direction: Dir2::NORTH,
            speed: 1.0,
            position: Default::default(),
        }
    }
}

impl PlayerProjectile {
    pub fn scene(props: PlayerProjectileProps) -> impl Scene {
        bsn! {
            GameObject
            #PlayerProjectile
            @player_projectiles_collisions()
            Sprite {
                custom_size: vec2(3.0, 3.0),
                color: css::RED
            }
            ProjectileDamage(102.0)
            Transform::from_translation(props.position.extend(0.5)).with_rotation(Quat::from_rotation_arc_2d(vec2(0.0, 0.0), *props.direction))
            LinearVelocity({ *props.direction * props.speed })
            Collider::circle(1.5)
            RigidBody::Dynamic
            CollidingEntities
            Mass(1.0)
            Sensor
        }
    }
}

pub fn process_collisions(
    query: Query<
        (Entity, &CollidingEntities, &ProjectileDamage),
        (With<PlayerProjectile>, Without<Enemy>),
    >,
    mut q_enemy: Query<(&mut Health,), With<Enemy>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities, damage) in &query {
        if colliding_entities.0.is_empty() {
        } else {
            for i in colliding_entities.iter() {
                if let Ok((mut h,)) = q_enemy.get_mut(*i) {
                    h.0 -= damage.0
                }
            }
            commands.entity(entity).despawn();
        }
    }
}
