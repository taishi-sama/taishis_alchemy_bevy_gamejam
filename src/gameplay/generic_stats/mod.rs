use std::time::Duration;

use bevy::prelude::*;

use crate::GameState;

pub fn generic_stats_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_attack_cd, advance_attack_cd_timer, despawn_dead)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

#[derive(Debug, Component, Default, Clone, Reflect)]
pub struct Health(pub f32);

pub fn despawn_dead(q: Query<(Entity, &Health), Changed<Health>>, mut commands: Commands) {
    q.iter().for_each(|(e, h)| {
        if h.0 <= 0.0 {
            commands.entity(e).despawn();
        }
    });
}

#[derive(Debug, Component, Default, Clone, Reflect)]
pub struct MaxMovingSpeed(pub f32);

#[derive(Debug, Component, Default, Clone, Reflect)]
#[require(FinalStats)]
pub struct BaseAttackStats {
    pub base_radius: f32,
    pub base_interval_secs: f32,
}

// Generic calculations of attack CD with all modifiers
pub fn update_attack_cd(
    mut q: Query<(&mut FinalStats, &BaseAttackStats), Or<(Changed<BaseAttackStats>,)>>,
) {
    q.iter_mut()
        .for_each(|(mut final_stats, base_attack_stats)| {
            info!("Stats recalculated..");
            final_stats
                .attack_timer
                .set_duration(Duration::from_secs_f32(
                    base_attack_stats.base_interval_secs,
                ));
            final_stats.attack_radius = base_attack_stats.base_radius
        });
}

pub fn advance_attack_cd_timer(mut q: Query<(&mut FinalStats,)>, t: Res<Time<Fixed>>) {
    let delta = t.delta();
    q.iter_mut().for_each(|(mut stats,)| {
        stats.attack_timer.tick(delta);
    });
}

#[derive(Debug, Component, Clone, Reflect)]
pub struct FinalStats {
    pub attack_timer: Timer,
    pub attack_radius: f32,
}

impl Default for FinalStats {
    fn default() -> Self {
        Self {
            attack_timer: Timer::new(Duration::MAX, TimerMode::Repeating),
            attack_radius: Default::default(),
        }
    }
}
#[derive(Debug, Component, Default, Clone, Reflect)]
pub struct ProjectileDamage(pub f32);
