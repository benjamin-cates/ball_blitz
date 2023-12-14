use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component)]
pub struct BallSize(pub u8);

#[derive(Component)]
pub struct ExampleBall(pub ());

#[derive(Resource)]
pub struct BallTemplates {
    meshes: Vec<Vec<PbrBundle>>,
}

pub fn load_ball_templates(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
) {
    let colors: Vec<Color> = vec![
        Color::rgb(1.0, 0.5, 1.0),       // Zero ball (not used)
        Color::rgb(1.0, 0.45, 0.3),      // Ping pong
        Color::rgb(0.9, 0.9, 0.9),       // Golf
        Color::rgb(0.0, 0.0, 0.4),       // Pool
        Color::rgb(0.369, 0.624, 0.0),   // Tennis
        Color::rgb(1.0, 1.0, 1.0),       // Baseball
        Color::rgb(0.8, 0.8, 0.8),       // Soccer
        Color::rgb(0.612, 0.145, 0.036), // Basketball
        Color::rgb(1.0, 1.0, 1.0),       // Beach ball
        Color::rgb(1.0, 0.0, 1.0),
    ];
    let models: Vec<Option<(&'static str, f32, Vec<(&'static str, &'static str)>)>> = vec![
        None,
        None,
        Some(("Golf.glb", 0.31, vec![("Mesh0/Primitive0", "Material0")])),
        Some((
            "Pool.glb",
            0.95,
            vec![
                ("Mesh0/Primitive0", "Material0"),
                ("Mesh0/Primitive1", "Material1"),
                ("Mesh1/Primitive0", "Material2"),
                ("Mesh2/Primitive0", "Material0"),
            ],
        )),
        Some((
            "Tennis.glb",
            0.0106,
            vec![
                ("Mesh0/Primitive0", "Material0"),
                ("Mesh1/Primitive0", "Material1"),
            ],
        )),
        Some((
            "Baseball.glb",
            0.4,
            vec![
                ("Mesh0/Primitive0", "Material0"),
                ("Mesh0/Primitive1", "Material1"),
                ("Mesh0/Primitive2", "Material2"),
            ],
        )),
        Some(("Bowling.glb", 6.85, vec![("Mesh0/Primitive0", "Material0")])),
        Some((
            "Soccer.glb",
            2.16,
            vec![
                ("Mesh0/Primitive0", "Material0"),
                ("Mesh0/Primitive1", "Material1"),
            ],
        )),
        Some((
            "Basketball.glb",
            4.7,
            vec![
                ("Mesh0/Primitive0", "Material0"),
                ("Mesh1/Primitive0", "Material1"),
            ],
        )),
        Some((
            "Beach_ball.glb",
            2.65,
            vec![
                ("Mesh0/Primitive0", "Material0"),
                ("Mesh1/Primitive0", "Material1"),
                ("Mesh2/Primitive0", "Material2"),
                ("Mesh3/Primitive0", "Material3"),
            ],
        )),
        None,
        None,
        None,
        None,
    ];
    let pbr_bundles: Vec<Vec<PbrBundle>> = (0..10u8)
        .map(|idx| match &models[idx as usize] {
            None => vec![PbrBundle {
                material: materials.add(StandardMaterial {
                    base_color: colors[idx as usize],
                    perceptual_roughness: 0.67,
                    specular_transmission: 0.5,
                    ..default()
                }),
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: BallSize(idx).start_radius(),
                    sectors: (BallSize(idx).radius() * 10. + 20.) as usize,
                    stacks: (BallSize(idx).radius() * 10. + 20.) as usize,
                })),
                ..default()
            }],
            Some((file, scale, meshes)) => meshes
                .iter()
                .map(|(mesh, mat)| PbrBundle {
                    material: assets.load((*file).to_owned() + "#" + *mat),
                    mesh: assets.load((*file).to_owned() + "#" + *mesh),
                    transform: Transform {
                        scale: Vec3::from_array([*scale; 3]),
                        ..default()
                    },
                    ..default()
                })
                .collect(),
        })
        .collect();
    commands.insert_resource(BallTemplates {
        meshes: pbr_bundles,
    });
}

impl BallSize {
    pub fn radius(&self) -> f32 {
        self.0 as f32 * 0.3 + 0.4
    }
    pub fn start_radius(&self) -> f32 {
        (self.0 as f32 * 0.3 - 0.3 + 0.4) * 0.95
    }
    pub fn scale(&self) -> f32 {
        self.radius() / self.start_radius()
    }
}

#[derive(Bundle)]
pub struct Ball {
    pub size: BallSize,
    rigid_body: RigidBody,
    pub collider: Collider,
    pub vel: LinearVelocity,
    pub spatial: SpatialBundle,
    mass: Mass,
    restitution: Restitution,
    friction: Friction,
}

const DEFAULT_RESTITUTION: f32 = 0.0;
const DEFAULT_FRICTION: f32 = 0.8;

impl Default for Ball {
    fn default() -> Self {
        Self {
            size: BallSize(1),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(BallSize(1).start_radius()),
            restitution: Restitution::new(DEFAULT_RESTITUTION),
            friction: Friction::new(DEFAULT_FRICTION),
            mass: Mass(1.0),
            vel: LinearVelocity(Vec3::new(0., 0., 0.)),
            spatial: SpatialBundle { ..default() },
        }
    }
}

impl Ball {
    pub fn new(size: u8) -> Self {
        let mut out: Ball = Ball::default();
        out.size = BallSize(size);
        out.collider = Collider::ball(BallSize(size).start_radius());
        out.spatial.transform.scale = Vec3::from_array([BallSize(size).scale(); 3]);
        out.mass = Mass((size as f32) * (size as f32) * 10.0);
        out
    }
    pub fn get_meshes(size: u8, templates: &BallTemplates, commands: &mut Commands) -> Vec<Entity> {
        templates.meshes[if size > 9 { 9 } else { size } as usize]
            .iter()
            .map(|pbr_bundle| commands.spawn(pbr_bundle.clone()).id())
            .collect()
    }
    pub fn spawn<'w, 's, 'a>(
        self,
        templates: &BallTemplates,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a> {
        let meshes = Ball::get_meshes(self.size.0, templates, commands);
        let mut entity_commands = commands.spawn(self);
        entity_commands.push_children(meshes.as_slice());
        entity_commands
    }
}
