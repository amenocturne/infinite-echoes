use macroquad::math::Vec2;

pub trait Hover {
    fn is_hovered_over(&self, relative_mouse_position: Vec2) -> bool;
}

pub fn is_inside(top_left: Vec2, bottom_right: Vec2, position: Vec2) -> bool {
    top_left.x < position.x
        && position.x < bottom_right.x
        && top_left.y < position.y
        && position.y < bottom_right.y
}
