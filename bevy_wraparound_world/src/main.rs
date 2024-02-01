use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod materials;
mod player;
mod gravity_gun;

mod test_world;
// mod setup;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(Gravity(Vec2::NEG_Y * 50.0))
    // Default plugin
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    // Physics plugin
    .add_plugins(PhysicsPlugins::default())
    // My plugins
    .add_plugins(
        test_world::WorldPlugin { world_width: 1000.0 }
    )
    .run();
}
