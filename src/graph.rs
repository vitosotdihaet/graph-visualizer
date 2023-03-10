use bevy::prelude::*;

use std::{
    collections::HashMap,
    hash::{
        Hasher,
        Hash,
    }
};


const MINIMAL_F:        f32 = 1.;

const MINIMAL_DISTANCE: f32 = 50.;
const MAXIMUM_DISTANCE: f32 = 400.;
const RELATION_POWER:   f32 = 500000.;

const AIMING_DISTANCE:  f32 = 375.;
const ACCEPTABLE_FLUCT: f32 = 100.;
const FLUCT_POWER:      f32 = 0.1;

const SPRING_COEF:      f32 = 0.01;
const DEFAULT_MOVEMENT: f32 = 1.;


#[derive(Clone, Default, Debug, Component)]
pub struct Vertex {
    pub id: usize,
    pub connected: Vec<Vertex>,
    pub coords: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Vertex {
    pub fn relate(&self, other: &Self, only_low: bool) -> Vec2 {
        let c1 = self.coords;
        let c2 = other.coords;
        let nv = c1 - c2;

        let mut acceleration = Vec2::new(0., 0.);
        let mut d = nv.length();
        if let Some(a) = nv.try_normalize() { acceleration = a; }

        if d < MAXIMUM_DISTANCE || only_low {
            d = MINIMAL_DISTANCE.max(d);

            acceleration *= RELATION_POWER/(d*d);
        } else {
            acceleration *= SPRING_COEF * (AIMING_DISTANCE - d) + DEFAULT_MOVEMENT;
            if (AIMING_DISTANCE - d).abs() < ACCEPTABLE_FLUCT { acceleration *= FLUCT_POWER; }
        }
        if acceleration.length() < MINIMAL_F { acceleration *= 0.; }
        acceleration
    }

    pub fn add_acc(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
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
    pub arcs: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn add_vertex(&mut self, v: Vertex) {
        self.verticies.push(v.clone());

        if let Some(vec) = self.arcs.get_mut(&v.id) {
            for u in &v.connected {
                vec.push(u.id);
            }
        } else {
            self.arcs.insert(v.id, vec![]);
        }
    }

    pub fn add_arc(&mut self, i1: usize, i2: usize) {
        match self.arcs.get_mut(&i1) {
            Some(vec) => { vec.push(i2); },
            _ => { self.arcs.insert(i1, vec![i2]); }
        }
    }

    pub fn all_arcs(&mut self) -> Vec<(usize, usize)> {
        let mut arcs = Vec::new();
        for i in self.arcs.keys() {
            for j in self.arcs.get(i).unwrap() {
                arcs.push((*i, *j));
            }
        }
        arcs
    }

    pub fn len(&self) -> usize {
        self.verticies.len()
    }

    pub fn len_arcs(&self) -> usize {
        let mut sum = 0;
        for i in self.arcs.values() {
            sum += (*i).len();
        }
        sum
    }

    pub fn is_empty(&self) -> bool {
        self.verticies.len() == 0
    }
}
