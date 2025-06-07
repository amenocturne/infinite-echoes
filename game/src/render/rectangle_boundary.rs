use macroquad::math::vec2;
use macroquad::math::Vec2;

pub trait RectangleBoundary {
    fn center(&self) -> Vec2;
    fn size(&self) -> Vec2;

    fn is_inside(&self, position: Vec2) -> bool {
        Self::is_inside_from(self.top_left(), self.bottom_right(), position)
    }

    fn is_inside_from(top_left: Vec2, bottom_right: Vec2, position: Vec2) -> bool {
        top_left.x < position.x
            && position.x < bottom_right.x
            && top_left.y < position.y
            && position.y < bottom_right.y
    }

    fn top_left(&self) -> Vec2 {
        Self::top_left_from(self.center(), self.size())
    }

    fn top_right(&self) -> Vec2 {
        self.center() + vec2(self.size().x, -self.size().y) / 2.0
    }

    fn bottom_left(&self) -> Vec2 {
        self.center() + vec2(-self.size().x, self.size().y) / 2.0
    }

    fn bottom_right(&self) -> Vec2 {
        self.center() + self.size() / 2.0
    }

    fn left_center(&self) -> Vec2 {
        Self::left_center_from(self.center(), self.size())
    }

    fn right_center(&self) -> Vec2 {
        self.center() + vec2(self.size().x, 0.0) / 2.0
    }

    fn grid_centers(&self, columns: u32, rows: u32) -> Vec<Vec2> {
        Self::grid_centers_from(self.center(), self.size(), columns, rows)
    }

    fn left_center_from(center: Vec2, size: Vec2) -> Vec2 {
        center - vec2(size.x / 2.0, 0.0)
    }

    fn top_left_from(center: Vec2, size: Vec2) -> Vec2 {
        center - size / 2.0
    }

    fn grid_centers_from(center: Vec2, size: Vec2, columns: u32, rows: u32) -> Vec<Vec2> {
        let grid_size = size / vec2(columns as f32, rows as f32);
        let first_center = Self::top_left_from(center, size) + grid_size / 2.0;

        let mut centers = vec![];
        for r in 0..rows {
            for c in 0..columns {
                centers.push(first_center + grid_size * vec2(c as f32, r as f32))
            }
        }
        centers
    }
}

pub fn is_inside_rectangle(top_left: &Vec2, bottom_right: &Vec2, position: &Vec2) -> bool {
    top_left.x < position.x
        && position.x < bottom_right.x
        && top_left.y < position.y
        && position.y < bottom_right.y
}
