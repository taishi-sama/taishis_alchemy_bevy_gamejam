pub mod crossbow;

use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::RigidBody,
    spatial_query::{SpatialQuery, SpatialQueryFilter},
};
use bevy::prelude::*;

use crate::{
    GameState,
    gameplay::{
        buildings::{base_building, towers::crossbow::perform_crossbow_attack},
        collisions::GameLayer::Enemy,
        game_mechanics::stat_node::{
            IOContainer, Input, InputMarker, NodeContainer, OutputMarker, StatNode,
        },
        ui::{UISurface, edit_pane::stat_node_edit_panel::spawn_edit_panel_from},
    },
};

pub fn towers_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        perform_crossbow_attack.run_if(in_state(GameState::Playing)),
    );
}

#[derive(Debug, Default, Clone, SceneComponent)]
pub struct Tower {}

impl Tower {
    fn scene() -> impl Scene {
        bsn! {
            #Tower
            Pickable
            RigidBody::Static
            @base_building()
            on(on_click_tower)
            NodeContainer [
                #N1
                StatNode
                IOContainer[
                    #N1I1
                    InputMarker
                    --
                    #N1I2
                    InputMarker
                    --
                    #N1O1
                    OutputMarker
                    --
                    #N1O2
                    OutputMarker
                ]
                --
                #N2
                StatNode
                IOContainer[
                    #N2I1
                    InputMarker
                    Input(#N1O1)
                    --
                    #N2I2
                    InputMarker
                    Input(#N1O2)
                    --
                    #N2O1
                    OutputMarker
                    --
                    #N2O2
                    OutputMarker
                ]
            ]
        }
    }
}

pub fn on_click_tower(
    on: On<PointerClick>,
    mut commands: Commands,
    q_tower: Query<&NodeContainer>,
    q_statnodes: Query<&IOContainer>,
    q_input: Query<&InputMarker>,
    q_output: Query<&OutputMarker>,
    s: Single<(Entity, &UISurface)>,
) {
    warn!("Clicking on {}", on.entity);
    let e = commands
        .spawn_scene(spawn_edit_panel_from(
            on.entity,
            q_tower,
            q_statnodes,
            q_input,
            q_output,
        ))
        .id();
    commands.entity(s.0).add_child(e);
}

pub fn find_all_targets_in_radius(
    origin: Entity,
    radius: f32,
    position: Vec2,
    enemies: bool,
    spatial_query: &SpatialQuery,
) -> Vec<Entity> {
    let mut filter = SpatialQueryFilter::from_excluded_entities([origin]);
    if enemies {
        filter = filter.with_mask([Enemy]);
    }
    spatial_query.shape_intersections(&Collider::circle(radius), position, Rot2::IDENTITY, &filter)
}
