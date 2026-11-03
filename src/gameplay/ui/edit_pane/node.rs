use bevy::{color::palettes::css, ecs::template::EntityTemplate, prelude::*, ui_widgets::Activate};

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
            Children[
                @scene
            ]
        ]
    }
}