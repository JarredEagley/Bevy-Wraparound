use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct GravGunField {
    pub entities: Vec<Entity>,
}

pub fn update_field_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut q_fields: Query<&mut Transform, With<GravGunField>>,
) {
    // Get mouse cursor position in world coordinates.
    let cursor_screen_pos = q_windows.single().cursor_position();
    if cursor_screen_pos.is_none() {
        return;
    }
    let cursor_screen_pos = cursor_screen_pos.unwrap();

    let (camera, camera_tr) = q_camera.single();
    let cursor_world_pos = camera.viewport_to_world_2d(camera_tr, cursor_screen_pos).unwrap();

    for mut field_tr in q_fields.iter_mut() {
        // Follow mouse cursor.
        field_tr.translation = cursor_world_pos.extend(0.0);
    }
}

pub fn update_field_entities(
    q_fields: Query<(&GlobalTransform, &GravGunField), With<GravGunField>>,
    mut q_phys: Query<(&GlobalTransform, &LinearVelocity, &mut ExternalImpulse), With<LinearVelocity>>,
    // time: Res<Time>,
    // mut commands: Commands
) {
    let field = q_fields.get_single();
    if field.is_err() {
        return;
    };
    let (field_tr, field) = field.unwrap();
    
    for entity in field.entities.iter() {
        let maybe_components = q_phys.get_mut(entity.clone());
        if let Ok((entity_tr, entity_vel, mut entity_force)) = maybe_components {

            let position_delta =  field_tr.translation() - entity_tr.translation();

            let desired_velocity = position_delta.truncate() * position_delta.length() * 0.1;

            let velocity_delta = desired_velocity - entity_vel.0;

            entity_force.apply_impulse(velocity_delta);
            // entity_vel.0 += velocity_delta;
            // println!("Updated an entity's velocity {}", position_delta);
        } 
        else {
            // println!("Failed to update an entity...");
        }
    }
}
