use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Add;

use macroquad::color::WHITE;
use macroquad::text::draw_multiline_text;
use macroquad::time::get_fps;
use macroquad::time::get_time;

use crate::render::Render;
use crate::render::RenderCtx;

pub struct DebugHud {
    fps_queue: RefCell<VecDeque<i32>>,
    latency_queue: RefCell<VecDeque<f64>>,
    buffer_window_size: Cell<usize>,
    previous_time: Cell<f64>,
}

impl DebugHud {
    pub fn new(buffer_window_size: usize) -> Self {
        let fps_queue = RefCell::new(VecDeque::new());
        let latency_queue = RefCell::new(VecDeque::new());
        let buffer_window_size = Cell::new(buffer_window_size);
        let previous_time = Cell::new(0.0);
        Self {
            fps_queue,
            latency_queue,
            buffer_window_size,
            previous_time,
        }
    }
}

impl Render for DebugHud {
    fn render(&self, _render_ctx: &RenderCtx) -> crate::engine::errors::GameResult<()> {
        let fps = compute_average(get_fps(), &mut self.fps_queue.borrow_mut(), 100) as i32;
        let time = get_time();
        let latency_ms = (1000.0
            * compute_average(
                time - self.previous_time.get(),
                &mut self.latency_queue.borrow_mut(),
                100,
            )) as i32;

        draw_multiline_text(
            &format!("FPS avg: {fps} | Latency avg: {latency_ms}ms",),
            20.0,
            40.0,
            30.0,
            Some(1.0),
            WHITE,
        );
        self.previous_time.set(time);
        Ok(())
    }
}

fn compute_average<T>(value: T, queue: &mut VecDeque<T>, window_size: usize) -> f64
where
    T: Copy + Add<Output = T> + Into<f64>,
{
    if queue.len() >= window_size {
        queue.pop_front();
    }
    queue.push_back(value);

    let sum: f64 = queue.iter().copied().map(Into::into).sum();
    sum / (queue.len() as f64)
}
