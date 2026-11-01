pub mod spawn_preview;

use std::mem;

use crate::{
    GameState,
    gameplay::{
        building_preview::spawn_preview::{
            SelectedBuilding, TryPutBuildingEvent, drop_preview, spawn_preview,
        },
        game_object::GameObject,
    },
};
use bevy::prelude::*;

#[derive(Debug, Component, Default, Clone)]
#[require(GameObject)]
pub struct PreviewMarker;

#[derive(Component)]
pub struct OnSuccessSpawn(pub Option<Box<dyn Scene>>);

pub fn building_preview(app: &mut App) {
    app.init_resource::<SelectedBuilding>();
    app.add_systems(
        Update,
        (spawn_preview, drop_preview).run_if(in_state(GameState::Playing)),
    );
    app.add_observer(try_place);
}

pub fn try_place(
    on: On<TryPutBuildingEvent>,
    mut commands: Commands,
    mut q: Single<(&mut OnSuccessSpawn, &Transform), With<PreviewMarker>>,
) {
    let entity = on.entity;
    let success = true;
    if success {
        let (mut scene, transform) = q.into_inner();
        let scene = mem::take(&mut scene.0);
        if let Some(scene) = scene {
            commands.spawn_scene(scene).insert(transform.clone());
            commands.despawn_all::<With<PreviewMarker>>();
        } else {
            error!("OnSuccessSpawn on {entity} is empty!!!")
        }
    }
}
