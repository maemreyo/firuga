use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

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
    app.add_plugin(HelloPlugin);

    // * Run app
    app.run();
}

pub struct HelloPlugin;
struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(
            Timer::from_seconds(2.0, true),
        ))
        .add_startup_system(add_people)
        .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name(
        "Elaina Proctor".to_string(),
    ));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished()
    {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}
