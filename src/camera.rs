use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

#[derive(Component)]
pub struct CameraAngle(f32, f32);

const CAMERA_DIST: f32 = 30.0;

pub fn new_camera() -> (Camera3dBundle, CameraAngle) {
    (
        Camera3dBundle {
            transform: Transform::from_xyz(CAMERA_DIST, 6.0, 0.0)
                .looking_at(Vec3::new(0.0, 6.0, 0.0), Vec3::Y),
            ..default()
        },
        CameraAngle(0.0, 0.0),
    )
}

pub fn orbit_camera(
    window: Query<&Window, With<PrimaryWindow>>,
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraAngle)>,
) {
    let window = window.get_single().unwrap();
    let mut rotation_move = Vec2::ZERO;
    for ev in ev_motion.read() {
        rotation_move += ev.delta
    }
    if !input_mouse.pressed(MouseButton::Left) {
        return;
    }
    for (mut transform, mut pos) in query.iter_mut() {
        if rotation_move.length_squared() > 0.0 {
            let delta_x = rotation_move.x / window.width() * PI * 2.0;
            let delta_y = rotation_move.y / window.height() * PI;
            pos.0 -= delta_x;
            pos.1 -= delta_y;
            pos.1 = pos.1.clamp(-PI / 2.0, PI / 2.0);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, pos.0, pos.1, 0.0);
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, CAMERA_DIST)) + Vec3::new(0.0, 6.0, 0.0);
        }
    }
}
