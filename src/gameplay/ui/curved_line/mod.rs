pub mod bounded_to_entity;

use bevy::{color::palettes::css, prelude::*, render::render_resource::AsBindGroup};

use crate::gameplay::ui::curved_line::bounded_to_entity::{bounded_to_entities_beginning, bounded_to_entities_ending};

pub fn curved_line_plugin(app: &mut App) {
    app.add_plugins(UiMaterialPlugin::<CurvedLineMaterial>::default());
    app.add_systems(Update, (bounded_to_entities_beginning, bounded_to_entities_ending, synchronyze).chain());
}
#[derive(Debug, Component, Default, Clone, Reflect)]
pub struct CurvedLineParams {
    pub color: Color,
    pub point1: Vec2,
    pub point2: Vec2,
    pub point3: Vec2,
    pub point4: Vec2,
    pub width: f32,
}

// Regular bezier curve drawn on transparent layer.
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone, Default)]
pub struct CurvedLineMaterial {
    /// Color multiplied with the image
    #[uniform(0)]
    color: Vec4,
    #[uniform(1)]
    point1: Vec2,
    #[uniform(2)]
    point2: Vec2,
    #[uniform(3)]
    point3: Vec2,
    #[uniform(4)]
    point4: Vec2,
    #[uniform(5)]
    width: f32,
}

impl UiMaterial for CurvedLineMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/curved_line.wesl".into()
    }
}

pub fn curved_line() -> impl Scene {
    bsn! {
        #CUSTOM_NODE_MATERIAL
        Node{
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
        }
        MaterialNode<CurvedLineMaterial>(asset_value(CurvedLineMaterial{
            ..Default::default()
        }))
        CurvedLineParams{
            color: css::RED,
            point1: vec2(0.0, 10.0),
            point2: vec2(5.0, 5.0),
            point3: vec2(100.0, 5.0),
            point4: vec2(150.0, 10.0),
            width: 5.0
        }
        Pickable::IGNORE
    }
}

fn synchronyze(
    mut materials: ResMut<Assets<CurvedLineMaterial>>,
    q: Query<(&MaterialNode<CurvedLineMaterial>, &CurvedLineParams), Changed<CurvedLineParams>>,
) {
    for (handle, component) in &q {
        if let Some(mut m) = materials.get_mut(handle) {
            m.color = component.color.to_srgba().to_vec4();
            m.point1 = component.point1;
            m.point2 = component.point2;
            m.point3 = component.point3;
            m.point4 = component.point4;
            m.width = component.width;
        }
    }
}
