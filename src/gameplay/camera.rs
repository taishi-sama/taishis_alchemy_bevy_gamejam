use bevy::prelude::*;
use bevy_egui::PrimaryEguiContext;

use crate::{GameState, gameplay::game_object::GameObject};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), reset_camera);
    }
}

fn reset_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        GameObject,
        IsDefaultUiCamera,
        PrimaryEguiContext,
    ));
}
