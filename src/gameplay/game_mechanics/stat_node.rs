use bevy::prelude::*;

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=NodeContainer)]
#[reflect(Component)]
pub struct ContainedStatNode(pub Entity);
#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=ContainedStatNode, linked_spawn)]
#[reflect(Component)]
pub struct NodeContainer(Vec<Entity>);

#[derive(Debug, Clone, Component, FromTemplate, Default, Reflect)]
#[reflect(Component)]
pub struct StatNode {}

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=IOContainer)]
#[reflect(Component)]
pub struct ContainedIO(pub Entity);
#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=ContainedIO, linked_spawn)]
#[reflect(Component)]
pub struct IOContainer(Vec<Entity>);

#[derive(Debug, Clone, Component, FromTemplate, Default, Reflect)]
#[reflect(Component)]
pub struct InputMarker;
#[derive(Debug, Clone, Component, FromTemplate, Default, Reflect)]
#[reflect(Component)]
pub struct OutputMarker;

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=Output)]
#[require(InputMarker)]
#[reflect(Component)]
pub struct Input(pub Entity);

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=Input)]
#[require(OutputMarker)]
#[reflect(Component)]
pub struct Output(Entity);
