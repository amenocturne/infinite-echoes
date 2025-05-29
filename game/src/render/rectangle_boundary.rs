use macroquad::math::vec2;
use macroquad::math::Vec2;

pub trait RectangleBoundary {
    fn center(&self) -> Vec2;
    fn size(&self) -> Vec2;

    fn top_left(&self) -> Vec2 {
        Self::top_left_from(self.center(), self.size())
    }

    fn top_left_from(center: Vec2, size: Vec2) -> Vec2 {
        center - size / 2.0
    }

    fn left_center(&self) -> Vec2 {
        Self::left_center_from(self.center(), self.size())
    }
    fn left_center_from(center: Vec2, size: Vec2) -> Vec2 {
        center - size / 2.0
    }

    fn grid_centers(&self, columns: u32, rows: u32) -> Vec<Vec2> {
        Self::grid_centers_from(self.center(), self.size(), columns, rows)
    }

    fn grid_centers_from(center: Vec2, size: Vec2, columns: u32, rows: u32) -> Vec<Vec2> {
        let grid_size = size / vec2(columns as f32, rows as f32);
        let first_center = Self::top_left_from(center, size) + grid_size / 2.0;

        let mut centers = vec![];
        for c in 0..columns {
            for r in 0..rows {
                centers.push(first_center + grid_size * vec2(c as f32, r as f32))
            }
        }
        centers
    }
}
