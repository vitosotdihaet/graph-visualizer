pub use bevy::{
    prelude::*,
    window::close_on_esc,
    render::{
        render_resource::SamplerDescriptor,
        texture::ImageSampler
    }
};

use std::{
    path::Path,
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum State {
    Graph,
    Algorithm,
}

#[derive(Resource)]
pub struct Resources {
    font: Handle<Font>,
}


const FONT_NAME: &'static str = "FOTNewRodin Pro B.otf";

const FONT_SIZE: f32 = 60.0;
const INPUT_TEXT_FONT_SIZE: f32 = 120.0;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const COLOR_NODE: Color = Color::rgb(0.7, 0.7, 0.7);
const COLOR_HOVERED_NODE: Color = Color::rgb(0.8, 0.8, 0.8);
const COLOR_PRESSED_NODE: Color = Color::rgb(0.3, 0.3, 0.3);

pub fn startup(
    a: Res<AssetServer>,
    mut c: Commands,
) {
    c.spawn(Camera2dBundle::default());

    c.insert_resource(Resources { font: a.load(Path::new("fonts").join(FONT_NAME)) });

}

pub fn init(
    r: Res<Resources>,
    mut c: Commands,
) {
    c.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Hello, Graphs!".to_owned(),
                style: TextStyle {
                    font: r.font.clone(),
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
        },
        transform: Transform {
            translation: Vec3 {
                x: -225.,
                y: 225.,
                z: 1.,
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn app(
    
) {

}