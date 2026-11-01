mod actions;
mod audio;
mod collision;
mod finish_areas;
mod game_object;
pub mod main_building;
mod player;

use bevy::prelude::*;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use finish_areas::FinishAreaPlugin;
use game_object::GameObjectPlugin;
use player::PlayerPlugin;

use crate::gameplay::main_building::BuildingPlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            InternalAudioPlugin,
            FinishAreaPlugin,
            GameObjectPlugin,
            PlayerPlugin,
            BuildingPlugin,
        ));
    }
}
