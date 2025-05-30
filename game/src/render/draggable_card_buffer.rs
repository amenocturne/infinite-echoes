use std::cell::RefCell;

use macroquad::math::Vec2;

use super::{
    hover::Hover,
    rectangle_boundary::{is_inside_rectangle, RectangleBoundary},
    widgets::card_widget::Card,
};

pub trait DraggableCardBuffer {
    fn cards(&self) -> &Vec<RefCell<Card>>;
    fn remove_card(&mut self, i: usize) -> RefCell<Card>;
    fn insert_card(&mut self, i: usize, card: RefCell<Card>);
    fn snapping_margin(&self) -> Vec2;
    fn card_centers(&self) -> Vec<Vec2>;
    fn drag_in_regions(&self) -> Vec<(Vec2, Vec2)>;
    // Should put cards into their default locations
    fn organize_cards(&mut self);

    // Starts card dragging if mouse_pos is over a card
    fn try_start_dragging(&self, mouse_pos: Vec2) -> Option<usize>{
        for (i, c) in self.cards().iter().enumerate() {
            if c.borrow().is_hovered_over(mouse_pos) {
                c.borrow_mut().start_dragging();
                return Some(i);
            }
        }
        None
    }

    fn pop_dragged_card(&mut self) -> Option<RefCell<Card>>{
        let mut removed = None;
        let mut removed_index = None;
        for (i, c) in self.cards().iter().enumerate() {
            if c.borrow().is_dragged() {
                c.borrow_mut().stop_dragging();
                removed_index = Some(i);
                removed = Some(c.clone());
                break;
            }
        }
        removed_index.map(|i| self.remove_card(i));
        removed
    }

    // Aborts dragging cards and puts all of them back
    fn abort_dragging(&mut self) {
        for c in self.cards() {
            if c.borrow().is_dragged() {
                c.borrow_mut().stop_dragging();
                self.organize_cards();
                break;
            }
        }
    }

    fn snap(&self) {
        let margin = self.snapping_margin();
        for (c, center) in self.cards().iter().zip(self.card_centers()) {
            c.borrow_mut().snap(center, margin);
        }
    }

    fn drag_card_in(&mut self, card: &RefCell<Card>) -> bool {
        for (i, (top_left, bottom_right)) in (self.drag_in_regions()).iter().enumerate() {
            if is_inside_rectangle(top_left, bottom_right, &card.borrow().center()) {
                self.insert_card(i, card.clone());
                self.organize_cards();
                return true;
            }
        }
        false
    }

    fn update_dragged_position(&self, mouse_position: Vec2) {
        for c in self.cards() {
            c.borrow_mut().update_dragged_position(mouse_position);
        }
    }
}
