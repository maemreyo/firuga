use bevy::prelude::*;
use crate::{
    components::_character::CCharacter,
    resources::character::_stats::Stats,
};

pub fn init(mut commands: Commands) {
    let character = CCharacter::new(Stats {
        health: 100.0,
        attack: 50.0,
        defense: 20.0,
        agile: 50.0,
        luck: 100.0,
    });
	println!("character {:?}", character);

    commands
        .spawn()
        .insert(character)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(
                    40.0, 130.0, 0.0,
                ),
                translation: Vec3::new(
                    500.0, -250.0, 1.0,
                ),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 0.5),
                ..default()
            },
            ..default()
        });
}
