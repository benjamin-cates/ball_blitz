use crate::setup::{BoxScaleEvent, BoxSize, BoxTag};
use bevy::prelude::*;

pub(crate) fn box_scale(
    mut event: EventReader<BoxScaleEvent>,
    mut box_size: ResMut<BoxSize>,
    mut commands: Commands,
    mut query: Query<Entity, With<BoxTag>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    if event.is_empty() {
        return;
    }
    let new_size = event.read().next().unwrap();
    let ent = query.single_mut();
    let name = Name::new("Box");
    commands.entity(ent).insert(name.clone()).insert({
        let mut player = AnimationPlayer::default();
        let mut animation = AnimationClip::default();
        let default_box_size = BoxSize::default();
        let start = Vec3::new(
            box_size.x / default_box_size.x,
            box_size.y / default_box_size.y,
            box_size.z / default_box_size.z,
        );
        let end = Vec3::new(
            new_size.x / default_box_size.x,
            new_size.y / default_box_size.y,
            new_size.z / default_box_size.z,
        );
        animation.add_curve_to_path(
            EntityPath {
                parts: vec![name.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0., 1., 100000000.],
                keyframes: Keyframes::Scale(vec![
                    start, end,
                    end, //Issue with Bevy 0.12.0 animations
                        // ending. Will fix later
                ]),
            },
        );
        player.play(animations.add(animation)).set_speed(1.0);
        player
    });
    box_size.as_mut().x = new_size.x;
    box_size.as_mut().y = new_size.y;
    box_size.as_mut().z = new_size.z;
    event.clear();
}
