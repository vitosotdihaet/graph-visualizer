use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use std::{
    iter::zip,
    path::Path,
};

use crate::graph::*;
use crate::segment::*;
use crate::bevy_resources::*;
use crate::misc::*;


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


pub fn startup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Resources { font: asset_server.load(Path::new("fonts").join(FONT_NAME)) });
}

pub fn init(
    resources: Res<Resources>,
    mut commands: Commands,
) {
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "To create new vertex press RMB".to_owned(),
                style: TextStyle {
                    font: resources.font.clone(),
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
    mut _state: ResMut<State<GraphState>>,
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
    resources: Res<Resources>,
    mouse: Res<Input<MouseButton>>,
    cursor_position_to_center: Res<CursorPositionToCenter>,
    mut commands: Commands,
    mut graph: ResMut<Graph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hint_text_query: Query<Entity, With<HintText>>,
) {
    let right_release = mouse.just_released(MouseButton::Right);

    if right_release {
        for e in &mut hint_text_query { commands.entity(e).despawn(); } // despawn hint text

        let new_id = graph.len();
        let vertex = Vertex { id: new_id, coords: cursor_position_to_center.0, ..Default::default() };

        graph.add_vertex(vertex.clone());

        commands.spawn(MaterialMesh2dBundle { // bg circle
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
        .with_children(|parent| { // number
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("{}", new_id),
                            style: TextStyle {
                                font: resources.font.clone(),
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
    mut commands: Commands,
    mut graph: ResMut<Graph>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_touched_vertex_id: ResMut<LastTouchedId>,
    mut vertex_transform_query: Query<&mut Transform, (With<Vertex>, With<Children>)>,
    mut segment_transform_query: Query<&mut Transform, (With<Segment>, Without<Vertex>)>,
) {
    let left_click = mouse.just_pressed(MouseButton::Left);
    let left_release = mouse.just_released(MouseButton::Left);

    for (i, t) in zip(0..graph.len(), &mut vertex_transform_query) {
        graph.verticies[i].coords = Vec2::new(t.translation.x, t.translation.y);
    }

    // iterate over all vertecies to add force to each vertex
    for (i, mut t) in zip(0..graph.len(), &mut vertex_transform_query) {
        let mut v1 = graph.verticies[i].clone();

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
                graph.add_arc(last_touched_vertex_id.0, v1.id);
                Segment::spawn_from_two_points(ARC_WIDTH, COLOR_BG_VERTEX, &mut commands, &mut meshes, &mut materials);
                last_touched_vertex_id.0 = usize::MAX;
            }
        }

        if !apply_force.0 { continue; }
        for v2 in graph.verticies.clone() {
            if v1 == v2 { continue; }
            let only_low = !(graph.arcs.get(&v1.id).unwrap().contains(&v2.id) || graph.arcs.get(&v2.id).unwrap().contains(&v1.id));
            let acceleration = v1.relate(&v2, only_low);
            v1.add_acc(acceleration);
        }
        v1.update();

        let (x, y) = (v1.coords.x, v1.coords.y); // bro i can't even unwrap Vec2 to tuple, literally 1984
        t.translation = Vec3::new(x, y, 0.);
    }

    for (arcs, mut t) in zip(graph.all_arcs(), &mut segment_transform_query) {
        let (i, j) = arcs;

        let p1 = graph.verticies[i].coords;
        let p2 = graph.verticies[j].coords;
        let sum = p1 + p2;
        let sub = p1 - p2;

        t.rotation = Quat::from_rotation_z((sub.y / sub.x).atan());
        t.scale = Vec3 { x: sub.length(), y: 1., z: 1.};
        t.translation = Vec3 { x: sum.x / 2., y: sum.y / 2., z: 0.};
        // if i == j { // maybe needs some processing
        // } else {
        // }
    }

}
