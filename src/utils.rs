use bevy::math::Vec2;
use bevy::prelude::{Camera, GlobalTransform, Query, Single, Window};

pub fn get_cursor_world_pos(
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

pub trait Vec2CapToVec2 {
    fn cap_to_vec2(self, max: Vec2) -> Vec2;
}
impl Vec2CapToVec2 for Vec2 {
    fn cap_to_vec2(self, max: Vec2) -> Vec2 {
        let mut capped_move_vec = self;
        if (self.x > 0.0 && self.x > max.x) || (self.x < 0.0 && self.x < max.x) {
            capped_move_vec.x = max.x;
        }
        if (self.y > 0.0 && capped_move_vec.y > max.y)
            || (self.y < 0.0 && capped_move_vec.y < max.y)
        {
            capped_move_vec.y = max.y;
        }
        capped_move_vec
    }
}
