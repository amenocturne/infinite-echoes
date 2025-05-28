use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

use crate::core::GameTime;

use macroquad::time::get_time;

use super::game_state::GameEvent;

pub struct Scheduler {
    queue: BinaryHeap<ScheduledEvent>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, event: GameEvent, delay: Option<Duration>) {
        let trigger_time = delay.map(|d| get_time() + d.as_secs_f64());
        self.queue.push(ScheduledEvent {
            trigger_time,
            event,
        })
    }

    pub fn process_events(&mut self, handler: &mut dyn Fn(GameEvent)) {
        while let Some(event) = self.queue.peek() {
            let current_time = get_time();
            match event.trigger_time {
                Some(t) => {
                    if t <= current_time {
                        handler(self.queue.pop().unwrap().event);
                    } else {
                        break;
                    }
                }
                None => handler(self.queue.pop().unwrap().event),
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
