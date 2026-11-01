use bevy::prelude::*;

use crate::GameState;


pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_main_building,
        );
    }
}
#[derive(Debug, Component, Clone, Default)]
pub struct MainBuilding;



pub fn main_building() -> impl Scene {
    bsn! {
        MainBuilding
        Sprite{custom_size: vec2(100.0, 100.0)}
    }
}

pub fn spawn_main_building(mut commands: Commands) {
    commands.spawn_scene(main_building());
}