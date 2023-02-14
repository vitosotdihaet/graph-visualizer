use graphs::app::*;

fn main() {
    let windows_info: WindowDescriptor = WindowDescriptor {
        width: 1000.0,
        height: 1000.0,
        title: "Graph visualizer".to_owned(),
        resizable: true,
        ..Default::default()
    };

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: windows_info,
            ..Default::default()
        }))
        .add_system(close_on_esc)
        .add_startup_system(startup)
        .add_state(State::Graph)
        .add_system_set(SystemSet::on_update(State::Graph).with_system(app))
        .run();
}
