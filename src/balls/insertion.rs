use crate::balls::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_3d::prelude::{Collider, Mass, RigidBody};
use rand::{self, Rng};

/// Iterate over every pair of balls and check if they should be merged
pub fn insertion_check(
    window: Query<&Window, With<PrimaryWindow>>,
    query: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    ball_templates: Res<BallTemplates>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut example_ball: Query<
        (&BallSize, &mut Transform, Entity, &mut Visibility),
        With<ExampleBall>,
    >,
) {
    let cursor = window.single().cursor_position();
    if cursor.is_none() {
        return;
    }
    let (camera, transform) = query.single();
    let ray = camera.viewport_to_world(transform, cursor.unwrap());
    if ray.is_none() {
        return;
    }
    let dist = ray
        .unwrap()
        .intersect_plane(Vec3::new(0.0, 12.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    if dist.is_none() {
        return;
    }
    let mut point = ray.unwrap().get_point(dist.unwrap());
    let mut example_ball = match example_ball.get_single_mut() {
        Ok(ex) => ex,
        Err(_) => return,
    };
    if point.x.abs() > 4.0 || point.z.abs() > 4.0 {
        *example_ball.3 = Visibility::Hidden;
        return;
    }
    *example_ball.3 = Visibility::Visible;
    let size = example_ball.0 .0;
    let radius = BallSize(size).radius() + 0.05;
    point.x = point.x.clamp(-4.0 + radius, 4.0 - radius);
    point.z = point.z.clamp(-4.0 + radius, 4.0 - radius);
    // Draw example ball and line
    gizmos.ray(point, Vec3::new(0.0, -12.2, 0.0), Color::GREEN);
    example_ball.1.translation = point;
    //Check if mouse pressed
    if buttons.just_pressed(MouseButton::Left) {
        // Spawn ball
        let mut new_ball = Ball::new(size);
        new_ball.spatial.transform.translation = point;
        new_ball.spatial.transform.rotation = example_ball.1.rotation;
        new_ball.spawn(&ball_templates, &mut commands);
        let new_size = if keys.pressed(KeyCode::ShiftLeft) {
            5
        } else {
            rand::thread_rng().gen_range(1..=4)
        };
        // Replace example ball
        commands.entity(example_ball.2).despawn_recursive();
        let mut new_example_ball = Ball::new(new_size);
        new_example_ball.spatial.transform.translation = Vec3::new(0.0, 4000.0, 0.0);
        new_example_ball.spatial.transform.rotation = random_quaternion();
        new_example_ball
            .spawn(&ball_templates, &mut commands)
            .remove::<Collider>()
            .remove::<RigidBody>()
            .insert(ExampleBall(()))
            .insert(Mass(1.0));
    }
}

/// Generate a random quaternion
fn random_quaternion() -> Quat {
    use std::f32::consts::PI;
    let u = rand::random::<f32>();
    let v = rand::random::<f32>();
    let w = rand::random::<f32>();
    Quat::from_xyzw(
        (1. - u).sqrt() * (2. * PI * v).sin(),
        (1. - u).sqrt() * (2. * PI * v).cos(),
        u.sqrt() * (2. * PI * w).sin(),
        u.sqrt() * (2. * PI * w).cos(),
    )
}
