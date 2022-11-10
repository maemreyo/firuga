use bevy::prelude::*;

pub mod _initiate;

pub struct Character;

impl Plugin for Character {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     SystemSet::new()
        //         .with_system(_initiate::init),
        // );
		app.add_system(_initiate::init);
    }
}
