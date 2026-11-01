use avian2d::{collision::collider::Collider, spatial_query::SpatialQuery};
use bevy::{color::palettes::css, prelude::*};

use crate::gameplay::{
    building_preview::PreviewMarker,
    buildings::towers::{Tower, find_all_targets_in_radius},
    enemies::Enemy,
    generic_stats::{BaseAttackStats, FinalStats},
    projectiles::player_projectiles::PlayerProjectile,
};

#[derive(Debug, Clone, Default, Component)]
pub struct Crossbow;

pub fn crossbow_preview() -> impl Scene {
    bsn! {
        PreviewMarker
        Sprite{
            custom_size: vec2(50.0, 50.0),
            color: css::YELLOW
        }
        Collider::circle(25.0)
    }
}

pub fn crossbow() -> impl Scene {
    bsn! {
        Sprite{
            custom_size: vec2(50.0, 50.0),
            color: css::YELLOW
        }
        Collider::circle(25.0)
        Crossbow
        @Tower
        BaseAttackStats {
            base_radius: 200.0,
            base_interval_secs: 1.0
        }
    }
}

pub fn perform_crossbow_attack(
    q: Query<(Entity, &FinalStats, &Transform), (With<Crossbow>, Without<Enemy>)>,
    q_enemy: Query<(&Transform,), (With<Enemy>)>,
    command: ParallelCommands,
    spatial_query: SpatialQuery,
) {
    q.par_iter().for_each(|(e, stats, transform)| {
        if stats.attack_timer.times_finished_this_tick() < 1 {
            return;
        }
        let res = find_all_targets_in_radius(
            e,
            stats.attack_radius,
            transform.translation.xy(),
            true,
            &spatial_query,
        );
        let closest = res.iter()
            .filter_map(|x| q_enemy.get(*x).ok())
            .min_by_key(|(tr,)| {
                (tr.translation
                    .xy()
                    .distance_squared(transform.translation.xy()))
                .round() as u64
            });
        if let Some((enemy_transform,)) = closest {
            for i in 0..stats.attack_timer.times_finished_this_tick() {
                warn!("Shot!");
                command.command_scope(|mut c| {
                    c.spawn_scene(bsn! {
                        @PlayerProjectile {
                            @position: { transform.translation.xy() },
                            @speed: 300.0,
                            @direction: { Dir2::new_unchecked((enemy_transform.translation.xy() - transform.translation.xy()).normalize()) }
                        }
                    });
                });
            }
        }
    });
}
