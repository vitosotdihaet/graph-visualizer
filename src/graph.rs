use bevy::prelude::*;

use std::{
    hash::{
        Hasher,
        Hash,
    },
    collections::HashMap,
};

#[derive(Component, Clone, Default, Eq, Debug)]
pub struct Vertex {
    pub id: usize,
    pub connected: Vec<Vertex>,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


#[derive(Resource, Default, Clone)]
pub struct Graph {
    pub vertecies: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    pub fn add_vertex(&mut self, v: Vertex) {
        if !self.vertecies.contains_key(&v) {
            self.vertecies.insert(v.clone(), Vec::new());
        }
        for u in &v.connected {   
            self.vertecies.get_mut(&v).unwrap().push(u.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.vertecies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vertecies.len() == 0
    }
}
