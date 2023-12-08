use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component)]
pub struct BallSize(pub u8);

#[derive(Component)]
pub struct ExampleBall(pub ());

#[derive(Resource)]
pub struct BallTemplates {
    materials: Vec<Handle<StandardMaterial>>,
    meshes: Vec<Handle<Mesh>>,
}

pub fn load_ball_templates(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
    let materials: Vec<Handle<StandardMaterial>> = colors
        .into_iter()
        .map(|col| {
            materials.add(StandardMaterial {
                emissive: col,
                ..Default::default()
            })
        })
        .collect();
    let meshes: Vec<Handle<Mesh>> = (0..10)
        .map(|size| {
            meshes.add(Mesh::from(shape::UVSphere {
                radius: BallSize(size).start_radius(),
                sectors: (BallSize(size).radius() * 10. + 20.) as usize,
                stacks: (BallSize(size).radius() * 10. + 20.) as usize,
            }))
        })
        .collect();
    commands.insert_resource(BallTemplates { materials, meshes });
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
    pub pbr_bundle: PbrBundle,
    pub vel: LinearVelocity,
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
            pbr_bundle: PbrBundle { ..default() },
            restitution: Restitution::new(DEFAULT_RESTITUTION),
            friction: Friction::new(DEFAULT_FRICTION),
            mass: Mass(1.0),
            vel: LinearVelocity(Vec3::new(0., 0., 0.)),
        }
    }
}

impl Ball {
    pub fn new(size: u8, templates: &Res<BallTemplates>) -> Self {
        let mut out: Ball = Ball::default();
        out.size = BallSize(size);
        out.collider = Collider::ball(BallSize(size).start_radius());
        out.pbr_bundle.transform.scale = Vec3::from_array([BallSize(size).scale(); 3]);
        out.mass = Mass((size as f32) * (size as f32) * 10.0);
        out.pbr_bundle.mesh = templates.meshes[size as usize].clone();
        out.pbr_bundle.material = templates.materials[size as usize].clone();
        out
    }
}
