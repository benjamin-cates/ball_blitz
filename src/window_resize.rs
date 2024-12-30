use bevy::prelude::*;

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, browser_resize)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {}
}

#[cfg(target_arch = "wasm32")]
fn browser_resize(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {}
