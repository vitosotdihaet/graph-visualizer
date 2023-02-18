use bevy::prelude::*;

use std::{
    hash::{
        Hasher,
        Hash,
    },
    collections::HashMap,
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
            f *= SPRING_COEF * (AIMING_DISTANCE - d) + DEFAULT_MOVEMENT;
            if (AIMING_DISTANCE - d).abs() < ACCEPTABLE_FLUCT { f *= FLUCT_POWER; }
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
    pub arcs: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn add_vertex(&mut self, v: Vertex) {
        self.verticies.push(v.clone());

        for u in &v.connected {
            self.arcs.get_mut(&v.id).unwrap().push(u.id);
        }
    }

    pub fn add_arc(&mut self, k: usize, v: usize) {
        if !self.arcs.contains_key(&k) {
            self.arcs.insert(k, vec![v]);
        } else {
            self.arcs.get_mut(&k).unwrap().push(v);
        }
    }

    pub fn all_arcs(&mut self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for i in self.arcs.keys() {
            for j in self.arcs.get(i).unwrap() {
                v.push((*i, *j));
            }
        }
        v
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
