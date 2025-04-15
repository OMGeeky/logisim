use crate::camera::Canvas;
use bevy::color::palettes::basic::{GREEN, RED, WHITE};
use bevy::prelude::*;
use bevy::text::TextBounds;
use std::ops::BitOr;

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
pub struct Block {
    inputs: Vec<ConnectionReference>,
    outputs: Vec<ConnectionReference>,
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
    values: ConnectionValues,
}
#[derive(Debug, Clone, Copy)]
pub enum ConnectionValues {
    Single(bool),
    HalfByte(bool, bool, bool, bool),
    Byte(u8),
    X16(u16),
    X32(u32),
    X64(u64),
    X128(u128),
    X256(u128, u128),
}

impl ConnectionValues {
    pub fn len(&self) -> usize {
        match self {
            ConnectionValues::Single(_) => 1,
            ConnectionValues::HalfByte(_, _, _, _) => 4,
            ConnectionValues::Byte(_) => 8,
            ConnectionValues::X16(_) => 16,
            ConnectionValues::X32(_) => 32,
            ConnectionValues::X64(_) => 64,
            ConnectionValues::X128(_) => 128,
            ConnectionValues::X256(_, _) => 256,
        }
    }
    //region inner_x
    fn inner_u128(self) -> u128 {
        if self.len() < 128 {
            return self.inner_u64() as u128;
        }
        match self {
            ConnectionValues::Single(_)
            | ConnectionValues::HalfByte(_, _, _, _)
            | ConnectionValues::Byte(_)
            | ConnectionValues::X16(_)
            | ConnectionValues::X32(_)
            | ConnectionValues::X64(_) => unreachable!(),
            ConnectionValues::X128(b) => b,
            ConnectionValues::X256(b, _) => b,
        }
    }
    fn inner_u64(self) -> u64 {
        if self.len() < 64 {
            return self.inner_u32() as u64;
        }
        match self {
            ConnectionValues::Single(_)
            | ConnectionValues::HalfByte(_, _, _, _)
            | ConnectionValues::Byte(_)
            | ConnectionValues::X16(_)
            | ConnectionValues::X32(_) => unreachable!(),
            ConnectionValues::X64(b) => b,
            ConnectionValues::X128(b) => b as u64,
            ConnectionValues::X256(b, _) => b as u64,
        }
    }
    fn inner_u32(self) -> u32 {
        if self.len() < 32 {
            return self.inner_u16() as u32;
        }
        match self {
            ConnectionValues::Single(_)
            | ConnectionValues::HalfByte(_, _, _, _)
            | ConnectionValues::Byte(_)
            | ConnectionValues::X16(_) => unreachable!(),
            ConnectionValues::X32(b) => b as u32,
            ConnectionValues::X64(b) => b as u32,
            ConnectionValues::X128(b) => b as u32,
            ConnectionValues::X256(b, _) => b as u32,
        }
    }
    fn inner_u16(self) -> u16 {
        if self.len() < 16 {
            return self.inner_u8() as u16;
        }
        match self {
            ConnectionValues::Single(_)
            | ConnectionValues::HalfByte(_, _, _, _)
            | ConnectionValues::Byte(_) => unreachable!(),
            ConnectionValues::X16(b) => b,
            ConnectionValues::X32(b) => b as u16,
            ConnectionValues::X64(b) => b as u16,
            ConnectionValues::X128(b) => b as u16,
            ConnectionValues::X256(b, _) => b as u16,
        }
    }
    fn inner_u8(self) -> u8 {
        match self {
            ConnectionValues::Single(b) => b as u8,
            ConnectionValues::HalfByte(b0, b1, b2, b3) => {
                (b0 as u8) | (b1 as u8) << 1 | (b2 as u8) << 2 | (b3 as u8) << 3
            }
            ConnectionValues::Byte(b) => b,
            ConnectionValues::X16(b) => b as u8,
            ConnectionValues::X32(b) => b as u8,
            ConnectionValues::X64(b) => b as u8,
            ConnectionValues::X128(b) => b as u8,
            ConnectionValues::X256(b, _) => b as u8,
        }
    }
    //endregion

