pub mod zombie;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    GameState,
    gameplay::{
        collisions::enemies_collisions, enemies::zombie::spawn_zombies_around,
        game_object::GameObject,
    },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_zombies_around
                .run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_secs(1)))
                .run_if(|| false),
        );
    }
}

#[derive(Debug, Clone, SceneComponent, Default)]
pub struct Enemy {}

impl Enemy {
    pub fn scene() -> impl Scene {
        bsn! {
            GameObject
            #Enemy
            @enemies_collisions()
        }
    }
}
