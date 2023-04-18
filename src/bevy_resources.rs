use bevy::prelude::*;

use crate::graph::Vertex;


#[derive(Clone, PartialEq, Eq, Debug, Hash, Resource)]
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


#[derive(Debug, Resource)]
pub struct Clique(pub Vec<Vertex>);


#[derive(Resource)]
pub struct Resources {
    pub font: Handle<Font>,
}
