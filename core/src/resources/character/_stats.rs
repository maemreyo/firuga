use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Inspectable
)]
pub struct Stats {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub agile: f32,
    pub luck: f32,
}
