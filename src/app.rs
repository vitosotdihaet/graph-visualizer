pub use bevy::{
    prelude::*,
    window::close_on_esc,
    render::{
        render_resource::SamplerDescriptor,
        texture::ImageSampler
    },
};

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
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

#[derive(Clone, PartialEq, Eq, Debug, Resource)]
pub enum MouseMode {
    Move,
    Build,
}

impl Default for MouseMode {
    fn default() -> Self {
        MouseMode::Move
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Resource)]
pub struct LastTouchedId(pub usize);


#[derive(Clone, PartialEq, Debug, Resource)]
pub struct CursorPosition(pub Vec2);

#[derive(Clone, PartialEq, Debug, Resource)]
pub struct CursorPositionToCenter(pub Vec2);


#[derive(Debug, Resource)]
pub struct ApplyForce(pub bool);


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
const _COLOR_HOVERED_VERTEX: Color = Color::rgb(0.65, 0.65, 0.65);
const _COLOR_PRESSED_VERTEX: Color = Color::rgb(0.3, 0.3, 0.3);

const ARC_WIDTH: f32 = 10.;

const KEYCODE_BUILD: KeyCode = KeyCode::B;
const KEYCODE_MOVE: KeyCode = KeyCode::M;
const KEYCODE_TOGGLE_FORCE: KeyCode = KeyCode::Space;

fn is_in_circle(p1: Vec2, p2: Vec2, r: f32) -> bool {
    (p2.x - r < p1.x && p1.x < p2.x + r) && (p2.y - r < p1.y && p1.y < p2.y + r)
}

#[derive(Component)]
pub struct Segment;

impl Segment {
    pub fn spawn_from_two_points(
        width: f32,
        color: Color,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let material_mesh = MaterialMesh2dBundle {
            material: materials.add(ColorMaterial::from(color)),
            mesh: Mesh2dHandle(meshes.add(Mesh::from(shape::Quad { size: Vec2::new(1., width), flip: false}))),
            ..Default::default()
        };

        commands.spawn(material_mesh).insert(Segment);
    }
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
            alignment: TextAlignment::CENTER,
        },
        ..Default::default()
    })
    .insert(HintText);
}

pub fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut apply_force: ResMut<ApplyForce>,
    mut lmb_mode: ResMut<MouseMode>,
    // mut _state: ResMut<State<GraphState>>,
) {
    if keys.just_pressed(KEYCODE_BUILD) {
        *lmb_mode = MouseMode::Build;
    } else if keys.just_pressed(KEYCODE_MOVE) {
        *lmb_mode = MouseMode::Move;
    } else if keys.just_pressed(KEYCODE_TOGGLE_FORCE) {
        apply_force.0 = !apply_force.0;
    }
}

pub fn update_mouse_coords(
    windows: Res<Windows>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
    mut cursor_position_to_center: ResMut<CursorPositionToCenter>,
) {
    let window = windows.get_primary().unwrap();
    let (w, h) = ((*window).width(), (*window).height());
    
    if let Some(moved_cursor) = cursor_moved.iter().last() {
        cursor_position.0 = moved_cursor.position; // from left bottom corner
        cursor_position_to_center.0 = cursor_position.0 - Vec2::new(w/2., h/2.);
    }
}

pub fn add_verticies(
    r: Res<Resources>,
    mouse: Res<Input<MouseButton>>,
    cursor_position_to_center: Res<CursorPositionToCenter>,
    mut c: Commands,
    mut g: ResMut<Graph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hint_text_query: Query<Entity, With<HintText>>,
) {
    let right_release = mouse.just_released(MouseButton::Right);

    if right_release {
        for e in &mut hint_text_query { c.entity(e).despawn(); } // despawn hint text

        let new_id = g.len();
        let vertex = Vertex { id: new_id, coords: cursor_position_to_center.0, ..Default::default() };

        g.add_vertex(vertex.clone());

        c.spawn(MaterialMesh2dBundle { // bg circle
            mesh: meshes.add(shape::Circle::new(VERTEX_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(COLOR_BG_VERTEX)),
            transform: Transform::from_translation(Vec3::new(cursor_position_to_center.0.x, cursor_position_to_center.0.y, 0.)),
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
}

pub fn update_verticies(
    mouse: Res<Input<MouseButton>>,
    lmb_mode: Res<MouseMode>,
    cursor_position_to_center: Res<CursorPositionToCenter>,
    apply_force: Res<ApplyForce>,
    mut c: Commands,
    mut g: ResMut<Graph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_touched_vertex_id: ResMut<LastTouchedId>,
    mut vertex_query: Query<&mut Transform, (With<Vertex>, With<Children>)>,
    mut segment_query: Query<&mut Transform, (With<Segment>, Without<Vertex>)>,
) {
    let left_click = mouse.just_pressed(MouseButton::Left);
    let left_release = mouse.just_released(MouseButton::Left);

    for (i, t) in zip(0..g.len(), &mut vertex_query) {
        g.verticies[i].coords = Vec2::new(t.translation.x, t.translation.y);
    }

    // iterate over all vertecies to add force to each vertex
    for (i, mut t) in zip(0..g.len(), &mut vertex_query) {
        let mut v1 = g.verticies[i].clone();

        // drag a vertex
        if *lmb_mode == MouseMode::Move {
            if left_click && is_in_circle(cursor_position_to_center.0, v1.coords, VERTEX_RADIUS) {
                last_touched_vertex_id.0 = i;
            } else if left_release {
                last_touched_vertex_id.0 = usize::MAX;
            } else if !left_click && last_touched_vertex_id.0 == i {
                t.translation = Vec3::new(cursor_position_to_center.0.x, cursor_position_to_center.0.y, 0.);
                continue;
            }
        } else if *lmb_mode == MouseMode::Build {
            if left_click && is_in_circle(cursor_position_to_center.0, v1.coords, VERTEX_RADIUS) {
                last_touched_vertex_id.0 = i;
            } else if left_release && is_in_circle(cursor_position_to_center.0, v1.coords, VERTEX_RADIUS) {
                g.add_arc(last_touched_vertex_id.0, v1.id);
                Segment::spawn_from_two_points(ARC_WIDTH, COLOR_BG_VERTEX, &mut c, &mut meshes, &mut materials);
                last_touched_vertex_id.0 = usize::MAX;
            }
        }

        if !apply_force.0 { continue; }
        for j in 0..g.len() {
            if i == j { continue; }
            let v2 = g.verticies[j].clone();
            let f = v1.relate(&v2);
            v1.add_acc(f);
        }
        v1.update();

        let (x, y) = (v1.coords.x, v1.coords.y); // bro i can't even unwrap Vec2 to tuple, literally 1984
        t.translation = Vec3::new(x, y, 0.);
    }

    for (arcs, mut t) in zip(g.all_arcs(), &mut segment_query) {
        let (i, j) = arcs;

        let p1 = g.verticies[i].coords;
        let p2 = g.verticies[j].coords;
        let sum = p1 + p2;
        let sub = p1 - p2;

        t.rotation = Quat::from_rotation_z((sub.y / sub.x).atan());
        t.scale = Vec3 { x: sub.length(), y: 1., z: 1.};
        t.translation = Vec3 { x: sum.x / 2., y: sum.y / 2., z: 0.};
        if i == j {
        } else {
        }
    }

}
