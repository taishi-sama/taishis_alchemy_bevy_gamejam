use std::mem;

use bevy::prelude::*;

use crate::gameplay::{
    building_preview::{OnSuccessSpawn, PreviewMarker},
    utility::FollowCursor,
};

#[derive(Resource, Default)]
pub struct SelectedBuilding {
    pub preview_scene: Option<Box<dyn Scene>>,
    pub final_building_scene: Option<Box<dyn Scene>>,
}

#[derive(Debug, Event, Clone)]
pub struct TryPutBuildingEvent {
    pub entity: Entity,
}

pub fn spawn_preview(mut trg: ResMut<SelectedBuilding>, mut commands: Commands) {
    if trg.final_building_scene.is_some() && trg.preview_scene.is_some() {
        commands.despawn_all::<With<PreviewMarker>>();
        let preview_scene = mem::take(&mut trg.preview_scene).unwrap();
        let final_building_scene = mem::take(&mut trg.final_building_scene).unwrap();
        commands
            .spawn_scene(preview_scene)
            .insert(OnSuccessSpawn(Some(final_building_scene)))
            .insert(FollowCursor)
            .insert(Pickable {
                should_block_lower: false,
                ..Default::default()
            })
            .observe(|mut on: On<PointerPress>, mut commands: Commands| {
                if on.button == PointerButton::Primary {
                    info!("Trying to place entity....");
                    commands.trigger(TryPutBuildingEvent { entity: on.entity });
                } else if on.button == PointerButton::Secondary {
                    commands.despawn_all::<With<PreviewMarker>>();
                } else {
                    on.propagate(true);
                }
            });
    }
}

pub fn drop_preview(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.despawn_all::<With<PreviewMarker>>();
    }
}
