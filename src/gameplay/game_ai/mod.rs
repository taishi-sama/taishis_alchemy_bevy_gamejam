use bevy::app::App;

use crate::gameplay::game_ai::path::pathing_plugin;

pub mod path;

pub fn game_ai_plugin(app: &mut App) {
    app.add_plugins(pathing_plugin);
}
