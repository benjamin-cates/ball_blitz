mod balls;
mod camera;
mod scene_setup;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(bevy::asset::AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, balls::load_ball_templates)
        .add_systems(PostStartup, scene_setup::setup)
        .add_systems(Update, balls::merge_check)
        .add_systems(Update, balls::insertion_check)
        .add_systems(Update, camera::orbit_camera)
        .insert_resource(Gravity(Vec3::new(0.0, -45.0, 0.0)))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .run();
}
