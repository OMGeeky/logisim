use crate::utils::Vec2CapToVec2;
use crate::utils::get_cursor_world_pos;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::color::palettes::basic::{RED, WHITE};
use bevy::math::Vec2;
use bevy::prelude::*;

const FOCUS_MARK_SIZE: f32 = 10.0;
const SHAPE_Z_POS: f32 = 0.0;
const SHAPE_FOLLOW_SPEED: f32 = 100.0;
pub struct ShapeFollowPlugin;
impl Plugin for ShapeFollowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_systems(Startup, setup_shapes)
            .add_systems(Update, show_focused_shape)
            .add_systems(FixedUpdate, focused_follow_cursor);
    }
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
    // prevent overshooting if we are already close to the target
    let capped_move_vec = capped_move_vec.cap_to_vec2(move_vec);
    transform.translation += capped_move_vec.extend(SHAPE_Z_POS);
}
