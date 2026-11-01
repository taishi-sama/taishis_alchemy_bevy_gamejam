use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::mass_properties::bevy_heavy::ComputeMassProperties2d,
};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::GameState;

pub fn utility_plugin(app: &mut App) {
    app.add_systems(Update, (follow_cursor).run_if(in_state(GameState::Playing)));
}

#[derive(Debug, Component, Default, Clone)]
pub struct FollowCursor;

pub fn follow_cursor(
    mut q: Query<(&mut Transform, &Collider), With<FollowCursor>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    if let Some(pos) = q_window.cursor_position()
        && let Ok(world_pos) = q_camera.0.viewport_to_world_2d(q_camera.1, pos)
    {
        q.iter_mut().for_each(|(mut tr, coll)| {
            tr.translation = (coll.center_of_mass() + world_pos).extend(tr.translation.z);
        });
    }
}
