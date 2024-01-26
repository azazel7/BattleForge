use crate::monster::*;
use crate::action::*;


pub struct Event {
    action: ActionStruct,
    targets: Vec<i8>,
}
impl Event {
    pub fn new(action: ActionStruct, targets: Vec<i8>) -> Self {
        Self { action, targets }
    }
    pub fn run(&self, target: &mut Monster) {
        self.action.apply(target);
    }
    pub fn is_target(&self, idx: i8) -> bool {
        self.targets.contains(&idx)
    }
}
impl Default for Event {
    fn default() -> Self {
        Self {
            action: ActionStruct::default(),
            targets: Vec::new(),
        }
    }
}
