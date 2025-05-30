use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

use crate::core::GameTime;

use macroquad::time::get_time;
use miniquad::info;

use super::game_state::GameEvent;

pub struct Scheduler {
    queue: RefCell<BinaryHeap<ScheduledEvent>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(BinaryHeap::new()),
        }
    }

    pub fn schedule(&self, event: GameEvent, delay: Option<Duration>) {
        let trigger_time = delay.map(|d| get_time() + d.as_secs_f64());
        self.queue.borrow_mut().push(ScheduledEvent {
            trigger_time,
            event,
        })
    }

    pub fn process_events(&self, handler: &mut dyn Fn(GameEvent)) {
        let mut queue = self.queue.borrow_mut(); // Get one mutable borrow

        while let Some(event) = queue.peek() {
            let current_time = get_time();
            let trigger_time = event.trigger_time; // Copy trigger_time to avoid borrowing issues

            match trigger_time {
                Some(t) => {
                    if t <= current_time {
                        // Since we already have the mutable borrow, we can pop directly
                        let popped_event = queue.pop().unwrap().event;
                        handler(popped_event);
                    } else {
                        break; // Event is in the future, stop processing
                    }
                }
                None => {
                    // Since we already have the mutable borrow, we can pop directly
                    let popped_event = queue.pop().unwrap().event;
                    handler(popped_event);
                }
            }
        }
    }
}

// --------------- ScheduledEvent ----------------------

pub struct ScheduledEvent {
    trigger_time: Option<GameTime>,
    event: GameEvent,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.trigger_time, other.trigger_time) {
            (Some(t1), Some(t2)) => t1.partial_cmp(&t2),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.trigger_time == other.trigger_time
    }
}

impl Eq for ScheduledEvent {}
