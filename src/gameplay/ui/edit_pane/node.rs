use bevy::{
    color::palettes::css, ecs::template::EntityTemplate, prelude::*, scene::Ready,
    ui_widgets::Activate,
};

use crate::gameplay::ui::edit_pane::{CloseButton, Header, MovableWindow, close_button};

pub fn movable_node(scene: Box<dyn Scene>) -> impl Scene {
    bsn! {
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
                @scene
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
                right: px(0),
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
    }
}
