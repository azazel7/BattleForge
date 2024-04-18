use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    #[default]
    Turn,
    Round,
    DealDamage,
    ReceiveDamage,
    NewEffect,
}

pub trait HandleEvent {
    fn handle(&mut self, event_type : EventType);
}
