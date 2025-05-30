use super::draggable_card_buffer::DraggableCardBuffer;
use super::widgets::card_widget::Card;
use super::Render;
use super::RenderCtx;
use crate::engine::errors::GameResult;
use macroquad::math::Vec2;
use miniquad::info;
use std::cell::RefCell;

#[derive(Debug)]
pub struct DragManager {
    state: Option<DragState>,
}

#[derive(Debug)]
pub struct DragState {
    dragged_card: RefCell<Card>,
    original_buffer_pos: usize,
    original_card_location: usize,
}

impl DragState {
    pub fn new(
        dragged_card: RefCell<Card>,
        original_buffer_pos: usize,
        original_card_location: usize,
    ) -> Self {
        Self {
            dragged_card,
            original_buffer_pos,
            original_card_location,
        }
    }
}

impl DragManager {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn handle_mouse_press(
        &mut self,
        mouse_pos: Vec2,
        buffers: &mut [&mut dyn DraggableCardBuffer],
    ) {
        if self.state.is_some() {
            return; // Already dragging something
        }

        // Extract the dragged card if any buffer has one
        for (pos, buffer) in buffers.iter_mut().enumerate() {
            if let Some((loc, card)) = buffer.try_start_dragging(mouse_pos) {
                self.state = Some(DragState::new(card, pos, loc));
                break; // Found a card to drag, stop looking
            }
        }
    }

    pub fn handle_mouse_drag(&mut self, mouse_pos: Vec2) {
        if let Some(ref state) = self.state {
            info!("Updating dragged card position to: {:?}", mouse_pos);
            state
                .dragged_card
                .borrow_mut()
                .update_dragged_position(mouse_pos);
        }
    }

    pub fn handle_mouse_release(&mut self, buffers: &mut [&mut dyn DraggableCardBuffer]) {
        if let Some(ref state) = self.state {
            let mut placed = false;

            // Try to place the card in any buffer
            for buffer in buffers.iter_mut() {
                if buffer.drag_card_in(&state.dragged_card) {
                    placed = true;
                    info!("Card placed in buffer");
                    break;
                }
            }

            for buffer in buffers.iter_mut() {
                buffer.organize_cards();
            }

            if placed {
                state.dragged_card.borrow_mut().stop_dragging();
                self.state = None;
            } else {
                buffers[state.original_buffer_pos]
                    .insert_card(state.original_card_location, state.dragged_card.clone());
                buffers[state.original_buffer_pos].organize_cards();
                self.state = None;
            }
        }

        // Clean up any remaining dragging state in buffers
        for buffer in buffers.iter_mut() {
            buffer.abort_dragging();
        }
    }

    pub fn snap(&self, buffers: &[&dyn DraggableCardBuffer]) {
        for buffer in buffers {
            buffer.snap();
        }
    }

    pub fn abort_all_dragging(&mut self, buffers: &mut [&mut dyn DraggableCardBuffer]) {
        if let Some(ref state) = self.state {
            state.dragged_card.borrow_mut().stop_dragging();
            self.state = None;
        }

        for buffer in buffers {
            buffer.abort_dragging();
        }
    }
}

impl Render for DragManager {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        if let Some(ref state) = self.state {
            state.dragged_card.borrow().render(render_ctx)?;
        }
        Ok(())
    }
}
