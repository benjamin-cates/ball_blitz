use crate::setup::BoxSize;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct TouchState {
    movement: Vec2,
    event_type: CursorChangeType,
    position: Vec2,
    is_orbit: bool,
}

#[derive(Clone, Debug, Resource)]
pub struct CursorTracking {
    touches: BTreeMap<u64, TouchState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorChangeType {
    Drag,
    DragStart,
    DragEnd,
    Move,
    NoChange,
}

#[derive(Clone, Copy, Debug, PartialEq, Event)]
pub struct OrbitUpdate {
    pub delta: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq, Event)]
pub struct BallSpawnUpdate {
    pub cursor_type: CursorChangeType,
    pub position: Option<Vec3>,
}

impl CursorTracking {
    pub fn new() -> Self {
        Self {
            touches: BTreeMap::new(),
        }
    }
    fn get_tracked(
        &mut self,
        mut touch_event: EventReader<TouchInput>,
        buttons: &Res<Input<MouseButton>>,
        window: &Query<&Window, With<PrimaryWindow>>,
        is_orbit_fn: impl Fn(Vec2) -> Option<Vec3>,
    ) {
        use bevy::input::touch::TouchPhase;
        // Cursor movement
        let cursor: Option<Vec2> = window.get_single().unwrap().cursor_position();
        let mut new_touches = BTreeMap::new();
        if buttons.just_pressed(MouseButton::Left) {
            new_touches.insert(
                0,
                TouchState {
                    event_type: CursorChangeType::DragStart,
                    position: cursor.unwrap(),
                    is_orbit: is_orbit_fn(cursor.unwrap()).is_none(),
                    movement: Vec2::ZERO,
                },
            );
        } else {
            let event_type = if cursor.is_none() || buttons.just_released(MouseButton::Left) {
                CursorChangeType::DragEnd
            } else if buttons.pressed(MouseButton::Left) {
                CursorChangeType::Drag
            } else {
                CursorChangeType::Move
            };
            if let Some(mouse) = self.touches.get(&0) {
                new_touches.insert(
                    0,
                    TouchState {
                        event_type,
                        position: cursor.unwrap_or(mouse.position),
                        movement: cursor.unwrap_or(mouse.position) - mouse.position,
                        ..mouse.clone()
                    },
                );
            }
        }
        // Parse touch events
        for touch in touch_event.read() {
            match touch.phase {
                TouchPhase::Started => {
                    new_touches.insert(
                        touch.id,
                        TouchState {
                            movement: Vec2::ZERO,
                            event_type: CursorChangeType::DragStart,
                            position: touch.position,
                            is_orbit: is_orbit_fn(touch.position).is_none(),
                        },
                    );
                }
                TouchPhase::Moved => {
                    if let Some(old_touch) = self.touches.get(&touch.id) {
                        new_touches.insert(
                            touch.id,
                            TouchState {
                                movement: touch.position - old_touch.position,
                                event_type: CursorChangeType::Drag,
                                position: touch.position,
                                ..old_touch.clone()
                            },
                        );
                    } else {
                        println!("Couldn't find old touch");
                    }
                }
                TouchPhase::Canceled | TouchPhase::Ended => {
                    if let Some(old_touch) = self.touches.get(&touch.id) {
                        new_touches.insert(
                            touch.id,
                            TouchState {
                                movement: touch.position - old_touch.position,
                                event_type: CursorChangeType::DragEnd,
                                position: touch.position,
                                ..old_touch.clone()
                            },
                        );
                    }
                }
            }
        }
        for (id, touch) in self.touches.iter() {
            if touch.event_type == CursorChangeType::DragEnd {
                continue;
            }
            if new_touches.get(&id).is_none() {
                new_touches.insert(
                    *id,
                    TouchState {
                        movement: Vec2::ZERO,
                        event_type: CursorChangeType::NoChange,
                        ..touch.clone()
                    },
                );
            }
        }
        self.touches = new_touches;
    }
}

fn raycast_box_top(
    cursor: Vec2,
    camera_trans: (&Camera, &GlobalTransform),
    box_size: &Res<BoxSize>,
) -> Option<Vec3> {
    let (camera, transform) = camera_trans;
    let ray = camera.viewport_to_world(transform, cursor)?;
    let dist = ray.intersect_plane(Vec3::new(0.0, box_size.y, 0.0), Vec3::new(0.0, 1.0, 0.0))?;
    let point = ray.get_point(dist);
    if point.x > box_size.x
        || point.x < -box_size.x
        || point.z > box_size.z
        || point.z < -box_size.z
    {
        None
    } else {
        Some(point)
    }
}

pub fn cursor_read(
    touch_event: EventReader<TouchInput>,
    mut orbit_updates: EventWriter<OrbitUpdate>,
    mut ball_spawn_updates: EventWriter<BallSpawnUpdate>,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    cam_query: Query<(&Camera, &GlobalTransform)>,
    box_size: Res<BoxSize>,
    mut cursor_state: ResMut<CursorTracking>,
) {
    let raycast_fn = |cursor| raycast_box_top(cursor, cam_query.single(), &box_size);
    cursor_state.get_tracked(touch_event, &buttons, &window, raycast_fn);
    // Orbit events
    for (_id, change) in cursor_state.touches.iter() {
        if change.event_type == CursorChangeType::Drag && change.is_orbit {
            orbit_updates.send(OrbitUpdate {
                delta: change.movement,
            });
        }
    }
    // Ball spawning events
    let ball_spawner = cursor_state.touches.iter().find_map(|(_id, touch)| {
        if touch.is_orbit && touch.event_type != CursorChangeType::Move {
            None
        } else {
            raycast_fn(touch.position).map(|x| (Some(x), touch.event_type))
        }
    });
    let (position, cursor_type) = ball_spawner.unwrap_or((None, CursorChangeType::Move));
    ball_spawn_updates.send(BallSpawnUpdate {
        cursor_type,
        position,
    });
}
