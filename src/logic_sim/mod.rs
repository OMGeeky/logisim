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
pub struct ConnectionReference(Entity);
#[derive(Component, Debug)]
pub struct WireReference(Entity);
#[derive(Component, Debug)]
pub struct Block {
    inputs: Vec<ConnectionReference>,
    outputs: Vec<ConnectionReference>,
    wires: Vec<WireReference>,
}
#[derive(Component, Debug)]
#[require(Transform)]
pub struct Wire {
    connections: Vec<ConnectionReference>,
}
#[derive(Component, Debug)]
pub struct InputConnection;
#[derive(Component, Debug)]
pub struct OutputConnection;

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Connection {
    size: u32,
    values: Vec<bool>,
}

pub struct LogicSimPlugin;
impl Plugin for LogicSimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                (update_connection_positions, draw_connections).chain(),
                render_blocks,
                scale_labels,
                draw_wires,
                update_connection_states,
            ),
        );
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

    //region connections
    let input1 = (
        InputConnection,
        Connection {
            size: 1,
            values: vec![false],
        },
    );
    let input2 = (
        InputConnection,
        Connection {
            size: 1,
            values: vec![true],
        },
    );
    let output1 = (
        OutputConnection,
        Connection {
            size: 1,
            values: vec![false],
        },
    );
    let output2 = (
        OutputConnection,
        Connection {
            size: 1,
            values: vec![true],
        },
    );
    let output3 = (
        OutputConnection,
        Connection {
            size: 1,
            values: vec![true],
        },
    );
    let output4 = (
        OutputConnection,
        Connection {
            size: 1,
            values: vec![false],
        },
    );
    let input1 = commands.spawn(input1).id();
    let input2 = commands.spawn(input2).id();
    let output1 = commands.spawn(output1).id();
    let output2 = commands.spawn(output2).id();
    let output3 = commands.spawn(output3).id();
    let output4 = commands.spawn(output4).id();
    //endregion
    let wire1 = Wire {
        connections: vec![
            ConnectionReference(input1),
            ConnectionReference(output1),
            ConnectionReference(output2),
        ],
    };
    let wire1 = commands.spawn(wire1).id();
    commands
        .spawn(BlockBundle {
            block_visuals: BlockVisuals {
                size,
                color: Color::from(RED),
            },
            block: Block {
                inputs: vec![ConnectionReference(input1), ConnectionReference(input2)],
                outputs: vec![
                    ConnectionReference(output1),
                    ConnectionReference(output2),
                    ConnectionReference(output3),
                    ConnectionReference(output4),
                ],
                wires: vec![WireReference(wire1)],
            },
            global_transform: GlobalTransform::default(),
            transform: Transform::default(),
        })
        .with_child(BlockLabelBundle::new("AND", size, text_font));
}
fn render_blocks(
    blocks: Query<(&BlockVisuals, &Block, &Transform)>,
    connections: Query<&mut Connection>,
    canvas: Res<Canvas>,
    mut gizmos: Gizmos,
) {
    for (block_visual, block, transform) in blocks.iter() {
        let size = block_visual.size.as_vec2() * canvas.zoom;
        let center = transform.translation.xy();
        gizmos.rect_2d(center, size, block_visual.color);
    }
}

fn update_connection_positions(
    blocks: Query<(&BlockVisuals, &Block, &GlobalTransform)>,
    mut connections: Query<&mut Transform, With<Connection>>,
    canvas: Res<Canvas>,
) {
    for (block_visual, block, transform) in blocks.iter() {
        let size = block_visual.size.as_vec2() * canvas.zoom;
        let center = transform.translation().xy();
        let half_size = size / 2.0;
        let top = center.y + half_size.y;
        let left = center.x - half_size.x;
        let right = center.x + half_size.x;

        update_connection_position(
            Vec2::new(left, top),
            size.y,
            &canvas,
            block.inputs.iter(),
            &mut connections,
            block.inputs.len(),
        );
        update_connection_position(
            Vec2::new(right, top),
            size.y,
            &canvas,
            block.outputs.iter(),
            &mut connections,
            block.outputs.len(),
        );
    }
}
fn update_connection_position<'a>(
    pos: Vec2,
    available_height: f32,
    canvas: &Canvas,
    connection_refs: impl Iterator<Item = &'a ConnectionReference>,
    connections: &mut Query<&mut Transform, With<Connection>>,
    connections_count: usize,
) {
    let spacing = available_height / (connections_count + 1) as f32;

    for (i, connection) in connection_refs.enumerate() {
        let pos = pos - Vec2::Y * spacing * (i + 1) as f32;
        if let Ok(mut transform) = connections.get_mut(connection.0) {
            transform.translation = pos.extend(0.0);
        }
    }
}
fn draw_connections(
    connections: Query<(&Connection, &GlobalTransform)>,
    canvas: Res<Canvas>,
    mut gizmos: Gizmos,
) {
    for (connection, transform) in connections.iter() {
        let pos = transform.translation().xy();
        draw_connection(pos, connection, &canvas, &mut gizmos);
    }
}
fn draw_connection(pos: Vec2, connection: &Connection, canvas: &Canvas, gizmos: &mut Gizmos) {
    const CONNECTION_BIT_SIZE: f32 = 10.0;
    let connection_bit_size = CONNECTION_BIT_SIZE * canvas.zoom;
    let connection_bit_half_size = connection_bit_size * 0.5;

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
            let pos = pos + Vec2::new(x as f32, (rows - y - 1) as f32) * connection_bit_size
                - half_offset
                + half_one_size;
            let value = connection.values[index as usize];
            let color = if value { RED } else { GREEN };
            gizmos.circle_2d(pos, connection_bit_half_size, color);
        }
    }
    gizmos.rect_2d(
        pos,
        Vec2::new(columns as f32, rows as f32) * connection_bit_size,
        WHITE,
    );
}
fn draw_wires(
    wires: Query<(&Wire, &GlobalTransform)>,
    connections: Query<&GlobalTransform, With<Connection>>,
    mut gizmos: Gizmos,
) {
    for (wire, transform) in wires.iter() {
        let wire_root = transform.translation().xy();
        for connection in wire
            .connections
            .iter()
            .map(|connection| connections.get(connection.0))
        {
            if let Ok(connection) = connection {
                gizmos.line_2d(wire_root, connection.translation().xy(), WHITE);
            } else {
                println!("connection not found: {:?}", connection);
            }
        }
    }
}

fn scale_labels(mut labels: Query<&mut Transform, With<CanvasText>>, canvas: Res<Canvas>) {
    for mut transform in labels.iter_mut() {
        transform.scale = Vec3::splat(canvas.zoom) * LABEL_SCALING_FACTOR;
    }
}

fn update_connection_states(
    wires: Query<(&Wire)>,
    mut connections: Query<(
        &mut Connection,
        Option<&InputConnection>,
        Option<&OutputConnection>,
    )>,
) {
    for wire in wires.iter() {
        let inputs = wire.connections.iter().find_map(|connection| {
            let conn = connections.get(connection.0).unwrap();
            if conn.1.is_some() { Some(conn.0) } else { None }
        });
        //TODO: implement multiple inputs on one wire (use filter_map instead of find_map and then combine the values (binary-or?))

        if let Some(input) = inputs {
            let values = input.values.clone();
            for output in wire.connections.iter() {
                if let Ok((mut output, _, output_marker)) = connections.get_mut(output.0) {
                    if output_marker.is_none() {
                        continue;
                    }
                    output.values = values.clone();
                }
            }
        }
    }
}
