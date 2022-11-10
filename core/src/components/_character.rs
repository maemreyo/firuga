use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::resources::character::_stats::Stats;

#[derive(Component, Inspectable, Debug)]
pub struct CCharacter {
    pub base_stats: Stats,
}

impl CCharacter {
    pub fn new(base_stats: Stats) -> Self {
        Self { base_stats }
    }
}
