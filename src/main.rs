use crate::camera::CameraPlugin;
use crate::logic_sim::LogicSimPlugin;
use bevy::prelude::*;

mod fps_counter;
mod logic_sim;
mod shape_follow;
mod utils;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            fps_counter::SimpleFpsCounterPlugin,
            // ShapeFollowPlugin,
            LogicSimPlugin,
        ))
        .run();
}

mod camera {
    use crate::utils::get_cursor_world_pos;
    use bevy::color::palettes::basic::WHITE;
    use bevy::input::common_conditions::input_pressed;
    use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
    use bevy::prelude::*;

    const CURSOR_SIZE: f32 = 10.0;
    fn change_window_mode(mut windows: Query<&mut Window>) {
        let mut window = windows.single_mut();
        // window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
        window.present_mode = bevy::window::PresentMode::AutoNoVsync;
    }

    #[derive(Resource)]
    pub struct Canvas {
        pub zoom: f32,
    }
    pub struct CameraPlugin;
    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(CameraSettings {
                zoom_speed: 0.05,
                orthographic_zoom_range: 0.1..20.0,
            })
            .add_systems(Startup, (change_window_mode, setup_camera))
            .add_systems(PostUpdate, draw_cursor)
            .add_systems(Update, zoom)
            .add_systems(Update, handle_pan.run_if(input_pressed(MouseButton::Right)));
        }
    }
    fn setup_camera(mut commands: Commands) {
        commands.insert_resource(Canvas { zoom: 1.0 });
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

    fn handle_pan(
        mut camera: Single<(&mut Transform), With<Camera2d>>,
        move_event: Res<AccumulatedMouseMotion>,
    ) {
        camera.translation.x -= move_event.delta.x;
        camera.translation.y += move_event.delta.y;
    }
    fn zoom(
        mut canvas: ResMut<Canvas>,
        camera_settings: Res<CameraSettings>,
        mouse_wheel_input: Res<AccumulatedMouseScroll>,
    ) {
        // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
        let delta_zoom = mouse_wheel_input.delta.y * camera_settings.zoom_speed;
        // When changing scales, logarithmic changes are more intuitive.
        // To get this effect, we add 1 to the delta, so that a delta of 0
        // results in no multiplicative effect, positive values result in a multiplicative increase,
        // and negative values result in multiplicative decreases.
        let multiplicative_zoom = 1. + delta_zoom;

        canvas.zoom = (canvas.zoom * multiplicative_zoom).clamp(
            camera_settings.orthographic_zoom_range.start,
            camera_settings.orthographic_zoom_range.end,
        );
    }
    #[derive(Resource)]
    pub struct CameraSettings {
        zoom_speed: f32,
        orthographic_zoom_range: std::ops::Range<f32>,
    }
}
