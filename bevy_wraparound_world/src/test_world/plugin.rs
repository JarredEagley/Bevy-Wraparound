use bevy::{prelude::*, sprite::Material2dPlugin};

use crate::materials::*;
use crate::player::PlayerPlugin;
use crate::gravity_gun::GravityGunPlugin;

use super::WorldWidth;
use super::teleport_wraparound::TeleportWrapPlugin;
use super::setup;

pub struct WorldPlugin {
    pub world_width: f32,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(WorldWidth(self.world_width))
        // Custom materials
        .add_plugins(
            Material2dPlugin::<BackgroundMaterial>::default()
        )
        // Gameplay plugins
        .add_plugins((
            PlayerPlugin,
            GravityGunPlugin,
            TeleportWrapPlugin, // Dead simple wraparound implementation.
        ))
        // World setup
        .add_systems(Startup, (
            setup::setup_world,
            setup::setup_player,
            setup::setup_props,
        ))
        ;
    }
}