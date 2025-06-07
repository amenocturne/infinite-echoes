use std::cell::{Cell, RefCell, RefMut};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

use crate::core::GameTime;

use macroquad::time::get_time;
use miniquad::info;

use super::errors::GameResult;
use super::game_state::GameEvent;

pub struct Scheduler {
    queue: RefCell<BinaryHeap<ScheduledEvent>>,
    clear_on_next: Cell<bool>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: RefCell::new(BinaryHeap::new()),
            clear_on_next: Cell::new(false),
        }
    }

    pub fn schedule(&self, event: GameEvent, delay: Option<Duration>) {
        self.schedule_with_queue_ref(event, delay, &mut self.queue.borrow_mut());
    }

    fn schedule_with_queue_ref(
        &self,
        event: GameEvent,
        delay: Option<Duration>,
        queue_ref: &mut RefMut<BinaryHeap<ScheduledEvent>>,
    ) {
        let trigger_time = delay.map(|d| get_time() + d.as_secs_f64());
        queue_ref.push(ScheduledEvent {
            trigger_time,
            event,
        })
    }

    pub fn clear(&self) {
        self.clear_on_next.set(true);
    }

    pub fn process_events(
        &self,
        handler: &mut dyn Fn(GameEvent) -> GameResult<Vec<(GameEvent, Option<Duration>)>>,
    ) {
        let mut queue = self.queue.borrow_mut();
        if self.clear_on_next.get() {
            queue.clear();
            self.clear_on_next.set(false);
        }

        while let Some(event) = queue.peek() {
            let current_time = get_time();
            let trigger_time = event.trigger_time;

            match trigger_time {
                Some(t) => {
                    if t <= current_time {
                        let popped_event = queue.pop().unwrap().event;
                        match handler(popped_event) {
                            Ok(events) => {
                                for (event, delay) in events {
                                    self.schedule_with_queue_ref(event, delay, &mut queue);
                                }
                            }
                            Err(e) => {
                                info!("Error occured: {}", e.show());
                            }
                        }
                    } else {
                        break;
                    }
                }
                None => {
                    let popped_event = queue.pop().unwrap().event;
                    match handler(popped_event) {
                        Ok(events) => {
                            for (event, delay) in events {
                                self.schedule_with_queue_ref(event, delay, &mut queue);
                            }
                        }
                        Err(e) => {
                            info!("Error occured: {}", e.show());
                        }
                    }
                }
            }
        }
    }
}

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
