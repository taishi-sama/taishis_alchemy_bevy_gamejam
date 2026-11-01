use bevy::prelude::*;

use crate::gameplay::projectiles::player_projectiles::process_collisions;

pub mod player_projectiles;

pub fn projectiles_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, process_collisions);
}
