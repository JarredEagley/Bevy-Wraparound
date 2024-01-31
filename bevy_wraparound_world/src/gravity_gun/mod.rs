// This is a little for-fun plugin that lets you throw around physics objects.

use bevy::prelude::*;
mod grav_gun_field;
mod gravity_gun;

pub use gravity_gun::GravityGun;

/// Click on a physics object to control it with your mouse.  I hope.
pub struct GravityGunPlugin;
impl Plugin for GravityGunPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
            gravity_gun::mouse_event,
            grav_gun_field::update_field_position,
            grav_gun_field::update_field_entities,
        ))
        ;
    }
}