    pub(crate) fn set_by_index(&mut self, index: usize, value: bool) {
        if index >= self.len() {
            warn!("Tried writing out of bounds. Index: '{index}' ConnectionValues: '{self:?}'");
            return; // reasonable fallback for out of bounds writing
        }
        match self {
            ConnectionValues::Single(b) => *b = value,
            ConnectionValues::HalfByte(b0, b1, b2, b3) => match index {
                0 => *b0 = value,
                1 => *b1 = value,
                2 => *b2 = value,
                3 => *b3 = value,
                _ => unreachable!(), // Should be caught by the initial length check
            },
            ConnectionValues::Byte(byte) => {
                if value {
                    *byte |= 1 << index;
                } else {
                    *byte &= !(1 << index);
                }
            }
            ConnectionValues::X16(val) => {
                if value {
                    *val |= 1 << index;
                } else {
                    *val &= !(1 << index);
                }
            }
            ConnectionValues::X32(val) => {
                if value {
                    *val |= 1 << index;
                } else {
                    *val &= !(1 << index);
                }
            }
            ConnectionValues::X64(val) => {
                if value {
                    *val |= 1 << index;
                } else {
                    *val &= !(1 << index);
                }
            }
            ConnectionValues::X128(val) => {
                if value {
                    *val |= 1 << index;
                } else {
                    *val &= !(1 << index);
                }
            }
            ConnectionValues::X256(val1, val2) => {
                let val = if index < 128 { val1 } else { val2 };
                if value {
                    *val |= 1 << index;
                } else {
                    *val &= !(1 << index);
                }
            }
        }
    }
    pub(crate) fn get_by_index(self, index: usize) -> bool {
        if index >= self.len() {
            warn!("Tried reading out of bounds. Index: '{index}' ConnectionValues: '{self:?}'");
            return false; // reasonable fallback for out of bounds reading
        }

        match self {
            ConnectionValues::Single(b) => b,
            ConnectionValues::HalfByte(b0, b1, b2, b3) => match index {
                0 => b0,
                1 => b1,
                2 => b2,
                3 => b3,
                _ => unreachable!(), // Should be caught by the initial length check
            },
            ConnectionValues::Byte(byte) => (byte >> index) & 1 != 0,
            ConnectionValues::X16(val) => (val >> index) & 1 != 0,
            ConnectionValues::X32(val) => (val >> index) & 1 != 0,
            ConnectionValues::X64(val) => (val >> index) & 1 != 0,
            ConnectionValues::X128(val) => (val >> index) & 1 != 0,
            ConnectionValues::X256(val1, val2) => {
                if index < 128 {
                    (val1 >> index) & 1 != 0
                } else {
                    (val2 >> (index - 128)) & 1 != 0
                }
            }
        }
    }
}

impl BitOr for ConnectionValues {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let (left, right);
        if self.len() > rhs.len() {
            left = self;
            right = rhs;
        } else {
            left = rhs;
            right = self;
        }
        assert!(left.len() >= right.len());

