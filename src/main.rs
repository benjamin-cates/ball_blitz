mod balls;
mod camera;
mod scene_scale;
mod setup;

use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;
use bevy_wasm_window_resize::WindowResizePlugin;
use bevy_xpbd_3d::prelude::*;
use setup::BoxScaleEvent;

fn main() {
    App::new()
        .insert_resource(bevy::asset::AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(WindowResizePlugin)
        .add_systems(Startup, balls::load_ball_templates)
        .add_systems(PostStartup, setup::setup)
        .add_systems(Update, balls::merge_check)
        .add_systems(Update, balls::insertion_check)
        .add_systems(Update, camera::orbit_camera)
        .add_systems(
            Update,
            scene_scale::box_scale.run_if(on_event::<BoxScaleEvent>()),
        )
        .add_event::<BoxScaleEvent>()
        .insert_resource(GizmoConfig {
            line_width: 100.0,
            line_perspective: true,
            depth_bias: 0.,
            ..default()
        })
        .insert_resource(Gravity(Vec3::new(0.0, -45.0, 0.0)))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .run();
}
