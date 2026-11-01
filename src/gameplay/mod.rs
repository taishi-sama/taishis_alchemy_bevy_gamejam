mod actions;
mod audio;
pub mod building_preview;
pub mod buildings;
pub mod camera;
pub mod collisions;
pub mod enemies;
pub mod game_ai;
mod game_object;
pub mod generic_stats;
pub mod projectiles;
pub mod ui;
pub mod utility;

use avian2d::{PhysicsPlugins, collision::collider::Collider, dynamics::integrator::Gravity};
use bevy::prelude::*;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use game_object::GameObjectPlugin;
use vleue_navigator::{VleueNavigatorPlugin, prelude::NavmeshUpdaterPlugin};

use crate::gameplay::building_preview::building_preview;
use crate::gameplay::generic_stats::generic_stats_plugin;
use crate::gameplay::projectiles::projectiles_plugin;
use crate::gameplay::{
    buildings::BuildingPlugin, camera::CameraPlugin, collisions::Obstacle, enemies::EnemyPlugin,
    game_ai::game_ai_plugin, ui::ui_system, utility::utility_plugin,
};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            InternalAudioPlugin,
            GameObjectPlugin,
            CameraPlugin,
            BuildingPlugin,
            PhysicsPlugins::default(),
            EnemyPlugin,
            game_ai_plugin,
            ui_system,
            utility_plugin,
            building_preview,
            generic_stats_plugin,
            projectiles_plugin,
        ));
        app.insert_resource(Gravity::ZERO);
        app.add_plugins((
            VleueNavigatorPlugin,
            // Auto update the navmesh.
            // Obstacles will be entities with the `Obstacle` marker component,
            // and use the `Aabb` component as the obstacle data source.
            NavmeshUpdaterPlugin::<Collider, Obstacle>::default(),
        ));
    }
}
