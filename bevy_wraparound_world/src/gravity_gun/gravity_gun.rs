use bevy::{prelude::*, input::mouse::MouseButtonInput, window::PrimaryWindow};
use bevy_xpbd_2d::prelude::*;

use crate::player::Player;

use super::grav_gun_field::GravGunField;

/// A gravity gun that can be attached to a player.
/// Can track one gravity gun field, which then can be used to manipulate physics objects.
#[derive(Component, Debug, Clone)]
pub struct GravityGun(pub Option<Entity>);


/// Handle gravity gun control.
/// There's probably a better way to structure this.
pub fn mouse_event (
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut q_gravity_gun: Query<&mut GravityGun>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    q_physics_objects: Query<(Entity, &GlobalTransform), (With<RigidBody>, Without<Player>)>,
    mut commands: Commands
) {
    for ev in mousebtn_evr.read() { 
        if ev.state.is_pressed() {
            // Get mouse cursor position in world coordinates.
            let cursor_screen_pos = q_windows.single().cursor_position();
            if cursor_screen_pos.is_none() {
                return;
            }
            let cursor_screen_pos = cursor_screen_pos.unwrap();

            let (camera, camera_tr) = q_camera.single();
            let cursor_world_pos = camera.viewport_to_world_2d(camera_tr, cursor_screen_pos).unwrap();

            // If we have an existing field, drop it.
            let mut gravity_gun = q_gravity_gun.single_mut();
            if let Some(field_entity) = gravity_gun.0 {
                commands.entity(field_entity).despawn();
                gravity_gun.0 = None;

                println!("Gravity gun field dropped.");

                // Early return.
                return;
            } 

            // Else, create a new field and track it.

            // Select all physics-enabled entities within some distance.
            const RANGE_SQR: f32 = 100.0;
            let mut selected_entities: Vec<Entity> = Vec::new();
            for (entity, obj_position) in q_physics_objects.iter() {
                if obj_position.translation().distance_squared(cursor_world_pos.extend(0.0)) < RANGE_SQR {
                    selected_entities.push(entity);
                } 
            }
            println!("Found {} entities!", selected_entities.len());
            if !selected_entities.is_empty() {
                let new_field = commands.spawn((
                    GravGunField{
                        entities: selected_entities,
                    },
                    TransformBundle {
                        local: Transform::from_translation(cursor_world_pos.extend(0.0)),
                        ..default()
                    }
                )).id();

                // Track the new field.
                gravity_gun.0 = Some(new_field);
            }
        }
    }    
}


