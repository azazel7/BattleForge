use std::collections::HashMap;

use crate::formula::Formula;
use crate::monster::*;
use crate::resource::Charge;
use crate::resource::Resource;
use crate::template::ActionTemplate;
use crate::template::TemplateBuilder;
use crate::utils::*;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};

pub trait Action {
    fn apply(&self, m: &mut Monster);
    fn target_count(&self) -> usize;
    fn consume_resources(&self, resources: &mut HashMap<Resource, i32>);
    fn is_available(&self, resources: &HashMap<Resource, i32>) -> bool;
    fn has_charges(&self) -> bool;
    fn use_charge(&mut self);
}
//https://serde.rs/enum-representations.html
#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ActionComponent {
    #[default]
    Nothing,
    Attack {
        attack_modifier: i32,
        #[serde(deserialize_with = "string_or_struct")]
        dammage: Formula,
        target_count: i32,
    },
}

impl ActionComponent {
    pub fn set_hit_roll(&mut self, attack_modifier: i32) {
        match self {
            Self::Attack {
                attack_modifier: att,
                ..
            } => *att = attack_modifier,
            _ => {}
        }
    }
    pub fn set_save_dc(&mut self, save_dc: i32) {}
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
                    let dmg = dammage.roll();
                    target.decrease_hp(dmg);
                    eprintln!("Dammage {dmg} -> hp target : {}", target.hp());
                }
            }
            ActionComponent::Nothing => {}
        }
    }
    fn target_count(&self) -> usize {
        match &self {
            ActionComponent::Attack {
                attack_modifier: _,
                dammage: _,
                target_count,
            } => *target_count as usize,
            ActionComponent::Nothing => 0,
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
impl Add for ActionComponent {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        //Check which enum the component is
        match (self, other) {
            (
                Self::Attack {
                    attack_modifier,
                    dammage: self_dmg,
                    target_count: self_target,
                },
                Self::Attack {
                    attack_modifier: _,
                    dammage: other_dmg,
                    target_count: other_target,
                },
            ) => {
                //Sum up
                Self::Attack {
                    attack_modifier,
                    dammage: self_dmg + other_dmg,
                    target_count: self_target + other_target,
                }
            }
            (Self::Nothing, a) | (a, Self::Nothing) => a,
            _ => unreachable!("Summing two different components"),
        }
    }
}
impl Mul<i32> for ActionComponent {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        //Check which enum the component is
        match self {
            Self::Attack {
                attack_modifier,
                dammage: self_dmg,
                target_count: self_target,
            } => {
                //Sum up
                Self::Attack {
                    attack_modifier,
                    dammage: self_dmg * other,
                    target_count: self_target * other,
                }
            }
            Self::Nothing => Self::Nothing,
            _ => unreachable!("Multiplying a component not implemented. ({:?})", self),
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
    pub fn from_template(
        builder: &TemplateBuilder,
        monster: &Monster,
        template: &ActionTemplate,
    ) -> Vec<Self> {
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
                vec![ActionStruct {
                    charges: Charge::Infinite,
                    resources: vec![Resource::Action],
                    components: vec![component],
                }]
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
                        ActionTemplate::Spell { .. } => {
                            unreachable!("Spell cannot be in MultiAttack.");
                        }
                    })
                    .collect();
                vec![ActionStruct {
                    charges: Charge::Infinite,
                    resources: vec![Resource::Action],
                    components,
                }]
            }
            ActionTemplate::Spell {
                name,
                spell_attack,
                spell_dc,
            } => {
                //Build the spell from the template
                let mut spell_template = builder.get_spell_template(name);
                let highest = monster.highest_spell_slot();
                let lowest = spell_template.get_base_level();
                //Expend action based on how much the spell can be upcasted
                (0..=(highest - lowest))
                    .into_iter()
                    .map(|upcast_lvl| {
                        spell_template
                            .spell_attack(*spell_attack)
                            .spell_dc(*spell_dc)
                            .upcast(upcast_lvl)
                            .build()
                    })
                    .collect()
            }
        }
    }
    pub fn set_charge(&mut self, charge: Charge) {
        self.charges = charge;
    }
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
    pub fn add_component(&mut self, component: ActionComponent) {
        self.components.push(component);
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
impl Default for ActionStruct {
    fn default() -> Self {
        Self {
            charges: Charge::Infinite,
            resources: vec![],
            components: vec![],
        }
    }
}
