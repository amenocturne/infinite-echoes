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
    dragged_card: Option<RefCell<Card>>,
    original_buffer_pos: Option<usize>,
    original_card_location: Option<usize>,
}

impl DragManager {
    pub fn new() -> Self {
        Self {
            dragged_card: None,
            original_buffer_pos: None,
            original_card_location: None,
        }
    }

    pub fn handle_mouse_press(
        &mut self,
        mouse_pos: Vec2,
        buffers: &mut [&mut dyn DraggableCardBuffer],
    ) {
        if self.dragged_card.is_some() {
            return; // Already dragging something
        }

        // Try to start dragging from any buffer
        for (pos, buffer) in buffers.iter().enumerate() {
            if let Some(loc) = buffer.try_start_dragging(mouse_pos) {
                self.original_buffer_pos = Some(pos);
                self.original_card_location = Some(loc);
                break; // Found a card to drag, stop looking
            }
        }

        // Extract the dragged card if any buffer has one
        for buffer in buffers.iter_mut() {
            if let Some(card) = buffer.pop_dragged_card() {
                card.borrow_mut().start_dragging(); // Ensure card knows it's being dragged
                self.dragged_card = Some(card);
                info!("Card extracted and now being dragged by DragManager");
                break;
            }
        }
    }

    pub fn handle_mouse_drag(&mut self, mouse_pos: Vec2) {
        if let Some(ref card) = self.dragged_card {
            info!("Updating dragged card position to: {:?}", mouse_pos);
            card.borrow_mut().update_dragged_position(mouse_pos);
        }
    }

    pub fn handle_mouse_release(&mut self, buffers: &mut [&mut dyn DraggableCardBuffer]) {
        if let Some(ref card) = self.dragged_card {
            let mut placed = false;

            // Try to place the card in any buffer
            for buffer in buffers.iter_mut() {
                if buffer.drag_card_in(card) {
                    placed = true;
                    info!("Card placed in buffer");
                    break;
                }
            }

            for buffer in buffers.iter_mut() {
                buffer.organize_cards();
            }

            if placed {
                card.borrow_mut().stop_dragging();
                self.dragged_card = None;
                self.original_buffer_pos = None;
            } else {
                if let Some(pos) = self.original_buffer_pos {
                    if let Some(loc) = self.original_card_location {
                        buffers[pos].insert_card(loc, card.clone());
                        buffers[pos].organize_cards();
                        self.dragged_card = None;
                        self.original_buffer_pos = None;
                        self.original_card_location = None;
                    }
                }
                // Card couldn't be placed anywhere - keep it dragged or handle as needed
                info!("Card couldn't be placed in any buffer");
                // You could implement logic here to return it to original buffer
                // or keep it floating until next placement attempt
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
        if let Some(ref card) = self.dragged_card {
            card.borrow_mut().stop_dragging();
            self.dragged_card = None;
        }

        for buffer in buffers {
            buffer.abort_dragging();
        }
    }

    // Helper methods for debugging
    pub fn is_dragging(&self) -> bool {
        self.dragged_card.is_some()
    }

    pub fn get_dragged_card_position(&self) -> Option<Vec2> {
        self.dragged_card.as_ref().map(|card| card.borrow().center)
    }
}

impl Render for DragManager {
    fn render(&self, render_ctx: &RenderCtx) -> GameResult<()> {
        if let Some(ref card) = self.dragged_card {
            card.borrow().render(render_ctx)?;
        }
        Ok(())
    }
}
