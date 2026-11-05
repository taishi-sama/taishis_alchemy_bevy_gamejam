use bevy::{color::palettes::css, prelude::*, scene::Ready, ui_widgets::Activate};

use crate::gameplay::{
    game_mechanics::stat_node::{
        IOContainer, Input, InputMarker, NodeContainer, Output, OutputMarker, StatNode,
    },
    ui::{
        curved_line::bounded_to_entity::bounded_to_entities_with_offset,
        edit_pane::{CloseButton, Header, MovableWindow, close_button},
    },
};

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=RepresentedBy)]
#[reflect(Component)]
pub struct Represents(pub Entity);

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=Represents, linked_spawn)]
#[reflect(Component)]
pub struct RepresentedBy(Vec<Entity>);

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=ChildrenPorts)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);
#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=ParentWindow)]
#[reflect(Component)]
pub struct ChildrenPorts(Vec<Entity>);

pub fn spawn_edit_panel_from(
    tower: Entity,
    q_tower: Query<&NodeContainer>,
    q_statnodes: Query<&IOContainer>,
    q_input: Query<&InputMarker>,
    q_output: Query<&OutputMarker>,
) -> impl Scene {
    let mut statnodes: Vec<Box<dyn Scene>> = vec![];
    if let Ok(node_container) = q_tower.get(tower) {
        for (i, ent) in node_container.iter().enumerate() {
            let x = i as f32 * 80.0 + 20.0;
            let y = if i % 2 == 0 { 50.0 } else { 80.0 };
            if let Ok(sn) = q_statnodes.get(ent) {
                statnodes.push(Box::new(bsn! {
                    @spawn_node(ent, q_statnodes, q_input, q_output)
                    @MovableWindow {
                        size: vec2(50.0, 40.0),
                        position: vec2(x, y),
                    }
                }));
            }
        }
    } else {
        error!("{} have no NodeContainer component", tower);
    };
    bsn! {
        #MovableWindowRef
        @MovableWindow {
            position: vec2(100.0, 400.0),
            size: vec2(400.0, 300.0)
        }
        Represents(tower)
        BackgroundColor(css::GREY)
        Node{
            border_radius: BorderRadius::all(px(10))
            flex_direction: FlexDirection::Column,
        }
        BoxShadow::new(
            Srgba::BLACK.with_alpha(0.9).into(),
            px(0),
            px(0),
            px(1),
            px(4),
        )
        Children[
            @Header
            Node {
                flex_direction: FlexDirection::RowReverse
                border_radius: BorderRadius::new(px(10), px(10), auto(), auto()),
                padding: UiRect::new(px(10), px(10), px(10), px(10)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart
            }
            BackgroundColor(css::LIGHT_GREY)
            Node {
                width: percent(100),
                height: percent(10),
            }
            Children[
                @close_button( #MovableWindowRef)
                on(|on: On<Activate>, query: Query<&CloseButton>, mut commands: Commands| {
                    let Ok(cb) = query.get(on.entity) else {return ;};
                    commands.entity(cb.on_click_despawn).despawn();
                })
            ]
            --
            #Body
            Node {
                width: percent(100),
                height: percent(100)
            }
            Children[
                {statnodes}
            ]
            on(on_ready_spawn_edit_panel)
        ]

    }
}

pub fn on_ready_spawn_edit_panel(
    on: On<Ready>,
    q_ch: Query<&Children>,
    q_rep: Query<&Represents>,
    q_represented: Query<&RepresentedBy>,
    q_input: Query<&Input>,
    q_is_output_or_input: Query<(), Or<(With<InputMarker>, With<OutputMarker>)>>,
    mut commands: Commands,
) {
    let mut pairs = vec![];
    for i in q_ch.iter_descendants(on.entity) {
        if let Ok(rep) = q_rep.get(i) {
            if q_is_output_or_input.contains(rep.0) {
                commands.entity(i).insert(ParentWindow(on.entity));
            }
            if let Ok(inp) = q_input.get(rep.0) {
                if let Ok(outp) = q_represented.get(inp.0) {
                    for o in outp.iter() {
                        pairs.push((i, o))
                    }
                }
            }
        }
    }
    for (i, o) in pairs {
        commands.spawn_scene(bsn! {
            ChildOf({on.entity})
            @bounded_to_entities_with_offset(i.into(), o.into())
        });
    }
}

pub fn on_dragdrop_io(
    on: On<PointerDragDrop>,
    q_parent_window: Query<&ParentWindow>,
    q_rep: Query<(Entity, &Represents)>,
    q_input: Query<&InputMarker>,
    q_output: Query<&OutputMarker>,
    mut commands: Commands,
) {
    if on.dropped == on.entity {
        warn!("dropped == entity, returning");
        return;
    }
    let Ok(pws) = q_parent_window.get_many([on.dropped, on.entity]) else {
        warn!("No ParentWindow, returning");
        return;
    };
    if pws[0].0 != pws[1].0 {
        return;
    }
    if let Ok(reps) = q_rep.get_many([on.dropped, on.entity]) {
        let mut input: Option<Entity> = None;
        let mut output: Option<Entity> = None;
        for (ent, rep) in &reps {
            if let Ok(inp) = q_input.get(rep.0) {
                input = Some(*ent)
            }
            if let Ok(out) = q_output.get(rep.0) {
                output = Some(*ent)
            }
        }
        if input.is_none() || output.is_none() {
            return;
        }
        let input = input.unwrap();
        let output = output.unwrap();
        commands.spawn_scene(bsn! {
            ChildOf({pws[0].0})
            @bounded_to_entities_with_offset(input.into(), output.into())
        });
    }
}

pub fn spawn_node(
    node: Entity,
    q_statnodes: Query<&IOContainer>,
    q_input: Query<&InputMarker>,
    q_output: Query<&OutputMarker>,
) -> impl Scene {
    let Ok(statnode) = q_statnodes.get(node) else {
        error!("{} have no IOContainer component", node);
        return Box::new(bsn! {}) as Box<dyn Scene>;
    };
    let mut inputs: Vec<Box<dyn Scene>> = vec![];
    let mut outputs: Vec<Box<dyn Scene>> = vec![];

    for i in statnode.iter() {
        if let Ok(ip) = q_input.get(i) {
            inputs.push(Box::new(bsn! {
                Node{
                    width: px(10),
                    height: px(10),
                }
                BackgroundColor(css::YELLOW)
                Represents(i)
                Pickable
                on(on_dragdrop_io)
            }));
        }
        if let Ok(op) = q_output.get(i) {
            outputs.push(Box::new(bsn! {
                Node{
                    width: px(10),
                    height: px(10),
                }
                BackgroundColor(css::YELLOW_GREEN)
                Represents(i)
                Pickable
                on(on_dragdrop_io)
            }));
        }
    }
    Box::new(bsn! {
        #MovableNode
        Node {
            flex_direction: FlexDirection::Column,

        }
        @MovableWindow {
            size: vec2(50.0, 40.0)
        }
        Children [
            @Header
            Node {
                flex_direction: FlexDirection::RowReverse
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart
            }
            BackgroundColor(css::LIGHT_GREY)
            Node {
                width: percent(100),
                height: percent(10),
            }
            Children[
                Node {
                    height: percent(100),
                    width: percent(20),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                }
                BackgroundColor(css::RED)
                @CloseButton{on_click_despawn: #MovableNode}
                on(|on: On<Activate>, query: Query<&CloseButton>, mut commands: Commands| {
                    let Ok(cb) = query.get(on.entity) else {return ;};
                    commands.entity(cb.on_click_despawn).despawn();
                })
            ]
            --
            #Body
            Node {
                width: percent(100),
                height: percent(100),
            }
            BackgroundColor(css::PURPLE)
            --
            #LeftPanel
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                width: percent(0),
                height: percent(100),
                left: px(-10),
                top: px(0)
            }
            Children[
                {inputs}
            ]
            --
            #RightPanel
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                width: percent(0),
                height: percent(100),
                right: px(0),
                top: px(0)
            }
            Children[
                {outputs}
            ]
        ]
    })
}
