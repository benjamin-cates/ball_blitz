use crate::balls::*;
use crate::input::BallSpawnUpdate;
use crate::input::CursorChangeType;
use crate::points;
use crate::setup::BoxSize;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{Collider, Mass, RigidBody};
use rand::{self, Rng};

/// Iterate over every pair of balls and check if they should be merged
pub fn insertion_check(
    mut event: EventReader<BallSpawnUpdate>,
    keys: Res<Input<KeyCode>>,
    ball_templates: Res<BallTemplates>,
    box_size: Res<BoxSize>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut example_ball: Query<
        (&BallSize, &mut Transform, Entity, &mut Visibility),
        With<ExampleBall>,
    >,
    mut points: ResMut<points::GamePoints>,
) {
    let Some(BallSpawnUpdate {
        cursor_type,
        position,
    }) = event.read().next().cloned()
    else {
        return;
    };
    event.clear();

    let mut example_ball = match example_ball.get_single_mut() {
        Ok(ex) => ex,
        Err(_) => return,
    };
    let mut position = match position {
        Some(pos) => pos,
        None => {
            *example_ball.3 = Visibility::Hidden;
            return;
        }
    };
    *example_ball.3 = Visibility::Visible;
    let size = example_ball.0 .0;
    let radius = BallSize(size).radius() + 0.05;
    position.x = position.x.clamp(-box_size.x + radius, box_size.x - radius);
    position.z = position.z.clamp(-box_size.z + radius, box_size.z - radius);
    // Draw example ball and line
    gizmos.ray(
        position,
        Vec3::new(0.0, -box_size.y * 2. + 0.1, 0.0),
        Color::GREEN,
    );
    example_ball.1.translation = position;
    //Check if mouse pressed
    if cursor_type == CursorChangeType::DragEnd {
        // Add points
        points.as_mut().0 += size as i32;
        // Spawn ball
        let mut new_ball = Ball::new(size);
        new_ball.spatial.transform.translation = position;
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
