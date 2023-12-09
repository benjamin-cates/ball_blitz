use crate::balls::*;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub fn merge_check(
    mut query: Query<
        (Entity, &mut BallSize, &mut LinearVelocity, &mut Transform),
        Without<ExampleBall>,
    >,
    mut commands: Commands,
    ball_templates: Res<BallTemplates>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(ent1, size1, vel1, trans1), (ent2, size2, vel2, trans2)]) =
        combinations.fetch_next()
    {
        if size1.0 != size2.0 {
            continue;
        }
        if commands.get_entity(ent1).is_none() || commands.get_entity(ent2).is_none() {
            println!("Skipping non-existant");
            continue;
        }

        if trans1.translation.distance_squared(trans2.translation)
            < size1.radius() * size1.radius() * 4.0 + 0.1
        {
            let mut new_trans = if vel1.0.length_squared() < vel2.0.length_squared() {
                trans1.clone()
            } else {
                trans2.clone()
            };
            commands.entity(ent1).despawn_recursive();
            commands.entity(ent2).despawn_recursive();
            let name = Name::new("combined".to_owned() + format!("{}", size1.0 + 1).as_str());
            let start = new_trans.scale.x * BallSize(size1.0).start_radius()
                / BallSize(size1.0 + 1).start_radius()
                * 0.9;
            new_trans.scale = Vec3::from_array([start; 3]);
            let mut new_ball = Ball::new(size1.0 + 1);
            new_ball.spatial.transform = new_trans;
            new_ball.collider.set_scale(new_trans.scale, 0);
            new_ball
                .spawn(&ball_templates, &mut commands)
                .insert(name.clone())
                .insert({
                    let mut player = AnimationPlayer::default();
                    let mut animation = AnimationClip::default();
                    let end = BallSize(size1.0 + 1).scale();
                    animation.add_curve_to_path(
                        EntityPath {
                            parts: vec![name.clone()],
                        },
                        VariableCurve {
                            keyframe_timestamps: vec![0.0, 1.0, 100000000.0],
                            keyframes: Keyframes::Scale(vec![
                                Vec3::new(start, start, start),
                                Vec3::new(end, end, end),
                                Vec3::new(end, end, end), //Issue with Bevy 0.12.0 animations
                                                          // ending. Will fix later
                            ]),
                        },
                    );
                    player.play(animations.add(animation)).set_speed(2.0);
                    player
                });
        }
    }
}
