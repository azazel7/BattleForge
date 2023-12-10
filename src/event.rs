use crate::monster::*;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

pub trait Action {
    fn apply(&self, m: &mut Monster);
    fn target_count(&self) -> i8;
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmptyAction {}

impl Action for EmptyAction {
    fn apply(&self, m: &mut Monster) {}
    fn target_count(&self) -> i8 {
        0
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SimpleAttackAction {
        attack_modifier: i8,
        dammage: i8,
}

impl Action for SimpleAttackAction {
    fn apply(&self, target: &mut Monster) {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..=20);
        let throw = die.sample(&mut rng);
        let hit = throw + self.attack_modifier;
        eprintln!("Roll {throw}+{} = {hit}", self.attack_modifier);
        if throw >= target.ac() {
            target.decrease_hp(self.dammage);
            eprintln!("Dammage {} -> hp target : {}", self.dammage, target.hp());
        }
    }
    fn target_count(&self) -> i8 {
        1
    }
}

//https://serde.rs/enum-representations.html
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum ActionEnum {
    #[default]
    Nothing,
    Attack {
        attack_modifier: i8,
        dammage: i8,
        target_count: i8,
    },
    MultiAttack {
        attacks : Vec<ActionEnum>,
    }
}
impl Action for ActionEnum {
    // add code here
    fn apply(&self, target: &mut Monster) {
        match &self {
            ActionEnum::Nothing => {}
            ActionEnum::Attack {
                attack_modifier,
                dammage,
                target_count: _,
            } => {
                let mut rng = rand::thread_rng();
                let die = Uniform::from(1..=20);
                let throw = die.sample(&mut rng);
                let hit = throw + attack_modifier;
                eprintln!("Roll {throw}+{attack_modifier} = {hit}");
                if throw >= target.ac() {
                    target.decrease_hp(*dammage);
                    eprintln!("Dammage {dammage} -> hp target : {}", target.hp());
                }
            },
            ActionEnum::MultiAttack { attacks } => {
                let mut rng = rand::thread_rng();
                let die = Uniform::from(1..=20);
                
            }
        }
    }
    fn target_count(&self) -> i8 {

        match &self {
            ActionEnum::Nothing => 0,
            ActionEnum::Attack {
                attack_modifier,
                dammage,
                target_count,
            } => {
                *target_count
            },
            ActionEnum::MultiAttack { attacks } => {
                1
            }
        }
    }
}
pub struct Event {
    action: ActionEnum,
    targets: Vec<i8>,
}
impl Event {
    pub fn new(action: ActionEnum, targets: Vec<i8>) -> Self {
        Self { action, targets }
    }
    pub fn run(&self, target: &mut Monster) {
        self.action.apply(target);
    }
    pub fn is_target(&self, idx: i8) -> bool {
        self.targets.contains(&idx)
    }
}
impl Default for Event{
    fn default() -> Self {
        Self {
            action : ActionEnum::Nothing,
            targets : Vec::new()
        }
    }
}
