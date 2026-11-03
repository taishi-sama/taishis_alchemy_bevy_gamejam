use bevy::{ecs::{relationship::Relationship, template::EntityTemplate}, prelude::*};

use crate::gameplay::ui::curved_line::{CurvedLineParams, curved_line};

#[derive(Debug, Reflect, Component, FromTemplate)]
#[relationship(relationship_target = FollowedAsBeginning)]
pub struct FollowAsBeginning(pub Entity);

#[derive(Debug, Reflect, Component, FromTemplate)]
#[relationship_target(relationship = FollowAsBeginning)]
pub struct FollowedAsBeginning(Vec<Entity>);

#[derive(Debug, Reflect, Component, FromTemplate)]
#[relationship(relationship_target = FollowedAsEnding)]
pub struct FollowAsEnding(pub Entity);

#[derive(Debug, Reflect, Component, FromTemplate)]
#[relationship_target(relationship = FollowAsEnding)]
pub struct FollowedAsEnding(Vec<Entity>);

#[derive(Debug, Reflect, Clone, FromTemplate, Component)]
pub struct BoundedToEntities {
    pub offset1: Vec2,
    pub offset2: Vec2,
}

pub fn bounded_to_entities_beginning(
    q: Query<(&UiGlobalTransform, &FollowedAsBeginning), Or<(Changed<UiGlobalTransform>, Changed<FollowedAsBeginning>)>>,
    mut q_curved: Query<(&BoundedToEntities, &mut CurvedLineParams, &UiGlobalTransform)>,
) {
    for (ugt, fabi) in q.iter() {
        for i in fabi.iter(){
            if let Ok((bte, mut curve, ugt_c)) = q_curved.get_mut(i) {
                curve.point1 = ugt.translation - ugt_c.translation;
                curve.point2 = curve.point1 + bte.offset1;
            } else {
                error!("Match not found!")
            }
        }
    }
}

pub fn bounded_to_entities_ending(
    q: Query<(&UiGlobalTransform, &FollowedAsEnding), Or<(Changed<UiGlobalTransform>, Changed<FollowedAsEnding>)>>,
    mut q_curved: Query<(&BoundedToEntities, &mut CurvedLineParams, &UiGlobalTransform)>,
) {
    for (ugt, faei) in q.iter() {
        for i in faei.iter() {
            if let Ok((bte, mut curve, ugt_c)) = q_curved.get_mut(i) {
                curve.point4 = ugt.translation - ugt_c.translation;
                curve.point3 = curve.point4 + bte.offset2;
            } else {
                error!("Match not found!")
            }
        }
    }
}

pub fn bounded_to_entities(beginning: EntityTemplate, ending: EntityTemplate) -> impl Scene {
    bsn! {
        @curved_line()
        FollowAsBeginning(beginning)
        FollowAsEnding(ending)
        BoundedToEntities
    }
}
pub fn bounded_to_entities_with_offset(beginning: EntityTemplate, ending: EntityTemplate) -> impl Scene {
    bsn! {
        @bounded_to_entities(beginning, ending)
        BoundedToEntities {
            offset1: vec2(25.0, 0.0),
            offset2: vec2(-25.0, 0.0)
        }
    }
}