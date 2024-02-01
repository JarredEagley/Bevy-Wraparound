use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use super::WorldWidth;

// use crate::constants::WORLD_WIDTH;


pub struct TeleportWrapPlugin;
impl Plugin for TeleportWrapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, handle_wraparound)
        ;
    }
}

/// A Very, very simple teleport-based wraparound.
fn handle_wraparound(
    mut q_rigidbodies: Query<(&GlobalTransform, &mut Transform), With<RigidBody>>,
    r_width: Res<WorldWidth>,
) {
    const PADDING: f32 = 5.0;
    let width = r_width.0;

    for rigidbody in q_rigidbodies.iter_mut() {
        let (rb_global_tr, mut rb_tr) = rigidbody;
        let x_pos = rb_global_tr.translation().x;


        // Left border
        if x_pos < -PADDING {
            rb_tr.translation.x += width;
        }
        // Right border
        else if x_pos > width + PADDING {
            rb_tr.translation.x -= width;
        }
    }
}
