use crate::shape_follow::ShapeFollowPlugin;
use crate::utils::get_cursor_world_pos;
use bevy::color::palettes::basic::{RED, WHITE};
use bevy::prelude::*;

mod fps_counter;
mod shape_follow;
mod utils;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            fps_counter::FpsCounterPlugin,
            ShapeFollowPlugin,
        ))
        .add_systems(Startup, (setup_camera, change_window_mode))
        .add_systems(Update, draw_cursor)
        .run();
}

const CURSOR_SIZE: f32 = 10.0;
fn change_window_mode(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    // window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
    window.present_mode = bevy::window::PresentMode::AutoNoVsync;
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Some(point) = get_cursor_world_pos(camera_query, windows) else {
        return;
    };

    gizmos.circle_2d(point, CURSOR_SIZE, WHITE);
}
