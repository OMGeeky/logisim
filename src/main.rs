use bevy::color::palettes::basic::{RED, WHITE};
use bevy::prelude::*;

mod fps_counter;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            fps_counter::FpsCounterPlugin,
            MeshPickingPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, change_window_mode)
        // .add_systems(Update, switch_focused_shape)
        .add_systems(Update, show_focused_shape)
        .add_systems(FixedUpdate, focused_follow_cursor)
        .add_systems(Update, draw_cursor)
        .run();
}

fn change_window_mode(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    // window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
    window.present_mode = bevy::window::PresentMode::AutoNoVsync;
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(Rectangle::new(100.0, 50.0)),
    ];
    let num_shapes = shapes.len();
    for (i, shape) in shapes.into_iter().enumerate() {
        commands
            .spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(Color::from(RED))),
                Focused(true),
                Transform::from_xyz((i as f32 - num_shapes as f32 / 2.0) * 150.0, 0.0, 0.0),
            ))
            .observe(on_click_shape);
    }
}
fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let Some(point) = get_cursor_worl_pos(camera_query, windows) else {
        return;
    };

    gizmos.circle_2d(point, 10.0, WHITE);
}

fn get_cursor_worl_pos(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera_query;
    let Ok(window) = windows.get_single() else {
        return None;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return None;
    };

    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return None;
    };
    Some(point)
}

#[derive(Component)]
struct Focused(bool);
fn show_focused_shape(focused: Query<(&Focused, &GlobalTransform)>, mut gizmos: Gizmos) {
    for (focused, transform) in focused.iter() {
        let point: Vec2 = transform.translation().xy();
        if focused.0 {
            gizmos.circle_2d(point, 10.0, WHITE);
        }
    }
}
fn focused_follow_cursor(
    time: Res<Time<Fixed>>,
    mut focused: Query<(&Focused, &mut Transform)>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let Some(point) = get_cursor_worl_pos(camera_query, windows) else {
        return;
    };
    const MOVEMENT_SPEED: f32 = 100.0;
    for (focused, mut transform) in focused.iter_mut() {
        if focused.0 {
            let move_vec = point - transform.translation.xy();
            if move_vec.length_squared() < 0.0001 {
                continue;
            }
            let capped_move_vec = move_vec.normalize() * time.delta_secs() * MOVEMENT_SPEED;
            if capped_move_vec.length_squared() > move_vec.length_squared() {
                transform.translation += move_vec.extend(0.0);
                continue;
            }
            transform.translation += capped_move_vec.extend(0.0);
        }
    }
}
fn on_click_shape(click: Trigger<Pointer<Click>>, mut focused: Query<&mut Focused>) {
    let Ok(mut focused) = focused.get_mut(click.entity()) else {
        return;
    };

    focused.0 = !focused.0;
}
