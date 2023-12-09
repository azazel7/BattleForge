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
pub enum AA {
    #[default]
    Nothing,
    Attack {
        attack_modifier: i8,
        dammage: i8,
        target_count: i8,
    },
}
pub struct Event {
    action: AA,
    targets: Vec<i8>,
}
impl Event {
    pub fn new(action: AA, targets: Vec<i8>) -> Self {
        Self { action, targets }
    }
    pub fn run(&self, target: &mut Monster) {
        match self.action {
            AA::Nothing => {}
            AA::Attack {
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
                    target.decrease_hp(dammage);
                    eprintln!("Dammage {dammage} -> hp target : {}", target.hp());
                }
            }
        }
    }
    pub fn is_target(&self, idx: i8) -> bool {
        self.targets.contains(&idx)
    }
}
