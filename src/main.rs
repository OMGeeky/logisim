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
        .add_systems(Startup, (setup_camera, change_window_mode))
        .add_systems(Startup, setup_shapes)
        .add_systems(Update, show_focused_shape)
        .add_systems(FixedUpdate, focused_follow_cursor)
        .add_systems(Update, draw_cursor)
        .run();
}
const CURSOR_SIZE: f32 = 10.0;
const FOCUS_MARK_SIZE: f32 = 10.0;
const SHAPE_Z_POS: f32 = 0.0;
const SHAPE_FOLLOW_SPEED: f32 = 100.0;

fn change_window_mode(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    // window.mode = WindowMode::Fullscreen(MonitorSelection::Current);
    window.present_mode = bevy::window::PresentMode::AutoNoVsync;
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
                Transform::from_xyz(
                    (i as f32 - num_shapes as f32 / 2.0) * 150.0,
                    0.0,
                    SHAPE_Z_POS,
                ),
            ))
            .observe(on_click_shape);
    }
}

fn on_click_shape(click: Trigger<Pointer<Click>>, mut focused: Query<&mut Focused>) {
    let Ok(mut focused) = focused.get_mut(click.entity()) else {
        return;
    };

    focused.0 = !focused.0;
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

fn get_cursor_world_pos(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) -> Option<Vec2> {
    let (camera, camera_transform) = *camera_query;
    let cursor_position = windows.get_single().ok()?.cursor_position()?;

    let point = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .ok()?;
    Some(point)
}

#[derive(Component)]
struct Focused(bool);

fn show_focused_shape(focused: Query<(&Focused, &GlobalTransform)>, mut gizmos: Gizmos) {
    for (focused, transform) in focused.iter() {
        let point: Vec2 = transform.translation().xy();
        if focused.0 {
            gizmos.circle_2d(point, FOCUS_MARK_SIZE, WHITE);
        }
    }
}
fn focused_follow_cursor(
    time: Res<Time<Fixed>>,
    mut focused: Query<(&Focused, &mut Transform)>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let Some(point) = get_cursor_world_pos(camera_query, windows) else {
        return;
    };
    for (focused, mut transform) in focused.iter_mut() {
        if focused.0 {
            move_towards(
                point,
                &mut transform,
                time.delta_secs() * SHAPE_FOLLOW_SPEED,
            );
        }
    }
}

fn move_towards(point: Vec2, transform: &mut Mut<Transform>, speed_per_frame: f32) {
    let move_vec = point - transform.translation.xy();
    if move_vec.length_squared() < 0.0001 {
        return;
    }
    let capped_move_vec = move_vec.normalize() * speed_per_frame;
    if capped_move_vec.length_squared() > move_vec.length_squared() {
        // prevent overshooting if we are already close to the target
        transform.translation += move_vec.extend(SHAPE_Z_POS);
    }
    transform.translation += capped_move_vec.extend(SHAPE_Z_POS);
}
