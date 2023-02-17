use bevy::prelude::*;

use std::{
    hash::{
        Hasher,
        Hash,
    },
    collections::HashMap,
};

const MINIMAL_F: f32 = 3.;
const MINIMAL_DISTANCE: f32 = 10.;
const MAXIMUM_DISTANCE: f32 = 400.;
const ACCEPTABLE_FLUCT: f32 = 100.;
const DEL: f32 = 375.;
const RELATION_POWER: f32 = 500000.;
const SPRING_COEF: f32 = 0.01;

#[derive(Component, Clone, Default, Debug)]
pub struct Vertex {
    pub id: usize,
    pub connected: Vec<Vertex>,
    pub coords: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Vertex {
    pub fn relate(&self, other: &Self) -> Vec2 {
        let c1 = self.coords;
        let c2 = other.coords;
        let nv = c1 - c2;

        let mut f = Vec2::new(0., 0.);
        let mut d = nv.length();
        if let Some(a) = nv.try_normalize() { f = a; }

        if d < MAXIMUM_DISTANCE {
            d = MINIMAL_DISTANCE.max(d);

            f *= RELATION_POWER/(d*d);
        } else {
            f *= SPRING_COEF*(DEL - d);
            if (DEL - d).abs() < ACCEPTABLE_FLUCT { f *= 0.25; }
        }
        if f.length() < MINIMAL_F { f *= 0.; }
        f
    }

    pub fn add_acc(&mut self, f: Vec2) {
        self.acceleration += f;
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;

        self.coords = self.coords.lerp(self.coords + self.velocity, 0.2);

        self.acceleration *= 0.;
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Vertex {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


#[derive(Resource, Default, Clone)]
pub struct Graph {
    pub verticies: Vec<Vertex>,
    pub arcs: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    pub fn add_vertex(&mut self, v: Vertex) {
        self.verticies.push(v.clone());

        if !self.arcs.contains_key(&v) {
            self.arcs.insert(v.clone(), Vec::new());
        }
        for u in &v.connected {   
            self.arcs.get_mut(&v).unwrap().push(u.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.verticies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.verticies.len() == 0
    }
}
