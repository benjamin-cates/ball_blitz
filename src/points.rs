use crate::balls::BallSize;
use bevy::prelude::*;

/// Resource that keeps track of points in game
#[derive(Resource)]
pub struct GamePoints(pub i32);

/// Label struct for points ui
#[derive(Component)]
pub struct PointsDisplay;

/// Spawn points display text
pub fn spawn_points_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let style = TextStyle {
        font: asset_server.load("fonts/mononoki-Regular.ttf"),
        font_size: 25.0,
        color: Color::WHITE,
    };
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Points: ", style.clone()),
            TextSection::from_style(style.clone()),
        ]),
        PointsDisplay,
    ));
}

/// Update game points display
pub fn update_points(mut query: Query<&mut Text, With<PointsDisplay>>, points: Res<GamePoints>) {
    let mut text = query.get_single_mut().unwrap();
    text.sections[1].value = format!("{}", points.0);
}

/// Despawns balls out of bounds and reduces point count
pub fn ball_out_of_bounds(
    mut points: ResMut<GamePoints>,
    query: Query<(Entity, &Transform, &BallSize)>,
    mut commands: Commands,
) {
    for (ent, trans, size) in query.iter() {
        if trans.translation.y < -50. {
            commands.entity(ent).despawn_recursive();
            points.as_mut().0 -= (size.0 as i32) * 100;
        }
    }
}
