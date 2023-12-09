use crate::balls::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_3d::prelude::{Collider, Mass, RigidBody};
use rand::{self, Rng};

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
    let radius = BallSize(size).radius();
    point.x = point.x.clamp(-4.0 + radius, 4.0 - radius);
    point.z = point.z.clamp(-4.0 + radius, 4.0 - radius);
    // Draw example ball and line
    gizmos.ray(point, Vec3::new(0.0, -12.0, 0.0), Color::GREEN);
    example_ball.1.translation = point;
    //Check if mouse pressed
    if buttons.just_pressed(MouseButton::Left) {
        // Spawn ball
        let mut new_ball = Ball::new(size);
        new_ball.spatial.transform.translation = point;
        new_ball.spawn(&ball_templates, &mut commands);
        let new_size = if keys.pressed(KeyCode::ShiftLeft) {
            5
        } else {
            rand::thread_rng().gen_range(1..=4)
        };
        // Replace example ball
        commands.entity(example_ball.2).despawn_recursive();
        Ball::new(new_size)
            .spawn(&ball_templates, &mut commands)
            .remove::<Collider>()
            .remove::<RigidBody>()
            .insert(ExampleBall(()))
            .insert(Mass(1.0));
    }
}
