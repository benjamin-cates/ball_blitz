use crate::{balls, camera};
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_xpbd_3d::{math::PI, prelude::*};

#[derive(Resource)]
pub struct BoxSize {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for BoxSize {
    fn default() -> Self {
        Self {
            x: 4.,
            y: 6.,
            z: 4.,
        }
    }
}

#[derive(Event)]
pub struct BoxScaleEvent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Component)]
pub struct BoxTag(pub ());

#[derive(Component)]
pub struct WallTag(pub char, pub f32);

#[derive(Bundle)]
struct WallBundle {
    rigid_body: RigidBody,
    collider: Collider,
    pbr_bundle: PbrBundle,
    shadows: bevy::pbr::NotShadowCaster,
    tag: WallTag,
}

impl WallBundle {
    fn new(
        box_size: &BoxSize,
        direction: char,
        side: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        let pos = match direction {
            'x' => Vec3::new(side * box_size.x, 0., 0.),
            'y' => Vec3::new(0., side * box_size.y, 0.),
            'z' => Vec3::new(0., 0., side * box_size.z),
            ch => panic!("Unexpected direction {}", ch),
        };
        let width = 2.
            * match direction {
                'x' => box_size.z,
                'y' => box_size.x,
                'z' => box_size.x,
                ch => panic!("Unexpected direction {}", ch),
            };
        let height = 2.
            * match direction {
                'x' => box_size.y,
                'y' => box_size.z,
                'z' => box_size.y,
                ch => panic!("Unexpected direction {}", ch),
            };
        let rotation = if direction == 'y' {
            Quat::from_rotation_x(-side * PI / 2.)
        } else {
            Quat::from_rotation_y(if direction == 'x' { side } else { side - 1. } * PI / 2.)
        };
        return Self {
            shadows: bevy::pbr::NotShadowCaster {},
            rigid_body: RigidBody::Static,
            collider: Collider::cuboid(width, height, 0.01),
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))),
                transform: Transform {
                    translation: pos,
                    rotation,
                    scale: Vec3::ONE,
                },
                material,
                ..default()
            },
            tag: WallTag(direction, side),
        };
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ball_templates: Res<balls::BallTemplates>,
) {
    spawn_box(&mut commands, &mut materials, &mut meshes);
    spawn_lights(&mut commands);
    commands.spawn(camera::new_camera());
    let mut example_ball = balls::Ball::new(1);
    example_ball.spatial.transform.translation = Vec3::new(0.0, 400000.0, 0.0);
    example_ball
        .spawn(&ball_templates, &mut commands)
        .insert(balls::ExampleBall(()))
        .insert(GravityScale(0.0))
        .insert(RigidBody::Kinematic);
}

/// Creates a ball-holding box with walls and a base
/// The inner width of the box is 8 units, each wall is 0.5 units thick
/// The height of the box is 12 units
fn spawn_box(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let mut bundles: Vec<Entity> = vec![];
    let box_size = BoxSize::default();
    let wall_mat = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        base_color: Color::rgba(0.3, 0.3, 0.3, 0.12),
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    // Boxes that are in the positive x and negative x direction
    for side in [-1.0f32, 1.0] {
        for dir in ['x', 'z'] {
            bundles.push(
                commands
                    .spawn(WallBundle::new(
                        &box_size,
                        dir,
                        side,
                        meshes,
                        wall_mat.clone(),
                    ))
                    .id(),
            );
        }
    }
    // Base of the box
    let base_mat = materials.add(StandardMaterial {
        base_color: Color::rgba(0.1, 0.4, 0.2, 1.0),
        double_sided: true,
        reflectance: 0.1,
        cull_mode: None,
        ..default()
    });
    bundles.push(
        commands
            .spawn(WallBundle::new(
                &box_size,
                'y',
                -1.,
                meshes,
                base_mat.clone(),
            ))
            .id(),
    );

    // Black lines representing the edges of the box
    let line_mat = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        unlit: true,
        ..default()
    });
    let line_mesh = meshes.add(
        Mesh::new(PrimitiveTopology::LineList).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                // 4 vertical lines
                [-box_size.x, -box_size.y, -box_size.z],
                [-box_size.x, box_size.y, -box_size.z],
                [-box_size.x, -box_size.y, box_size.z],
                [-box_size.x, box_size.y, box_size.z],
                [box_size.x, -box_size.y, -box_size.z],
                [box_size.x, box_size.y, -box_size.z],
                [box_size.x, -box_size.y, box_size.z],
                [box_size.x, box_size.y, box_size.z],
                // 4 horizontal lines on top making the square
                [-box_size.x, box_size.y, -box_size.z],
                [-box_size.x, box_size.y, box_size.z],
                [-box_size.x, box_size.y, -box_size.z],
                [box_size.x, box_size.y, -box_size.z],
                [box_size.x, box_size.y, -box_size.z],
                [box_size.x, box_size.y, box_size.z],
                [-box_size.x, box_size.y, box_size.z],
                [box_size.x, box_size.y, box_size.z],
            ],
        ),
    );
    commands.insert_resource(box_size);
    bundles.push(
        commands
            .spawn(PbrBundle {
                mesh: line_mesh,
                material: line_mat,
                ..default()
            })
            .id(),
    );
    let mut box_ent = commands.spawn((BoxTag(()), SpatialBundle::default()));
    box_ent.push_children(bundles.as_ref());
}

// Spawn two direcitonal lights and an ambient light
fn spawn_lights(commands: &mut Commands) {
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(30., 12., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        spot_light: SpotLight {
            color: Color::rgb(1., 0.8, 0.8),
            shadows_enabled: true,
            intensity: 80000.,
            inner_angle: PI / 10.,
            outer_angle: PI / 10.,
            range: 50.,
            ..default()
        },
        ..default()
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(-10., 4., 30.).looking_at(Vec3::ZERO, Vec3::Y),
        spot_light: SpotLight {
            color: Color::rgb(1., 1., 0.8),
            shadows_enabled: true,
            intensity: 120000.,
            inner_angle: PI / 10.,
            outer_angle: PI / 10.,
            range: 50.,
            ..default()
        },
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
}
