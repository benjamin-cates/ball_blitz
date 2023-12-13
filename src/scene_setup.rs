use crate::{balls, camera};
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};
use bevy_xpbd_3d::{math::PI, prelude::*};

const BOX_HEIGHT: f32 = 12.0;
const BOX_WIDTH: f32 = 8.0;
const BOX_THICKNESS: f32 = 0.5;

#[derive(Bundle)]
struct WallBundle {
    rigid_body: RigidBody,
    collider: Collider,
    pbr_bundle: PbrBundle,
    shadows: bevy::pbr::NotShadowCaster,
}

impl WallBundle {
    fn new(
        pos: Vec3,
        xlen: f32,
        zlen: f32,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        rotation: Quat,
    ) -> Self {
        return Self {
            shadows: bevy::pbr::NotShadowCaster {},
            rigid_body: RigidBody::Static,
            collider: Collider::cuboid(xlen, zlen, 0.01),
            pbr_bundle: PbrBundle {
                mesh,
                transform: Transform {
                    translation: pos,
                    rotation,
                    scale: Vec3::ONE,
                },
                material,
                ..default()
            },
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
    example_ball.spatial.transform.translation = Vec3::new(0.0, 12.0, 0.0);
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
    let wall_mat = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        base_color: Color::rgba(0.3, 0.3, 0.3, 0.12),
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    // Boxes that are in the positive x and negative x direction
    for side in [-1.0f32, 1.0] {
        commands.spawn(WallBundle::new(
            Vec3::new(side * BOX_WIDTH / 2.0, BOX_HEIGHT / 2.0, 0.0),
            BOX_WIDTH,
            BOX_HEIGHT,
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                BOX_WIDTH, BOX_HEIGHT,
            )))),
            wall_mat.clone(),
            Quat::from_rotation_y(side * PI / 2.0),
        ));
    }
    // Boxes that are in the positive z and negative z direction
    for side in [-1.0f32, 1.0] {
        commands.spawn(WallBundle::new(
            Vec3::new(0.0, BOX_HEIGHT / 2.0, side * BOX_WIDTH / 2.0),
            BOX_WIDTH,
            BOX_HEIGHT,
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                BOX_WIDTH, BOX_HEIGHT,
            )))),
            wall_mat.clone(),
            Quat::from_rotation_y((side - 1.0) * PI / 2.0),
        ));
    }
    // Base of the box
    let base_mat = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Blend,
        base_color: Color::rgba(0.1, 0.4, 0.2, 1.0),
        metallic: 0.6,
        perceptual_roughness: 0.3,
        double_sided: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn(WallBundle::new(
        Vec3::new(0.0, 0.0, 0.0),
        BOX_WIDTH + BOX_THICKNESS,
        BOX_WIDTH + BOX_THICKNESS,
        meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
            BOX_WIDTH, BOX_WIDTH,
        )))),
        base_mat.clone(),
        Quat::from_rotation_x(PI / 2.0),
    ));

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
                [-4., 0., -4.],
                [-4., 12., -4.],
                [-4., 0., 4.],
                [-4., 12., 4.],
                [4., 0., -4.],
                [4., 12., -4.],
                [4., 0., 4.],
                [4., 12., 4.],
                [-4., 12., -4.],
                [-4., 12., 4.],
                [-4., 12., -4.],
                [4., 12., -4.],
                [4., 12., -4.],
                [4., 12., 4.],
                [-4., 12., 4.],
                [4., 12., 4.],
            ],
        ),
    );
    commands.spawn(PbrBundle {
        mesh: line_mesh,
        material: line_mat,
        ..default()
    });
}

// Spawn two direcitonal lights and an ambient light
fn spawn_lights(commands: &mut Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 4000.0,
            shadows_enabled: true,
            color: Color::rgb(1.0, 1.0, 0.8),
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_y(1.2) * Quat::from_rotation_x(-0.5),
            ..default()
        },
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            color: Color::rgb(1.0, 1.0, 0.8),
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_y(2.6) * Quat::from_rotation_x(-0.8),
            ..default()
        },
        ..default()
    });
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
