use bevy::{color::palettes::css, prelude::*, scene::Ready, ui_widgets::Activate};

use crate::gameplay::{
    game_mechanics::stat_node::{
        IOContainer, Input, InputMarker, NodeContainer, Output, OutputMarker, StatNode,
    },
    ui::edit_pane::{CloseButton, Header, MovableWindow},
};

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship(relationship_target=RepresentedBy)]
pub struct Represents(pub Entity);

#[derive(Debug, Component, FromTemplate, Reflect)]
#[relationship_target(relationship=Represents, linked_spawn)]
pub struct RepresentedBy(Entity);

pub fn spawn_edit_panel_from(
    tower: Entity,
    q_tower: Query<&NodeContainer>,
    q_statnodes: Query<&IOContainer>,
    q_input: Query<&InputMarker>,
    q_output: Query<&OutputMarker>,
) -> impl Scene {
    let Ok(node_container) = q_tower.get(tower) else {
        error!("{} have no NodeContainer component", tower);
        return Box::new(bsn! {}) as Box<dyn Scene>;
    };
    let statnodes: Vec<Box<dyn Scene>> = vec![];

    return Box::new(bsn! {}) as Box<dyn Scene>;
}

pub fn on_ready_spawn_edit_panel(on: On<Ready>) {}

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
                Node{
                    width: px(10),
                    height: px(10),
                }
                BackgroundColor(css::YELLOW)
                --
                Node{
                    width: px(10),
                    height: px(10),
                }
                BackgroundColor(css::YELLOW)
            ]
            --
            #RightPanel
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                width: percent(0),
                height: percent(100),
                right: px(-10),
                top: px(0)
            }
            Children[
                Node{
                    width: px(10),
                    height: px(10),
                }
                BackgroundColor(css::YELLOW)
                --
                Node{
                    width: px(10),
                    height: px(10),
                }
                BackgroundColor(css::YELLOW)
            ]
        ]
    })
}
