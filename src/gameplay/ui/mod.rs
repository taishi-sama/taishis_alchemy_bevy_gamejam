pub mod curved_line;
pub mod edit_pane;
pub mod ui_state;
use crate::gameplay::ui::curved_line::{curved_line, curved_line_plugin};
use crate::gameplay::ui::edit_pane::{Header, MovableWindow, edit_pane_plugin, movable_window};
use crate::{
    GameState,
    gameplay::{
        building_preview::spawn_preview::SelectedBuilding,
        buildings::towers::crossbow::{crossbow, crossbow_preview},
        game_object::GameObject,
    },
};
use bevy::{
    color::palettes::css::{self, BLACK, RED, WHITE},
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{self, Activate},
};

pub fn ui_system(app: &mut App) {
    app.add_plugins((curved_line_plugin, edit_pane_plugin));
    app.add_systems(OnEnter(GameState::Playing), game_ui.spawn());
    app.add_systems(
        Update,
        update_button_appearance.run_if(in_state(GameState::Playing)),
    );
}

pub fn game_ui() -> impl Scene {
    bsn! {
        #UISurface
        GameObject
        Node {
            width: percent(100),
            height: percent(100)

        }
        Children [
            @dock()
            --
            @movable_window()
        ]
        Pickable::IGNORE

    }
}

pub fn dock() -> impl Scene {
    bsn! {
        #DOCK
        Node {
            position_type: PositionType::Absolute,
            bottom: px(10.0),
            width: percent(80),
            height: px(40.0),
            left: percent(10),
            right: percent(10),
            border_radius: BorderRadius::px(20.0, 20.0, 20.0, 20.0)
            justify_self: JustifySelf::Center,
            // border: UiRect::all(px(1.0)),
            padding: UiRect::px(10.0, 10.0, 1.0, 1.0),
            column_gap: px(20.0)
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        BoxShadow::new(
            Srgba::BLACK.with_alpha(0.4).into(),
            px(0),
            px(0),
            px(1),
            px(4),
        )
        BackgroundColor(Color::srgba_u8(60, 60, 60, 100))
        BorderColor::from(Color::BLACK)
        Children [
            @button("Crossbow")
            on(|_activate: On<Activate>, mut sel: ResMut<SelectedBuilding>| {
                sel.preview_scene = Some(Box::new(crossbow_preview()));
                sel.final_building_scene = Some(Box::new(crossbow()));
                info!("Button clicked!");
            })
            --
            @button("Beam")
            on(|_activate: On<Activate>| {
                info!("Button clicked!");
            })
            --
            @button("Potion")
            on(|_activate: On<Activate>| {
                info!("Button clicked!");
            })
        ]
    }
}
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Component, Default, Clone, Reflect)]
pub struct SpawnDockButton;

fn button(label: &str) -> impl Scene {
    bsn! {
        SpawnDockButton
        ui_widgets::Button
        Node {
            height: percent(100),
            width: auto(),

            border_radius: BorderRadius::px(5.0, 5.0, 5.0, 5.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::px(2.0, 2.0, 2.0, 2.0,),
            padding: UiRect::px(10.0, 10.0, 0.0, 0.0,),
        }
        Hovered
        BorderColor::from(Color::BLACK)
        BackgroundColor(NORMAL_BUTTON)
        Children [
            Text(label)
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow
        ]
    }
}

/// Restyle the button and update its label to reflect its current state.
///
/// The `Button` widget maintains a `Pressed` component while the button is held down, and the
/// picking backend keeps `Hovered` up to date. We simply read those each frame and pick a look.
fn update_button_appearance(
    mut buttons: Query<
        (
            &Hovered,
            Has<Pressed>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (With<ui_widgets::Button>, With<SpawnDockButton>),
    >,
) {
    for (hovered, pressed, mut color, mut border_color, _children) in &mut buttons {
        match (hovered.get(), pressed) {
            // Pressed (and, since you can only press what you're hovering, also hovered).
            (_, true) => {
                *color = PRESSED_BUTTON.into();
                border_color.set_all(RED);
            }
            // Hovered but not pressed.
            (true, false) => {
                *color = HOVERED_BUTTON.into();
                border_color.set_all(WHITE);
            }
            // Neither hovered nor pressed.
            (false, false) => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(BLACK);
            }
        }
    }
}
