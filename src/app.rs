pub use bevy::{
    prelude::*,
    window::close_on_esc,
    render::{
        render_resource::SamplerDescriptor,
        texture::ImageSampler
    },
};

use std::{
    path::Path,
};

use crate::graph::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GraphState {
    Graph,
    Algorithm,
}

#[derive(Resource)]
pub struct Resources {
    font: Handle<Font>,
}

#[derive(Component)]
pub struct StartingText;


const FONT_NAME: &'static str = "FOTNewRodin Pro B.otf";

const FONT_SIZE: f32 = 60.0;
const INIT_TEXT_FONT_SIZE: f32 = 40.0;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const INIT_TEXT_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
const COLOR_NODE: Color = Color::rgb(0.7, 0.7, 0.7);
const COLOR_HOVERED_NODE: Color = Color::rgb(0.8, 0.8, 0.8);
const COLOR_PRESSED_NODE: Color = Color::rgb(0.3, 0.3, 0.3);

pub fn startup(
    a: Res<AssetServer>,
    mut c: Commands,
) {
    c.spawn(Camera2dBundle::default());

    c.insert_resource(Resources { font: a.load(Path::new("fonts").join(FONT_NAME)) });
}

pub fn init(
    r: Res<Resources>,
    mut c: Commands,
) {
    c.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "To create a new vertex press RMB".to_owned(),
                style: TextStyle {
                    font: r.font.clone(),
                    font_size: INIT_TEXT_FONT_SIZE,
                    color: INIT_TEXT_COLOR,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        ..Default::default()
    })
    .insert(StartingText);
}

pub fn app(
    r: Res<Resources>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut c: Commands,
    mut g: ResMut<Graph>,
    mut state: ResMut<State<GraphState>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_position: Local<Vec2>,
    mut text_query: Query<Entity, With<StartingText>>,
    mut vertex_query: Query<&mut Transform, With<Vertex>>,
    mut vertex_interaction_query: Query<&Interaction, (Changed<Interaction>, With<Vertex>)>,
) {
    if let Some(moved_cursor) = cursor_moved.iter().last() {
        *cursor_position = moved_cursor.position;    
    }

    let left_click = mouse_button_input.just_released(MouseButton::Left);
    let right_click = mouse_button_input.just_released(MouseButton::Right);

    // create new vertex
    if right_click {
        for e in &mut text_query { c.entity(e).despawn(); }
        println!("Wow!");
        let new_id = (*g).len();
        (*g).add_vertex(Vertex { id: new_id, ..Default::default() });
    }

    // println!("{}!", (*g).len());
    // for (i, _) in (*g).vertecies.clone() {
    //     println!("{:?}", i);
    // }
}
