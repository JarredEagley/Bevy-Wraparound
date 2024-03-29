use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{constants::WORLD_WIDTH, player::{PlayerBundle, PlayerController}, gravity_gun};


/// Small helper function for spawning cuboids in a more streamlined manner.
fn spawn_box (
    commands: &mut Commands,
    scale: Vec3,
    pos: Vec3,
    color: Color,
    rb: RigidBody,
) -> Entity {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(scale.truncate()),
                ..default()
            },
            transform: Transform::from_translation(pos),
            ..default()
        },
        rb,
        Collider::cuboid(scale.x, scale.y),
    )).id()
}

pub fn setup_world(
    mut commands: Commands
) {
    let floor_scale = Vec3::new(WORLD_WIDTH, 100.0, 0.0);
    let floor_translation = Vec3::new(WORLD_WIDTH*0.5, -50.0, 0.0);

    spawn_box(&mut commands, floor_scale, floor_translation, Color::TEAL, RigidBody::Static);
}

pub fn setup_props(
    mut commands: Commands
) {
    let box_scale = Vec3::new(7.0, 12.0, 0.0);
    let box_translation = Vec3::new(20.0, 20.0, 0.0);

    spawn_box(&mut commands, box_scale, box_translation, Color::PURPLE, RigidBody::Dynamic);
}

pub fn setup_player(
    mut commands: Commands
) {
    let player_spawn_pos = Vec3::new(10.0, 50.0, 0.0);
    let player_scale = Vec3::new(5.0, 10.0, 0.0);

    let player_entity = commands.spawn(SpriteBundle {
        transform: Transform::from_translation(player_spawn_pos),
        sprite: Sprite {
            custom_size: Some(player_scale.truncate()),
            color: Color::PURPLE, // I don't feel like setting up textures right now lol
            ..default()
        },
        ..default()
    })
    .insert(PlayerBundle {
        collider: Collider::cuboid(player_scale.x, player_scale.y),
        controller: PlayerController {
            walk_speed: 60.0,
            jump_force: 50.0,
        },
        ..default()
    })
    .insert(gravity_gun::GravityGun(None))
    .id();

    // Camera is blatantly parented to the player.
    commands.spawn(Camera2dBundle {
        ..default()
    }).set_parent(player_entity);
}
