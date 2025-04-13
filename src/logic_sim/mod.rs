use bevy::color::palettes::basic::RED;
// pub struct Block
use bevy::prelude::*;
use bevy::text::TextBounds;

#[derive(Bundle, Debug)]
pub struct BlockBundle {
    // name: Text2d,
    block: Block,
    transform: Transform,
    global_transform: GlobalTransform,
    block_visuals: BlockVisuals,
}

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
            .add_systems(Update, render_blocks);
    }
}
fn setup(mut commands: Commands) {
    let size = IVec2::new(50, 40);
    commands
        .spawn(BlockBundle {
            // name: Text2d("AND".into()),
            block_visuals: BlockVisuals {
                size,
                color: Color::from(RED),
            },
            block: Block {
                inputs: vec![Connection {
                    size: 2,
                    values: vec![false, true],
                }],
                outputs: vec![
                    Connection {
                        size: 1,
                        values: vec![false],
                    },
                    Connection {
                        size: 1,
                        values: vec![true],
                    },
                ],
            },
            global_transform: GlobalTransform::default(),
            transform: Transform::default(),
        })
        .with_child((
            Text2d("AND".into()),
            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
            TextBounds::from(size.as_vec2()),
            Transform::from_translation(Vec3::Z),
        ));
}
fn render_blocks(blocks: Query<(&BlockVisuals, &GlobalTransform)>, mut gizmos: Gizmos) {
    for (block_visual, transform) in blocks.iter() {
        gizmos.rect_2d(
            transform.translation().xy(),
            block_visual.size.as_vec2(),
            block_visual.color,
        );
    }
}
