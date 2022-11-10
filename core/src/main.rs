pub mod components;
pub mod plugins;
pub mod resources;
pub mod scenes;

use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use plugins::character::Character;

const BACKGROUND_COLOR: Color =
    Color::rgb(0.9, 0.9, 0.9);

fn main() {
    let mut app = App::new();
    // * Add LogPlugin
    app.add_plugins_with(
        DefaultPlugins,
        |plugins| plugins.disable::<LogPlugin>(),
    );
    // bevy_mod_debugdump::print_render_graph(
    //     &mut app,
    // );

    // * Add Inspector Plugin
    app.add_plugin(WorldInspectorPlugin::new());

    // * Add our own plugin here!
    app.add_plugin(Context);
    app.add_plugin(Character);

    app.add_system(bevy::window::close_on_esc);
    // * Run app
    app.run();
}
pub struct Context;

impl Plugin for Context {
    fn build(&self, app: &mut App) {
        // Camera
        app.add_startup_system(setup);
        app.insert_resource(ClearColor(
            BACKGROUND_COLOR,
        ));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default());
}
