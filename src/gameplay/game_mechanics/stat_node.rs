use bevy::prelude::*;

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=NodeContainer)]
pub struct ContainedStatNode(pub Entity);
#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=ContainedStatNode, linked_spawn)]
pub struct NodeContainer(Vec<Entity>);

#[derive(Debug, Clone, Component, FromTemplate, Default, Reflect)]
pub struct StatNode {}

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=IOContainer)]
pub struct ContainedIO(pub Entity);
#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=ContainedIO, linked_spawn)]
pub struct IOContainer(Vec<Entity>);

#[derive(Debug, Clone, Component, FromTemplate, Default, Reflect)]
pub struct InputMarker;
#[derive(Debug, Clone, Component, FromTemplate, Default, Reflect)]
pub struct OutputMarker;

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=Output)]
#[require(InputMarker)]
pub struct Input(pub Entity);

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=Input)]
#[require(OutputMarker)]
pub struct Output(Entity);
