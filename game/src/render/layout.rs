use super::widgets::grid::GridWidget;

pub struct Layout {
    pub grid: GridWidget,
}

impl Layout {
    pub fn new(grid: GridWidget) -> Self {
        Layout { grid }
    }
}
