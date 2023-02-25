use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

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