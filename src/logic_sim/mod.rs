use crate::camera::Canvas;
use crate::logic_sim::block_label::{BlockLabelBundle, BlockLabelPlugin};
use bevy::color::palettes::basic::{GREEN, RED, WHITE};
use bevy::prelude::*;
use bevy::text::TextBounds;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;
use std::ops::BitOr;
pub mod block_label;

const CONNECTION_SCALE_FACTOR: f32 = 10.0;
const LABEL_SCALING_FACTOR: f32 = 0.2;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Running,
}

#[derive(Component)]
#[require(Transform)]
struct Root;

#[derive(Resource)]
struct BlockDefinitionHandle(Handle<BlockDefinition>);
#[derive(Deserialize, Asset, TypePath, Debug, Clone)]
pub struct BlockDefinition {
    id: usize,
    pos: Vec2,
    size: IVec2,
    name: String,
    color: Color,
    inner_blocks: Vec<BlockDefinition>,
    wires: Vec<WireDefinition>,
    inputs: Vec<ConnectionDefinition>,
    outputs: Vec<ConnectionDefinition>,
}
#[derive(Deserialize, Asset, TypePath, Debug, Clone)]
pub struct WireDefinition {
    connections: Vec<ConnectionDefinitionRef>,
}
#[derive(Deserialize, Asset, TypePath, Debug, Clone, Copy)]
pub struct ConnectionDefinition {
    id: usize,
    value: ConnectionValues,
}
#[derive(Deserialize, Asset, TypePath, Debug, Clone, Copy)]
pub struct ConnectionDefinitionRef {
    parent_block: usize,
    id: usize,
}

#[derive(Component, Debug)]
pub struct BlockVisuals {
    size: IVec2,
    color: Color,
}
#[derive(Component, Debug, Copy, Clone)]
pub struct ConnectionReference(Entity);
#[derive(Component, Debug, Copy, Clone)]
pub struct BlockReference(Entity);
#[derive(Component, Debug)]
#[require(Transform, Mesh2d, MeshMaterial2d<ColorMaterial>)]
pub struct Block {
    id: usize,
    input_count: usize,
    output_count: usize,
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
#[require(Transform, Mesh2d, MeshMaterial2d<ColorMaterial>)]
pub struct Connection {
    index: usize,
    values: ConnectionValues,
}
#[derive(Deserialize, Debug, Clone, Copy)]
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
            ConnectionValues::X32(b) => b,
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
        app
            //
            .add_plugins(JsonAssetPlugin::<BlockDefinition>::new(&["blockdef.json"]))
            .add_plugins(BlockLabelPlugin)
            .add_systems(Startup, setup)
            .init_asset::<BlockDefinition>()
            .init_state::<AppState>()
            .add_systems(
                Update,
                spawn_block_definition_from_asset.run_if(in_state(AppState::Loading)),
            )
            .add_systems(Update, (draw_connections, draw_wires))
            .add_systems(Update, update_connection_states);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let block_def =
        BlockDefinitionHandle(asset_server.load("logisim/blocks/sample1.blockdef.json"));
    commands.insert_resource(block_def);
}

fn spawn_block_definition_from_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    block: Res<BlockDefinitionHandle>,
    mut blocks: ResMut<Assets<BlockDefinition>>,
    mut state: ResMut<NextState<AppState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawned_blocks: Query<(Entity, &Block)>,
) {
    if let Some(block) = blocks.remove(block.0.id()) {
        let spawned_block = spawned_blocks
            .iter()
            .find_map(|(e, b)| if b.id == block.id { Some(e) } else { None });
        if let Some(e) = spawned_block {
            commands.entity(e).despawn_recursive();
        }
        commands.spawn(Root).with_children(|c| {
            spawn_block_definition(c, &asset_server, &mut meshes, &mut materials, block);
        });
        state.set(AppState::Running)
    }
}

