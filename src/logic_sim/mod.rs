use crate::camera::Canvas;
use bevy::color::palettes::basic::{GREEN, RED, WHITE};
use bevy::prelude::*;
use bevy::text::TextBounds;

const LABEL_SCALING_FACTOR: f32 = 0.2;

#[derive(Bundle, Debug)]
pub struct BlockBundle {
    block: Block,
    transform: Transform,
    global_transform: GlobalTransform,
    block_visuals: BlockVisuals,
}
#[derive(Bundle, Debug)]
pub struct BlockLabelBundle {
    text: Text2d,
    font: TextFont,
    text_layout: TextLayout,
    text_bounds: TextBounds,
    transform: Transform,
    marker: CanvasText,
}

impl BlockLabelBundle {
    fn new(name: impl Into<String>, size: IVec2, font: TextFont) -> Self {
        Self {
            text: Text2d(name.into()),
            font,
            text_layout: TextLayout::new(JustifyText::Justified, LineBreak::WordOrCharacter),
            text_bounds: TextBounds::from(size.as_vec2() * (1.0 / LABEL_SCALING_FACTOR)),
            transform: Transform::from_translation(Vec3::Z),
            marker: CanvasText,
        }
    }
}

#[derive(Component, Debug)]
pub struct CanvasText;
#[derive(Component, Debug)]
pub struct BlockVisuals {
    size: IVec2,
    color: Color,
}
#[derive(Component, Debug)]
pub struct Block {
    inputs: Vec<Connection>,
    outputs: Vec<Connection>,
}
#[derive(Debug)]
pub struct Connection {
    size: u32,
    values: Vec<bool>,
}

pub struct LogicSimPlugin;
impl Plugin for LogicSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (render_blocks, scale_labels));
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/arcane_nine.otf");
    let text_font = TextFont {
        font,
        font_size: 100.0,
        ..default()
    };

    let size = IVec2::new(50, 80);
    commands
        .spawn(BlockBundle {
            // name: Text2d("AND".into()),
            block_visuals: BlockVisuals {
                size,
                color: Color::from(RED),
            },
            block: Block {
                inputs: vec![
                    Connection {
                        size: 10,
                        values: vec![
                            false, true, false, true, false, true, false, true, false, true,
                        ],
                    },
                    Connection {
                        size: 9,
                        values: vec![false, true, false, true, false, true, false, true, false],
                    },
                ],
                outputs: vec![
                    Connection {
                        size: 1,
                        values: vec![false],
                    },
                    Connection {
                        size: 2,
                        values: vec![true, false],
                    },
                    Connection {
                        size: 3,
                        values: vec![true, false, true],
                    },
                    Connection {
                        size: 4,
                        values: vec![true, false, true, false],
                    },
                ],
            },
            global_transform: GlobalTransform::default(),
            transform: Transform::default(),
        })
        .with_child(BlockLabelBundle::new("AND", size, text_font));
}
fn render_blocks(
    blocks: Query<(&BlockVisuals, &Block, &GlobalTransform)>,
    canvas: Res<Canvas>,
    mut gizmos: Gizmos,
) {
    for (block_visual, block, transform) in blocks.iter() {
        let size = block_visual.size.as_vec2() * canvas.zoom;
        let center = transform.translation().xy();
        let half_size = size / 2.0;
        let top = center.y + half_size.y;
        let left = center.x - half_size.x;
        let right = center.x + half_size.x;
        gizmos.rect_2d(center, size, block_visual.color);
        draw_connections(left, top, size.y, &block.inputs, &mut gizmos, &canvas);
        draw_connections(right, top, size.y, &block.outputs, &mut gizmos, &canvas);
    }
}
fn draw_connections(
    x: f32,
    y: f32,
    available_height: f32,
    connections: &[Connection],
    gizmos: &mut Gizmos,
    canvas: &Canvas,
) {
    let input_count = connections.len();
    let input_spacing = available_height / (input_count + 1) as f32;
    // println!("{} -> {}", input_count, input_spacing);
    for (i, connection) in connections.iter().enumerate() {
        draw_connection(x, y, gizmos, input_spacing, i, connection, canvas);
    }
}

fn draw_connection(
    x: f32,
    y: f32,
    gizmos: &mut Gizmos,
    input_spacing: f32,
    i: usize,

    connection: &Connection,
    canvas: &Canvas,
) {
    const CONNECTION_BIT_SIZE: f32 = 10.0;
    let connection_bit_size = CONNECTION_BIT_SIZE * canvas.zoom;
    let connection_bit_half_size = connection_bit_size * 0.5;
    let connection_pos = Vec2::new(x, y - input_spacing * (i + 1) as f32);

    let rows = if connection.size > 8 { 2 } else { 1 };
    let columns = (connection.size as f32 / rows as f32).ceil() as u32;

    let half_offset = Vec2::new(columns as f32, rows as f32) * (connection_bit_size / 2.0);
    let one_size = Vec2::new(connection_bit_size, connection_bit_size);
    let half_one_size = one_size / 2.0;

    'rows: for y in 0..rows {
        for x in 0..columns {
            let index = y * columns + x;
            if index >= connection.size {
                break 'rows;
            }
            let pos = connection_pos
                + Vec2::new(x as f32, (rows - y - 1) as f32) * connection_bit_size
                - half_offset
                + half_one_size;
            let value = connection.values[index as usize];
            let color = if value { RED } else { GREEN };
            gizmos.circle_2d(pos, connection_bit_half_size, color);
        }
    }
    gizmos.rect_2d(
        connection_pos,
        Vec2::new(columns as f32, rows as f32) * connection_bit_size,
        WHITE,
    );
}

fn scale_labels(mut labels: Query<&mut Transform, With<CanvasText>>, canvas: Res<Canvas>) {
    for mut transform in labels.iter_mut() {
        transform.scale = Vec3::splat(canvas.zoom) * LABEL_SCALING_FACTOR;
    }
}
