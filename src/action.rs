use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use crate::dice::*;
use crate::monster::*;


pub trait Action {
    fn apply(&self, m: &mut Monster);
    fn target_count(&self) -> i8;
}

//https://serde.rs/enum-representations.html
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum ActionEnum {
    #[default]
    Nothing,
    Attack {
        attack_modifier: i8,
        dammage: Dice,
        target_count: i8,
    },
    MultiAttack {
        attacks: Vec<ActionEnum>,
    },
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
                eprintln!("Roll {throw}+{attack_modifier} = {hit} (AC {})", target.ac());
                if hit >= target.ac() {
                    let dmg = dammage.roll() as i8;
                    target.decrease_hp(dmg);
                    eprintln!("Dammage {dmg} -> hp target : {}", target.hp());
                }
            }
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
            } => *target_count,
            ActionEnum::MultiAttack { attacks } => 1,
        }
    }
}
impl From<&ActionTemplate> for ActionEnum {
    fn from(template: &ActionTemplate) -> Self {
        match template {
            ActionTemplate::Nothing => ActionEnum::Nothing,
            ActionTemplate::Attack {
                attack_modifier,
                dammage,
                target_count,
            } => ActionEnum::Attack {
                attack_modifier: *attack_modifier,
                dammage: Dice::from(dammage),
                target_count: *target_count,
            },
            ActionTemplate::MultiAttack { attacks } => ActionEnum::MultiAttack {
                attacks: attacks.iter().map(|t| ActionEnum::from(t)).collect(),
            },
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum ActionTemplate {
    #[default]
    Nothing,
    Attack {
        attack_modifier: i8,
        dammage: String,
        target_count: i8,
    },
    MultiAttack {
        attacks: Vec<ActionTemplate>,
    },
}
