pub use bevy::{
    prelude::*,
    window::close_on_esc,
    render::{
        render_resource::SamplerDescriptor,
        texture::ImageSampler
    },
};

use bevy::sprite::MaterialMesh2dBundle;
use std::{
    iter::zip,
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
pub struct HintText;


const FONT_NAME: &str = "FOTNewRodin Pro B.otf";

const VERTEX_RADIUS: f32 = 50.;

const FONT_SIZE: f32 = 60.;
const FONT_INIT_TEXT_SIZE: f32 = 40.;

const COLOR_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);
const COLOR_INIT_TEXT: Color = Color::rgb(0.4, 0.4, 0.4);

const COLOR_FG_VERTEX: Color = Color::rgb(0.5, 0.5, 0.5);
const COLOR_BG_VERTEX: Color = Color::rgb(0.2, 0.2, 0.2);
const COLOR_HOVERED_VERTEX: Color = Color::rgb(0.65, 0.65, 0.65);
const COLOR_PRESSED_VERTEX: Color = Color::rgb(0.3, 0.3, 0.3);

fn is_in_circle(p1: Vec2, p2: Vec2, r: f32) -> bool {
    (p2.x - r < p1.x && p1.x < p2.x + r) && (p2.y - r < p1.y && p1.y < p2.y + r)
}

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
                    font_size: FONT_INIT_TEXT_SIZE,
                    color: COLOR_INIT_TEXT,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        },
        ..Default::default()
    })
    .insert(HintText);
}

pub fn app(
    r: Res<Resources>,
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut c: Commands,
    mut g: ResMut<Graph>,
    mut state: ResMut<State<GraphState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_position: Local<Vec2>,
    mut last_dragged_id: Local<usize>,
    mut text_query: Query<Entity, With<HintText>>,
    mut vertex_query: Query<&mut Transform, (With<Vertex>, With<Children>)>,
) {
    let window = windows.get_primary().unwrap();
    let (w, h) = ((*window).width(), (*window).height());
    
    if let Some(moved_cursor) = cursor_moved.iter().last() {
        *cursor_position = moved_cursor.position;    
    }

    let left_click = mouse_button_input.just_pressed(MouseButton::Left);

    let left_release = mouse_button_input.just_released(MouseButton::Left);
    let right_release = mouse_button_input.just_released(MouseButton::Right);

    let (cx, cy) = ((*cursor_position).x - w/2., (*cursor_position).y - h/2.);

    // create new vertex
    if right_release {
        for e in &mut text_query { c.entity(e).despawn(); *last_dragged_id = usize::MAX; } // despawn hint text

        let new_id = (*g).len();
        let vertex = Vertex { id: new_id, coords: Vec2::new(cx, cy), ..Default::default() };

        (*g).add_vertex(vertex.clone());

        c.spawn(MaterialMesh2dBundle { // bg circle
            mesh: meshes.add(shape::Circle::new(VERTEX_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(COLOR_BG_VERTEX)),
            transform: Transform::from_translation(Vec3::new(cx, cy, 0.)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle { // fg circle
                mesh: meshes.add(shape::Circle::new(VERTEX_RADIUS * 0.8).into()).into(),
                material: materials.add(ColorMaterial::from(COLOR_FG_VERTEX)),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("{}", new_id),
                            style: TextStyle {
                                font: r.font.clone(),
                                font_size: FONT_SIZE,
                                color: COLOR_TEXT,
                            }    
                        }
                    ],
                    alignment: TextAlignment::CENTER
                },
                transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                ..Default::default()
            });
        })
        .insert(vertex);
    }

    for (i, t) in zip(0..(*g).len(), &mut vertex_query) {
        (*g).verticies[i].coords = Vec2::new(t.translation.x, t.translation.y);
    }

    // iterate over all arcs to give force to each vertex
    for (i, mut t) in zip(0..(*g).len(), &mut vertex_query) {
        let mut v1 = (*g).verticies[i].clone();

        // drag a vertex
        if left_click && is_in_circle(*cursor_position - Vec2::new(w/2., h/2.), v1.coords, VERTEX_RADIUS) {
            *last_dragged_id = i;
        } else if left_release {
            *last_dragged_id = usize::MAX;
        } else if !left_click && *last_dragged_id == i {
            (*t).translation = Vec3::new(cx, cy, 0.);
            continue;
        }

        for j in 0..(*g).len() {
            if i == j { continue; }
            let v2 = &mut (*g).verticies[j];
            let f = v1.relate(&v2);
            v1.add_acc(f);
        }
        v1.update();

        let (x, y) = (v1.coords.x, v1.coords.y); // bro i can't even unwrap Vec2 to tuple, literally 1984
        (*t).translation = Vec3::new(x, y, 0.);
    }

}
