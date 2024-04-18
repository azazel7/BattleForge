use std::collections::HashMap;

use crate::duration::Duration;
use crate::event::{EventType, HandleEvent};
use crate::modifier::{ModifierType, Modifier};

pub struct Effect {
    name: String,
    target_ids: Vec<i32>,
    source_id: i32,
    id: i32,
    modifiers : HashMap<ModifierType, Modifier>,
    durations: Vec<Duration>,
}

impl Effect {
    pub fn get_modifier(&self, mod_type : ModifierType) -> Option<&Modifier> {
        self.modifiers.get(&mod_type)
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn targets(&self, id : i32) -> bool {
        todo!("Effect::targets");
    }
    pub fn affects(&self, mod_type : ModifierType) -> bool {
        todo!("Effect::affects");
    }
}
impl HandleEvent for Effect {
    fn handle(&mut self, event_type: EventType) {
        self.durations.iter_mut().for_each(|d| d.handle(event_type));
    }
}
