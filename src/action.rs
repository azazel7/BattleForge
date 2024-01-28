use std::collections::HashMap;

use crate::formula::Formula;
use crate::monster::*;
use crate::resource::Charge;
use crate::resource::Resource;
use crate::utils::*;
use crate::template::ActionTemplate;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

pub trait Action {
    fn apply(&self, m: &mut Monster);
    fn target_count(&self) -> usize;
    fn consume_resources(&self, resources: &mut HashMap<Resource, i32>);
    fn is_available(&self, resources: &HashMap<Resource, i32>) -> bool;
    fn has_charges(&self) -> bool;
    fn use_charge(&mut self);
}
//https://serde.rs/enum-representations.html
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionComponent {
    Attack {
        attack_modifier: i8,
        #[serde(deserialize_with = "string_or_struct")]
        dammage: Formula,
        target_count: i8,
    },
}

impl Action for ActionComponent {
    // add code here
    fn apply(&self, target: &mut Monster) {
        match &self {
            ActionComponent::Attack {
                attack_modifier,
                dammage,
                target_count: _,
            } => {
                let mut rng = rand::thread_rng();
                let die = Uniform::from(1..=20);
                let throw = die.sample(&mut rng);
                let hit = throw + attack_modifier;
                eprintln!(
                    "Roll {throw}+{attack_modifier} = {hit} (AC {})",
                    target.ac()
                );
                if hit >= target.ac() {
                    let dmg = dammage.roll() as i8;
                    target.decrease_hp(dmg);
                    eprintln!("Dammage {dmg} -> hp target : {}", target.hp());
                }
            }
        }
    }
    fn target_count(&self) -> usize {
        match &self {
            ActionComponent::Attack {
                attack_modifier: _,
                dammage: _,
                target_count,
            } => *target_count as usize,
        }
    }
    fn consume_resources(&self, resources: &mut HashMap<Resource, i32>) {
        resources
            .entry(Resource::Action)
            .and_modify(|qty| *qty -= 1);
    }
    fn is_available(&self, resources: &HashMap<Resource, i32>) -> bool {
        let res = resources.get(&Resource::Action);
        if let Some(qty) = res {
            *qty > 0
        } else {
            false
        }
    }
    fn has_charges(&self) -> bool {
        true
    }
    fn use_charge(&mut self) {
        unimplemented!("ActionComponent::use_charge");
    }
}
impl Default for ActionComponent {
    fn default() -> Self {
        Self::Attack {
            attack_modifier: 0,
            dammage: Formula::from("1d4"),
            target_count: 1,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionStruct {
    charges: Charge,
    resources: Vec<Resource>,
    components: Vec<ActionComponent>, //Spell/Attack/MultiAttack/Object/
}
impl ActionStruct {
    pub fn get_components(&self) -> &Vec<ActionComponent> {
        &self.components
    }
}
impl Action for ActionStruct {
    // add code here
    fn apply(&self, target: &mut Monster) {
        unimplemented!("ActionStruct::apply");
    }
    fn target_count(&self) -> usize {
        unimplemented!("ActionStruct::apply");
    }
    fn consume_resources(&self, resources: &mut HashMap<Resource, i32>) {
        for resource in &self.resources {
            resources.entry(*resource).and_modify(|qty| *qty -= 1);
        }
    }
    fn is_available(&self, resources: &HashMap<Resource, i32>) -> bool {
        self.resources.iter().all(|resource| {
            if let Some(qty) = resources.get(resource) {
                *qty > 0
            } else {
                false
            }
        })
    }
    fn has_charges(&self) -> bool {
        match self.charges {
            Charge::Infinite => true,
            Charge::Limited(qty) => qty > 0,
        }
    }
    fn use_charge(&mut self) {
        match &mut self.charges {
            Charge::Infinite => {}
            Charge::Limited(qty) => {
                *qty -= 1;
            }
        }
    }
}
impl From<&ActionTemplate> for ActionStruct {
    fn from(template: &ActionTemplate) -> Self {
        match template {
            ActionTemplate::Attack {
                attack_modifier,
                dammage,
                target_count,
            } => {
                let component = ActionComponent::Attack {
                    attack_modifier: *attack_modifier,
                    dammage: *dammage,
                    target_count: *target_count,
                };
                ActionStruct {
                    charges: Charge::Infinite,
                    resources: vec![Resource::Action],
                    components: vec![component],
                }
            }
            ActionTemplate::MultiAttack { attacks } => {
                let components = attacks
                    .iter()
                    .map(|t| match t {
                        ActionTemplate::Attack {
                            attack_modifier,
                            dammage,
                            target_count,
                        } => ActionComponent::Attack {
                            attack_modifier: *attack_modifier,
                            dammage: *dammage,
                            target_count: *target_count,
                        },
                        ActionTemplate::MultiAttack { .. } => {
                            unreachable!("MultiAttack cannot be nested in the database.");
                        }
                    })
                    .collect();
                ActionStruct {
                    charges: Charge::Infinite,
                    resources: vec![Resource::Action],
                    components,
                }
            }
        }
    }
}
impl Default for ActionStruct {
    fn default() -> Self {
        Self {
            charges: Charge::Infinite,
            resources: vec![Resource::Action],
            components: vec![],
        }
    }
}
