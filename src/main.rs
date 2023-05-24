use bevy::{
    prelude::*,
    window::{close_on_esc, WindowDescriptor},
};

use graph_visualizer::graph::Graph;
use graph_visualizer::app::*;
use graph_visualizer::bevy_resources::*;

fn main() {
    let windows_info: WindowDescriptor = WindowDescriptor {
        width: 1000.0,
        height: 1000.0,
        title: "Graph visualizer".to_owned(),
        resizable: true,
        ..Default::default()
    };

    App::new()
        .init_resource::<Graph>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseMode::Move)
        .insert_resource(ApplyForce(true))
        .insert_resource(CursorPosition(Vec2::new(0., 0.)))
        .insert_resource(CursorPositionToCenter(Vec2::new(0., 0.)))
        .insert_resource(LastTouchedId(usize::MAX))
        .insert_resource(Clique(Vec::new()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: windows_info,
            ..Default::default()
        }))
        .add_system(close_on_esc)
        .add_startup_system(startup)
        .add_state(GraphState::Graph)
        .add_system_set(SystemSet::on_enter(GraphState::Graph).with_system(init))
        .add_system_set(SystemSet::on_update(GraphState::Graph)
            .with_system(handle_input)
            .with_system(update_mouse_coords)
            .with_system(add_verticies)
            .with_system(update_verticies)
            .with_system(update_text)
            .with_system(print_max_clique)
        )
        .run();
}
