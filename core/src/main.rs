use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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
    // app.add_plugin();

    // * Run app
    app.run();
}
