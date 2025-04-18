use super::*;
pub struct BlockLabelPlugin;
impl Plugin for BlockLabelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scale_labels);
    }
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
    pub fn new(name: impl Into<String>, size: IVec2, font: TextFont) -> Self {
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
fn scale_labels(mut labels: Query<&mut Transform, With<CanvasText>>, canvas: Res<Canvas>) {
    labels.par_iter_mut().for_each(|mut transform| {
        transform.scale = Vec3::splat(canvas.zoom) * LABEL_SCALING_FACTOR;
    });
}