        match left {
            ConnectionValues::Single(left) => match right {
                ConnectionValues::Single(right) => ConnectionValues::Single(left | right),
                _ => unreachable!("The right side should be smaller or equal than the left side"),
            },
            ConnectionValues::HalfByte(left0, left1, left2, left3) => match right {
                ConnectionValues::Single(right) => {
                    ConnectionValues::HalfByte(left0 | right, left1, left2, left3)
                }
                ConnectionValues::HalfByte(right0, right1, right2, right3) => {
                    ConnectionValues::HalfByte(
                        left0 | right0,
                        left1 | right1,
                        left2 | right2,
                        left3 | right3,
                    )
                }
                _ => unreachable!("The right side should be smaller or equal than the left side"),
            },
            ConnectionValues::Byte(left) => {
                let right = right.inner_u8();
                ConnectionValues::Byte(left | right)
            }
            ConnectionValues::X16(left) => {
                let right = right.inner_u16();
                ConnectionValues::X16(left | right)
            }
            ConnectionValues::X32(left) => {
                let right = right.inner_u32();
                ConnectionValues::X32(left | right)
            }
            ConnectionValues::X64(left) => {
                let right = right.inner_u64();
                ConnectionValues::X64(left | right)
            }
            ConnectionValues::X128(left) => {
                let right = right.inner_u128();
                ConnectionValues::X128(left | right)
            }
            ConnectionValues::X256(left0, left1) => {
                if let ConnectionValues::X256(right1, right2) = right {
                    ConnectionValues::X256(left0 | right1, left1 | right2)
                } else {
                    let right = right.inner_u128();
                    ConnectionValues::X256(left0 | right, left1)
                }
            }
        }
    }
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
            values: ConnectionValues::HalfByte(false, true, false, true),
        },
    );
    let input2 = (
        InputConnection,
        Connection {
            values: ConnectionValues::HalfByte(true, false, false, true),
        },
    );
    let output1 = (
        OutputConnection,
        Connection {
            values: ConnectionValues::HalfByte(false, false, true, false),
        },
    );
    let output2 = (
        OutputConnection,
        Connection {
            values: ConnectionValues::HalfByte(false, false, true, false),
        },
    );
    let output3 = (
        OutputConnection,
        Connection {
            values: ConnectionValues::HalfByte(false, false, true, false),
        },
    );
    let output4 = (
        OutputConnection,
        Connection {
            values: ConnectionValues::HalfByte(false, true, false, false),
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
            ConnectionReference(input2),
            ConnectionReference(output1),
            ConnectionReference(output2),
            ConnectionReference(output4),
        ],
    };
    commands.spawn(wire1);
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
            },
            global_transform: GlobalTransform::default(),
            transform: Transform::default(),
        })
        .with_child(BlockLabelBundle::new("AND", size, text_font));
}
fn render_blocks(
    blocks: Query<(&BlockVisuals, &Transform)>,
    canvas: Res<Canvas>,
    mut gizmos: Gizmos,
) {
    for (block_visual, transform) in blocks.iter() {
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
            block.inputs.iter(),
            &mut connections,
            block.inputs.len(),
        );
        update_connection_position(
            Vec2::new(right, top),
            size.y,
            block.outputs.iter(),
            &mut connections,
            block.outputs.len(),
        );
    }
}
fn update_connection_position<'a>(
    pos: Vec2,
    available_height: f32,
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

    let size = connection.values.len() as u32;
    let rows = if size > 8 { 2 } else { 1 };
    let columns = (size as f32 / rows as f32).ceil() as u32;

    let half_offset = Vec2::new(columns as f32, rows as f32) * (connection_bit_size / 2.0);
    let one_size = Vec2::new(connection_bit_size, connection_bit_size);
    let half_one_size = one_size / 2.0;

    'rows: for y in 0..rows {
        for x in 0..columns {
            let index = y * columns + x;
            if index >= size {
                break 'rows;
            }
            let pos = pos + Vec2::new(x as f32, (rows - y - 1) as f32) * connection_bit_size
                - half_offset
                + half_one_size;
            let value = connection.values.get_by_index(index as usize);
            let color = if value { GREEN } else { RED };
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
    wires: Query<&Wire>,
    connections: Query<&GlobalTransform, With<Connection>>,
    mut gizmos: Gizmos,
) {
    for wire in wires.iter() {
        let connection_positions: Vec<Vec2> = wire
            .connections
            .iter()
            .filter_map(|connection| {
                connections
                    .get(connection.0)
                    .map(|transform| transform.translation().xy())
                    .ok()
            })
            .collect();
        let average_pos = connection_positions
            .iter()
            .fold(Vec2::ZERO, |sum, pos| sum + pos)
            / connection_positions.len() as f32;
        for connection_pos in connection_positions {
            gizmos.line_2d(average_pos, connection_pos, WHITE);
        }
    }
}

fn scale_labels(mut labels: Query<&mut Transform, With<CanvasText>>, canvas: Res<Canvas>) {
    for mut transform in labels.iter_mut() {
        transform.scale = Vec3::splat(canvas.zoom) * LABEL_SCALING_FACTOR;
    }
}

fn update_connection_states(
    wires: Query<&Wire>,
    mut connections: Query<(
        &mut Connection,
        Option<&InputConnection>,
        Option<&OutputConnection>,
    )>,
) {
    for wire in wires.iter() {
        let input_value: ConnectionValues = wire
            .connections
            .iter()
            .filter_map(|connection| {
                let conn = connections.get(connection.0).unwrap();
                if conn.1.is_some() {
                    Some(conn.0.values)
                } else {
                    None
                }
            })
            .fold(ConnectionValues::Single(false), BitOr::bitor);
        for output in wire.connections.iter() {
            if let Ok((mut output, _, output_marker)) = connections.get_mut(output.0) {
                if output_marker.is_none() {
                    continue;
                }
                output.values = input_value;
            }
        }
    }
}

#[cfg(test)]
mod tests;
