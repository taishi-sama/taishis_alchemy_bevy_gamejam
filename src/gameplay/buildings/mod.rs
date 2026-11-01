use bevy::prelude::*;

use crate::{
    GameState,
    gameplay::{
        buildings::{main_building::main_building, towers::towers_plugin},
        collisions::player_collisions,
        game_object::GameObject,
    },
};
pub mod main_building;
pub mod towers;

pub struct BuildingPlugin;

pub fn base_building() -> impl Scene {
    bsn! {
        GameObject
        @player_collisions()
    }
}

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(towers_plugin);
        app.add_systems(OnEnter(GameState::Playing), main_building.spawn());
    }
}
