use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

const DEFAULT_WALK_SPEED: f32 = 30.0;
const DEFAULT_JUMP_STRENGTH: f32 = 50.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (update_player_controllers, update_camera_zoom))
        ;
    }
}

/// Right now just holds player name.
#[derive(Component, Debug, Clone)]
pub struct Player {
    pub name: String,
}
impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Default Player".to_string()
        }
    }
}

/// Designates a player as keyboard controllable, and holds info about the how the character handles.
#[derive(Component, Debug, Clone)]
pub struct PlayerController {
    pub walk_speed: f32,
    pub jump_force: f32,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController { walk_speed: DEFAULT_WALK_SPEED, jump_force: DEFAULT_JUMP_STRENGTH }
    }
}

pub fn update_player_controllers(
    mut q_players: Query<(&mut LinearVelocity, &mut ExternalImpulse, &mut ExternalForce, &PlayerController ), With<PlayerController>>,
    input: Res<Input<KeyCode>>,
) {
    let mut input_x = 0.0;
    if input.pressed(KeyCode::A) {
       input_x -= 1.0; 
    };
    if input.pressed(KeyCode::D) {
        input_x += 1.0;
    };

    let jump = input.just_pressed(KeyCode::Space);

    let grounded = true; // Would want to do more advanced checks theoretically, but I don't care.

    // Now do the real updates.
    for (
        player_vel, 
        mut player_impulse, 
        _, 
        player_controller,
    ) in q_players.iter_mut() {
        if !grounded {
            continue;
        }

        // How fast do we want to move on the x axis?
        let desired_x_vel = player_controller.walk_speed * input_x;
        let current_x_vel = player_vel.x;
        let x_vel_delta = desired_x_vel - current_x_vel;
        player_impulse.apply_impulse(Vec2::new(x_vel_delta, 0.0));

        // println!("Desired: {}\nCurrent: {}\nDelta: {}", desired_x_vel, current_x_vel, x_vel_delta);

        if jump {
            player_impulse.apply_impulse(Vec2::new(0.0, player_controller.jump_force));
        }
    }
}

fn update_camera_zoom(
    mut q_camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    input: Res<Input<KeyCode>>,
) {
    const ZOOM_SENSITIVITY: f32 = 0.1;

    let mut projection = q_camera.single_mut();

    let mut zoom_input = 1.0;
    if input.pressed(KeyCode::Q) {
        zoom_input += ZOOM_SENSITIVITY;
    };
    if input.pressed(KeyCode::E) {
        zoom_input -= ZOOM_SENSITIVITY;
    };

    let new_scale = projection.scale * zoom_input;
    projection.scale = new_scale;
}

/// Only missing transform, because I want it to be compatible with sprite bundle.
#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub player: Player,
    pub controller: PlayerController,
    pub force: ExternalForce,
    pub impulse: ExternalImpulse,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle { 
            player: Player::default(),
            controller: PlayerController::default(),
            force: ExternalForce::new(Vec2::ZERO),
            impulse: ExternalImpulse::new(Vec2::ZERO),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::default(),
            locked_axes: LockedAxes::new().lock_rotation(),
        }
    }
}