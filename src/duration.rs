use crate::event::{EventType, HandleEvent};

pub struct Duration {
    event_type : EventType,
    count : i32,
}

impl HandleEvent for Duration {
    fn handle(&mut self, event_type : EventType) {
        if event_type == self.event_type {
            self.count -= 1;
        }
    }
}