fn spawn_block_definition(
    commands: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    block: BlockDefinition,
) {
    let font = asset_server.load("fonts/arcane_nine.otf");
    let mesh = meshes.add(Rectangle::new(block.size.x as f32, block.size.y as f32));
    let block_material = materials.add(block.color);
    let rectangle = meshes.add(Rectangle::new(
        1.0 * CONNECTION_SCALE_FACTOR,
        1.0 * CONNECTION_SCALE_FACTOR,
    ));
    let connection_material = materials.add(Color::BLACK);
    let text_font = TextFont {
        font,
        font_size: 100.0,
        ..default()
    };
    let block_size = block.size.as_vec2();
    let input_count = block.inputs.len();
    let output_count = block.outputs.len();
    let mut block_id = commands.spawn((
        Name::new(format!("Block: {}", block.id)),
        Block {
            id: block.id,
            input_count,
            output_count,
        },
        Mesh2d(mesh),
        MeshMaterial2d(block_material),
        BlockVisuals {
            size: block_size.as_ivec2(),
            color: block.color,
        },
        Transform::from_translation(block.pos.extend(0.0)),
    ));
    block_id.with_child(BlockLabelBundle::new(block.name, block.size, text_font));
    let id = block_id.id();
    let inputs = block.inputs.iter().enumerate().map(|(i, input)| {
        let pos =
            get_connection_pos_from_index(block_size, input_count, ConnectionPosition::Input, i);
        (
            input.id,
            (
                Name::new(format!("Input: {}:{}", i, id)),
                Mesh2d(rectangle.clone()),
                MeshMaterial2d(connection_material.clone()),
                BlockReference(id),
                Transform::from_translation(pos.extend(2.0)),
                InputConnection,
                Connection {
                    index: i,
                    values: input.value,
                },
            ),
        )
    });
    let outputs = block.outputs.iter().enumerate().map(|(i, output)| {
        let pos =
            get_connection_pos_from_index(block_size, output_count, ConnectionPosition::Output, i);
        (
            output.id,
            (
                Name::new(format!("Output: {}:{}", i, id)),
                Mesh2d(rectangle.clone()),
                MeshMaterial2d(connection_material.clone()),
                BlockReference(id),
                OutputConnection,
                Transform::from_translation(pos.extend(1.0)),
                Connection {
                    index: i,
                    values: output.value,
                },
            ),
        )
    });

    block_id.with_children(|x| {
        let inputs: Vec<_> = inputs
            .map(|(id, con)| (id, ConnectionReference(x.spawn(con).id())))
            .collect();
        let outputs: Vec<_> = outputs
            .map(|(id, con)| (id, ConnectionReference(x.spawn(con).id())))
            .collect();

        let child_blocks = block.inner_blocks.iter().map(|block| {
            spawn_block_definition(x, asset_server, meshes, materials, block.clone());
        });

        let wires = block
            .wires
            .iter()
            .map(|wire| {
                wire.connections
                    .iter()
                    .flat_map(|con| {
                        if (con.parent_block != block.id) {
                            warn!("get connections from sub blocks is not implemented yet");
                            // todo!("get connections from sub blocks (not blocks outside the one containing the wire and only one level deep)");
                            None
                        } else if let Some((_, connection)) =
                            inputs.iter().find(|(id, _)| *id == con.id)
                        {
                            Some(*connection)
                        } else if let Some((_, connection)) =
                            outputs.iter().find(|(id, _)| *id == con.id)
                        {
                            Some(*connection)
                        } else {
                            warn!(
                                "could not find connection with id '{}' in block '{}",
                                con.id, block.id
                            );
                            None
                        }
                    })
                    .collect()
            })
            .enumerate()
            .map(|(i, connections)| (Wire { connections }, Name::new(format!("Wire: {}", i))));

        for wire in wires {
            x.spawn(wire);
        }
    });
}
#[derive(Debug, Copy, Clone)]
enum ConnectionPosition {
    Input,
    Output,
}
fn get_connection_pos_from_index(
    size: Vec2,
    count: usize,
    anchor: ConnectionPosition,
    index: usize,
) -> Vec2 {
    let size = size;
    let half_size = size / 2.0;
    let top = half_size.y;
    let left = -half_size.x;
    let right = half_size.x;
    let pos;
    match anchor {
        ConnectionPosition::Input => {
            pos = Vec2::new(left, top);
        }
        ConnectionPosition::Output => {
            pos = Vec2::new(right, top);
        }
    }
    let spacing = size.y / (count + 1) as f32;
    let pos = pos - Vec2::Y * spacing * (index + 1) as f32;
    pos
}

fn draw_connections(
    connections: Query<(&Connection, &GlobalTransform)>,
    canvas: Res<Canvas>,
    mut gizmos: Gizmos,
) {
    for (connection, transform) in connections.iter() {
        let pos = transform.translation().xy();
        draw_connection(pos, connection, transform.scale(), &mut gizmos);
    }
}

fn draw_connection(pos: Vec2, connection: &Connection, scale: Vec3, gizmos: &mut Gizmos) {
    const CONNECTION_BIT_SIZE: f32 = 10.0;
    let connection_bit_size = CONNECTION_BIT_SIZE * scale.xy();
    let connection_bit_half_size_x = connection_bit_size.x * 0.5;

    let size = connection.values.len() as u32;
    let rows = if size > 8 { 2 } else { 1 };
    let columns = (size as f32 / rows as f32).ceil() as u32;

    let half_offset = Vec2::new(columns as f32, rows as f32) * (connection_bit_size / 2.0);
    let half_one_size = connection_bit_size / 2.0;

    'rows: for y in 0..rows {
        for x in 0..columns {
            let index = y * columns + x;
            if index >= size {
                break 'rows;
            }
            let pos = pos + Vec2::new((columns - x - 1) as f32, (y) as f32) * connection_bit_size
                - half_offset
                + half_one_size;
            let value = connection.values.get_by_index(index as usize);
            let color = if value { GREEN } else { RED };
            gizmos.circle_2d(pos, connection_bit_half_size_x, color);
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
