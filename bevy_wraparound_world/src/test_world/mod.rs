use bevy::prelude::*;
mod plugin;
mod setup;
mod teleport_wraparound;

pub use plugin::WorldPlugin;

#[derive(Resource)]
pub struct WorldWidth(f32);
