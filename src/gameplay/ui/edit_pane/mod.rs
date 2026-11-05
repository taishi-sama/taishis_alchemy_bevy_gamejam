pub mod node;
pub mod stat_node_edit_panel;

use bevy::{
    color::palettes::css,
    ecs::template::EntityTemplate,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{self, Activate},
};

use crate::gameplay::ui::{
    curved_line::{
        bounded_to_entity::{bounded_to_entities, bounded_to_entities_with_offset},
        curved_line,
    },
    edit_pane::node::movable_node,
};

pub fn edit_pane_plugin(app: &mut App) {
    app.add_systems(Update, synchronize);
}

#[derive(Debug, Clone, SceneComponent, Reflect)]
pub struct MovableWindow {
    pub position: Vec2,
    pub size: Vec2,
}

pub fn synchronize(mut q: Query<(&MovableWindow, &mut Node), Changed<MovableWindow>>) {
    for (mw, mut node) in q.iter_mut() {
        node.left = px(mw.position.x);
        node.top = px(mw.position.y);
        node.width = px(mw.size.x);
        node.height = px(mw.size.y);
    }
}

impl MovableWindow {
    pub fn scene() -> impl Scene {
        bsn! {
            #MovableWindow
            Node {
                position_type: PositionType::Absolute,
            }
        }
    }
}

impl Default for MovableWindow {
    fn default() -> Self {
        Self {
            position: vec2(30.0, 30.0),
            size: vec2(200.0, 170.0),
        }
    }
}
#[derive(Debug, Clone, SceneComponent, Reflect, Default)]
pub struct Header {
    pub is_dragged: bool,
}

impl Header {
    pub fn scene() -> impl Scene {
        bsn! {
            #Header
            on(header_started_being_dragged)
            on(header_being_dragged)
            on(header_ended_being_dragged)
        }
    }
}
fn header_started_being_dragged(
    on: On<PointerDragStart>,
    mut headers: Query<(&ChildOf, &mut Header)>,
    window: Query<&MovableWindow>,
) {
    let Ok((child_of, mut header)) = headers.get_mut(on.entity) else {
        return;
    };
    let Ok(_parent_window) = window.get(child_of.parent()) else {
        return;
    };
    header.is_dragged = true;
}
fn header_being_dragged(
    on: On<PointerDrag>,
    mut headers: Query<(&ChildOf, &Header)>,
    mut window: Query<(&mut MovableWindow, Option<&ChildOf>)>,
    window_parent: Query<&ComputedNode>,
) {
    let Ok((child_of, header)) = headers.get_mut(on.entity) else {
        return;
    };
    if !header.is_dragged {
        return;
    }
    let Ok((mut parent_window, parent_of_window)) = window.get_mut(child_of.parent()) else {
        return;
    };

    parent_window.position += on.delta;
    if let Some(parent_of_window) = parent_of_window
        && let Ok(cn) = window_parent.get(parent_of_window.parent())
    {
        parent_window.position = parent_window
            .position
            .clamp(vec2(0.0, 0.0), cn.size() - parent_window.size)
    }
}
fn header_ended_being_dragged(
    on: On<PointerDragEnd>,
    mut headers: Query<(&ChildOf, &mut Header)>,
    window: Query<&MovableWindow>,
) {
    let Ok((child_of, mut header)) = headers.get_mut(on.entity) else {
        return;
    };
    let Ok(_parent_window) = window.get(child_of.parent()) else {
        return;
    };
    header.is_dragged = false;
}

#[derive(Debug, SceneComponent, FromTemplate, Clone)]
pub struct CloseButton {
    pub on_click_despawn: Entity,
}

impl CloseButton {
    pub fn scene() -> impl Scene {
        bsn! {
            ui_widgets::Button
        }
    }
}

fn close_button(entity: EntityTemplate) -> impl Scene {
    bsn! {
        @CloseButton{on_click_despawn: entity}
        ui_widgets::Button
        Node {
            height: px(20),
            width: px(20),
            justify_self: JustifySelf::End
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(2.0))
        }
        BorderColor::from(Color::BLACK)
        BackgroundColor(css::RED)
    }
}

pub fn movable_window() -> impl Scene {
    bsn! {
        #MovableWindowRef
        @MovableWindow {
            size: vec2(400.0, 300.0)
        }

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
                @bounded_to_entities_with_offset(#Node1, #Node2)
                --
                @bounded_to_entities_with_offset(#Node2, #Node3)
                --
                @bounded_to_entities_with_offset(#Node3, #Node1)
                --
                #Node1
                @movable_node({
                    Box::new(bsn! {
                        BackgroundColor(css::RED)

                    })
                })
                @MovableWindow{position: vec2(10.0, 10.0)}
                --
                #Node2
                @movable_node({
                    Box::new(bsn! {
                        BackgroundColor(css::GREEN)
                    })
                })
                @MovableWindow{position: vec2(40.0, 120.0)}
                --
                #Node3
                @movable_node({
                    Box::new(bsn! {
                        BackgroundColor(css::YELLOW)
                    })
                })
                @MovableWindow{position: vec2(140.0, 30.0)}
            ]
        ]
    }
}
