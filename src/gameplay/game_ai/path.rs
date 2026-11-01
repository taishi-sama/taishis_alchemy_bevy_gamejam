use avian2d::dynamics::rigid_body::{AngularVelocity, LinearVelocity};
use bevy::prelude::*;
use vleue_navigator::{NavMesh, prelude::ManagedNavMesh};

use crate::{GameState, gameplay::generic_stats::MaxMovingSpeed};

pub fn pathing_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_path_on_change, update_path, move_in_path).run_if(in_state(GameState::Playing)),
    );
}

#[derive(Debug, Clone, Component, Default)]
pub struct Path {
    pub target: Vec2,
}

pub fn update_path_on_change(
    mut q: Query<(Entity, &mut Path, Option<&mut PathingTo>), Changed<Path>>,
    mut commands: Commands,
) {
    q.iter_mut().for_each(|(e, path, pathing_to)| {
        if let Some(mut pathing_to) = pathing_to {
            *pathing_to = PathingTo {
                current: path.target,
                ..Default::default()
            };
        } else {
            commands.entity(e).insert(PathingTo {
                current: path.target,
                ..Default::default()
            });
        }
    });
}

#[derive(Debug, Clone, Component, Default)]
struct PathingTo {
    pub time_to_update: i32,
    pub radius_of_detection_intermedian: f32,
    pub radius_of_detection_final: f32,
    pub current: Vec2,
    pub next: Vec<Vec2>,
}

pub fn update_path(
    mut path: Query<(&mut PathingTo, &Path, &Transform)>,
    navmeshes: Res<Assets<NavMesh>>,
    navmesh: Single<&ManagedNavMesh>,
) {
    let Some(navmesh) = navmeshes.get(*navmesh) else {
        return;
    };
    for (mut p, path, t) in &mut path {
        if p.time_to_update <= 0 {
            let target = path.target;

            let Some(p_path) = navmesh.transformed_path(t.translation, target.extend(0.)) else {
                warn!("No path found to {}... Skipping", target);
                continue;
            };

            if let Some((first, remaining)) = p_path.path.split_first() {
                let mut remaining: Vec<Vec2> = remaining.iter().map(|x| x.xy()).collect();
                remaining.reverse();
                p.current = first.xy();
                p.next = remaining;
            }
            p.time_to_update = 30;
        } else {
            p.time_to_update -= 1;
        }
    }
}

pub fn move_in_path(
    mut path: Query<(
        Entity,
        &mut PathingTo,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut Transform,
        Option<&MaxMovingSpeed>,
    )>,
    mut commands: Commands,
) {
    for (e, mut p, mut l, mut a, mut t, ms) in &mut path {
        let ms = ms.map(|x| x.0).unwrap_or(10.);
        let move_vector = p.current - t.translation.xy();
        l.0 = move_vector.normalize_or_zero() * ms;
        t.rotation =
            Quat::from_rotation_arc_2d(vec2(1.0, 0.0), move_vector.normalize_or(vec2(1.0, 0.0)));
        a.0 = 0.0;
        if move_vector.length()
            < if p.next.is_empty() {
                p.radius_of_detection_final
            } else {
                p.radius_of_detection_intermedian
            }
        {
            if let Some(next) = p.next.pop() {
                p.current = next;
            } else {
                commands.entity(e).remove::<Path>();
                commands.entity(e).remove::<PathingTo>();
            }
        }
    }
}

pub fn display_debug_path(navigator: Query<(&Transform, &PathingTo)>, mut gizmos: Gizmos) {
    for (transform, path) in &navigator {
        let mut to_display = path.next.iter().map(|v| v.xy()).collect::<Vec<_>>();
        to_display.push(path.current.clone().xy());
        to_display.push(transform.translation.xy());
        to_display.reverse();
        if to_display.len() >= 1 {
            gizmos.linestrip_2d(to_display, Color::srgb(0.7, 0.7, 0.1));
        }
    }
}
