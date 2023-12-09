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
        Color::rgb(1.0, 0.5, 1.0),
        Color::rgb(1.0, 1.0, 0.5),
        Color::rgb(0.5, 1.0, 1.0),
        Color::rgb(0.5, 0.5, 0.5),
        Color::rgb(1.0, 0.0, 0.0),
        Color::rgb(0.0, 1.0, 0.0),
        Color::rgb(0.0, 0.0, 1.0),
        Color::rgb(0.0, 1.0, 1.0),
        Color::rgb(1.0, 1.0, 0.0),
        Color::rgb(1.0, 0.0, 1.0),
    ];
    let models: Vec<Option<Vec<(&'static str, f32, &'static str)>>> = vec![
        None,
        None,
        Some(vec![(
            "Golf.glb#Mesh0/Primitive0",
            0.31,
            "Golf.glb#Material0",
        )]),
        Some(vec![
            ("Pool.glb#Mesh0/Primitive0", 0.95, "Pool.glb#Material0"),
            ("Pool.glb#Mesh0/Primitive1", 0.95, "Pool.glb#Material1"),
            ("Pool.glb#Mesh1/Primitive0", 0.95, "Pool.glb#Material2"),
            ("Pool.glb#Mesh2/Primitive0", 0.95, "Pool.glb#Material0"),
        ]),
        Some(vec![
            (
                "Tennis.glb#Mesh0/Primitive0",
                0.0106,
                "Tennis.glb#Material0",
            ),
            (
                "Tennis.glb#Mesh1/Primitive0",
                0.0106,
                "Tennis.glb#Material1",
            ),
        ]),
        Some(vec![
            (
                "Baseball.glb#Mesh0/Primitive0",
                0.4,
                "Baseball.glb#Material0",
            ),
            (
                "Baseball.glb#Mesh0/Primitive1",
                0.4,
                "Baseball.glb#Material1",
            ),
            (
                "Baseball.glb#Mesh0/Primitive2",
                0.4,
                "Baseball.glb#Material2",
            ),
        ]),
        Some(vec![(
            "Bowling.glb#Mesh0/Primitive0",
            6.71,
            "Bowling.glb#Material0",
        )]),
        Some(vec![
            ("Soccer.glb#Mesh0/Primitive0", 2.04, "Soccer.glb#Material0"),
            ("Soccer.glb#Mesh0/Primitive1", 2.04, "Soccer.glb#Material1"),
        ]),
        Some(vec![
            (
                "Basketball.glb#Mesh0/Primitive0",
                4.64,
                "Basketball.glb#Material0",
            ),
            (
                "Basketball.glb#Mesh1/Primitive0",
                4.64,
                "Basketball.glb#Material1",
            ),
        ]),
        None,
        None,
        None,
        None,
        None,
    ];
    let pbr_bundles: Vec<Vec<PbrBundle>> = (0..10u8)
        .map(|idx| match &models[idx as usize] {
            None => vec![PbrBundle {
                material: materials.add(StandardMaterial {
                    emissive: colors[idx as usize],
                    ..default()
                }),
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: BallSize(idx).start_radius(),
                    sectors: (BallSize(idx).radius() * 10. + 20.) as usize,
                    stacks: (BallSize(idx).radius() * 10. + 20.) as usize,
                })),
                ..default()
            }],
            Some(mats) => mats
                .iter()
                .map(|(mesh, scale, mat)| PbrBundle {
                    material: assets.load(*mat),
                    mesh: assets.load(*mesh),
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
        templates.meshes[size as usize]
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