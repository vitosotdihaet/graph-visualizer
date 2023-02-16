use bevy::prelude::*;

use std::{
    hash::{
        Hasher,
        Hash,
    },
    collections::HashMap,
};

const MINIMAL_DISTANCE: f32 = 10.;
const MAXIMUM_DISTANCE: f32 = 400.;
const ACCEPTABLE_FLUCT: f32 = 200.;
const DEL: f32 = 350.;
const RELATION_POWER: f32 = 2000.;
const SPRING_COEF: f32 = 0.5;

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

        // println!("c1: {}, c2: {}", c1, c2);

        let mut f = Vec2::new(0., 0.);
        let mut d = nv.length();
        println!("dist from {} to {}: {}; MAX IS {}, so {}", self.id, other.id, d, MAXIMUM_DISTANCE, d < MAXIMUM_DISTANCE);
        if let Some(a) = nv.try_normalize() { f = a; }

        if d < MAXIMUM_DISTANCE {
            println!("NOT MAXXXXXXXXXXXXXXXXXXXX {{");
            d = MINIMAL_DISTANCE.max(d);

            f *= RELATION_POWER/(d*d);
            println!("\t f = {}\n}}", f);
        } else {
            println!("KUDAAAAAAAAAAAAAAAAAAAAAAA {{");
            f *= SPRING_COEF*(DEL - d);
            if (DEL - d).abs() < ACCEPTABLE_FLUCT { f *= 0.; }
            println!("\t f = {}\n}}", f);
        }
        f
    }

    pub fn add_acc(&mut self, f: Vec2) {
        self.acceleration += f;
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        let wanted = self.coords + self.velocity;

        self.coords = Vec2::new(self.coords.x + (wanted.x - self.coords.x) * 0.3, self.coords.y + (wanted.y - self.coords.y) * 0.3);
        self.coords.lerp(self.coords + self.velocity, 0.3);

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
