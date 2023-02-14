pub use bevy::{
    prelude::*,
    window::close_on_esc,
    render::{
        render_resource::SamplerDescriptor,
        texture::ImageSampler
    }
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum State {
    Graph,
    Algorithm,
}

#[derive(Component)]
pub struct NodeComponent;

const FONT_SIZE: f32 = 60.0;
const INPUT_TEXT_FONT_SIZE: f32 = 120.0;

const COLOR_NODE: Color = Color::rgb(0.7, 0.7, 0.7);
const COLOR_HOVERED_NODE: Color = Color::rgb(0.8, 0.8, 0.8);
const COLOR_PRESSED_NODE: Color = Color::rgb(0.3, 0.3, 0.3);

pub fn startup(
    mut c: Commands,
) {
    c.spawn(Camera2dBundle::default());

}

pub fn app(
    mut c: Commands,
) {
    
}