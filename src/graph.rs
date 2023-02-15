use std::ops::{Index, IndexMut};

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Node {
    pub number: usize,
    pub connected: Vec<Node>,
}


#[derive(Component)]
pub struct Graph {
    pub nodes: Vec<Vec<Node>>,
}

impl Graph {
    pub fn add_node(&mut self, v: Node) {
        for u in &v.connected {
            self.nodes[&v].push(u.clone());
        }
    }
}


impl Index<&Node> for Vec<Node> {
    type Output = Node;

    fn index(&self, node: &Node) -> &Self::Output {
        return &self[node.number];
    }
}

impl IndexMut<&Node> for Vec<Node> {
    fn index_mut(&mut self, node: &Node) -> &mut Self::Output {
        return &mut self[node.number];
    }
}


impl Index<&Node> for Vec<Vec<Node>> {
    type Output = Vec<Node>;

    fn index(&self, node: &Node) -> &Self::Output {
        return &self[node.number];
    }
}

impl IndexMut<&Node> for Vec<Vec<Node>> {
    fn index_mut(&mut self, node: &Node) -> &mut Self::Output {
        return &mut self[node.number];
    }
}