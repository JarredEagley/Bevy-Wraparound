use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod setup;
mod player;
mod constants;
mod gravity_gun;
mod teleport_wraparound; // Simple wraparound using a teleport


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 1.0)))
    .insert_resource(Gravity(Vec2::NEG_Y * 50.0))
    // Default plugin
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    // Physics plugin
    .add_plugins(PhysicsPlugins::default())
    // My plugins
    .add_plugins((
        player::PlayerPlugin,
        gravity_gun::GravityGunPlugin,
        teleport_wraparound::TeleportWrapPlugin, // Dead simple wraparound implementation.
    ))
    // Simply world setup
    .add_systems(Startup, (
        setup::setup_world,
        setup::setup_player,
        setup::setup_props,
    ))
    .run();
}
