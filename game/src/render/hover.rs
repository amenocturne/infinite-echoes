use macroquad::math::Vec2;

pub trait Hover {
    fn is_hovered_over(&self, relative_mouse_position: Vec2) -> bool;
}
