use core::panic;
use std::collections::HashMap;

use crate::ability::Ability;
use crate::formula::Formula;
use crate::monster::*;
use crate::resource::Charge;
use crate::resource::Resource;
use crate::template::ActionTemplate;
use crate::template::TemplateBuilder;
use crate::utils::*;
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ActionCondition {
    #[default]
    True,
    False,
    HitCondition {
        #[serde(default)]
        attack_modifier: i32,
    },
    SaveCondition {
        #[serde(default)]
        save_dc: i32,
        ability: Ability,
    },
}
impl ActionCondition {
    pub fn set_hit_roll(&mut self, attack_modifier: i32) {
        match self {
            Self::HitCondition {
                attack_modifier: am,
            } => {
                *am = attack_modifier;
            }
            _ => {}
        }
    }
    pub fn set_save_dc(&mut self, save_dc: i32) {
        match self {
            Self::SaveCondition { save_dc: sd, .. } => *sd = save_dc,
            _ => {}
        }
    }
    fn pass(&self, target: &mut Monster) -> bool {
        match self {
            Self::True => true,
            Self::False => false,
            Self::SaveCondition { save_dc, ability } => {
                let mut rng = rand::thread_rng();
                let die = Uniform::from(1..=20);
                let throw = die.sample(&mut rng);
                let save_mod = target.save_mod(*ability);
                let hit = throw + save_mod;
                eprintln!("Save {throw}+{save_mod} = {hit} (DC {save_dc})");
                hit >= *save_dc
            }
            Self::HitCondition { attack_modifier } => {
                //TODO roll a 1d20
                let mut rng = rand::thread_rng();
                let die = Uniform::from(1..=20);
                let throw = die.sample(&mut rng);
                let hit = throw + attack_modifier;
                eprintln!(
                    "Roll {throw}+{attack_modifier} = {hit} (AC {})",
                    target.ac()
                );
                hit >= target.ac()
            }
        }
    }
}

//https://serde.rs/enum-representations.html
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum ActionComponent {
    #[default]
    Nothing,
    Condition {
        target_count: i32,
        condition: ActionCondition,
        success: Box<ActionComponent>,
        failure: Box<ActionComponent>,
    },
    MultiComponent {
        next: Vec<ActionComponent>,
    },
    Damage {
        #[serde(deserialize_with = "string_or_struct")]
        damage: Formula,
    },
    HalfDamage {
        #[serde(deserialize_with = "string_or_struct")]
        damage: Formula,
    },
}

impl ActionComponent {
    pub fn set_hit_roll(&mut self, attack_modifier: i32) {
        match self {
            Self::Condition {
                condition,
                success,
                failure,
                ..
            } => {
                condition.set_hit_roll(attack_modifier);
                success.set_hit_roll(attack_modifier);
                failure.set_hit_roll(attack_modifier);
            }
            Self::MultiComponent { next } => {
                for comp in next {
                    comp.set_hit_roll(attack_modifier);
                }
            }
            _ => {}
        }
    }
    pub fn set_save_dc(&mut self, save_dc: i32) {
        match self {
            Self::Condition {
                condition,
                success,
                failure,
                ..
            } => {
                condition.set_save_dc(save_dc);
                success.set_save_dc(save_dc);
                failure.set_save_dc(save_dc);
            }
            Self::MultiComponent { next } => {
                for comp in next {
                    comp.set_save_dc(save_dc);
                }
            }
            _ => {}
        }
    }
    pub fn set_target_count(&mut self, target_count: i32) {
        match self {
            Self::Condition {
                target_count: tc,
                success,
                failure,
                ..
            } => {
                *tc = target_count;
                success.set_target_count(target_count);
                failure.set_target_count(target_count);
            }
            Self::MultiComponent { next } => {
                for comp in next {
                    comp.set_target_count(target_count);
                }
            }
            _ => {}
        }
    }
    pub fn increase_damage(&mut self, damage: &Formula) {
        match self {
            ActionComponent::Nothing => {}
            ActionComponent::Damage { damage: dmg, .. }
            | ActionComponent::HalfDamage { damage: dmg, .. } => *dmg += *damage,
            ActionComponent::Condition {
                success, failure, ..
            } => {
                success.increase_damage(damage);
                failure.increase_damage(damage);
            }
            ActionComponent::MultiComponent { next } => {
                for comp in next {
                    comp.increase_damage(damage);
                }
            }
        }
    }
    pub fn increase_target_count(&mut self, amount: i32) {
        match self {
            ActionComponent::Nothing
            | ActionComponent::Damage { .. }
            | ActionComponent::HalfDamage { .. } => {}
            ActionComponent::Condition {
                success,
                failure,
                target_count,
                ..
            } => {
                *target_count += amount;
                success.increase_target_count(amount);
                failure.increase_target_count(amount);
            }
            ActionComponent::MultiComponent { next } => {
                for comp in next {
                    comp.increase_target_count(amount);
                }
            }
        }
    }
    pub fn average_dammage(&self) -> f32 {
        match self {
            ActionComponent::Nothing => 0.0,
            ActionComponent::Damage { damage, .. } => damage.average_roll(),
            ActionComponent::HalfDamage { damage, .. } => damage.average_roll() / 2.0,
            ActionComponent::Condition { success, .. } => success.average_dammage(), //TODO take failure in account too
            ActionComponent::MultiComponent { next } => {
                next.iter().map(|comp| comp.average_dammage()).sum()
            }
        }
    }
    pub fn apply(&self, target: &mut Monster) {
        match &self {
            ActionComponent::Damage { damage } => {
                let dmg = damage.roll();
                target.decrease_hp(dmg);
                eprintln!("Dammage {dmg} -> hp target : {}", target.hp());
            }
            ActionComponent::HalfDamage { damage } => {
                let dmg = (damage.roll() as f32 / 2.0).floor() as i32;
                target.decrease_hp(dmg);
                eprintln!("Dammage {dmg} -> hp target : {}", target.hp());
            }
            ActionComponent::Condition {
                condition,
                success,
                failure,
                ..
            } => {
                if condition.pass(target) {
                    success.apply(target);
                } else {
                    failure.apply(target);
                }
            }
            ActionComponent::MultiComponent { next } => {
                for comp in next {
                    comp.apply(target);
                }
            }
            ActionComponent::Nothing => {}
        }
    }
    pub fn target_count(&self) -> usize {
        match &self {
            ActionComponent::Condition { target_count, .. } => *target_count as usize,
            ActionComponent::Damage { .. } | ActionComponent::HalfDamage { .. } => 1, //TODO does that makes sense?
            ActionComponent::MultiComponent { .. } => 1, //TODO does that makes sense?
            ActionComponent::Nothing => 0,
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
    ) -> HashMap<String, Self> {
        let mut ret = HashMap::new();
        match template {
            ActionTemplate::Attack {
                attack_modifier,
                dammage,
                target_count,
                name,
            } => {
                let dmg = ActionComponent::Damage { damage: *dammage };
                let component = ActionComponent::Condition {
                    condition: ActionCondition::HitCondition {
                        attack_modifier: *attack_modifier,
                    },
                    success: Box::new(dmg),
                    failure: Box::new(ActionComponent::Nothing),
                    target_count: *target_count,
                };
                let action = ActionStruct {
                    charges: Charge::Infinite,
                    resources: vec![Resource::Action],
                    components: vec![component],
                };
                ret.insert(name.clone(), action);
            }
            ActionTemplate::MultiAttack { attacks, name } => {
                let components = attacks
                    .iter()
                    .map(|t| match t {
                        ActionTemplate::Attack {
                            attack_modifier,
                            dammage,
                            target_count,
                            ..
                        } => {
                            let dmg = ActionComponent::Damage { damage: *dammage };
                            let component = ActionComponent::Condition {
                                condition: ActionCondition::HitCondition {
                                    attack_modifier: *attack_modifier,
                                },
                                success: Box::new(dmg),
                                failure: Box::new(ActionComponent::Nothing),
                                target_count: *target_count,
                            };
                            component
                        }
                        ActionTemplate::MultiAttack { .. } => {
                            unreachable!("MultiAttack cannot be nested in the database.");
                        }
                        ActionTemplate::Spell { .. } => {
                            unreachable!("Spell cannot be in MultiAttack.");
                        }
                    })
                    .collect();
                let action = ActionStruct {
                    charges: Charge::Infinite,
                    resources: vec![Resource::Action],
                    components,
                };
                ret.insert(name.clone(), action);
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
                for upcast_lvl in 0..=(highest - lowest) {
                    let action = spell_template
                        .spell_attack(*spell_attack)
                        .spell_dc(*spell_dc)
                        .upcast(upcast_lvl)
                        .build();
                    let mut name = name.clone();
                    let lvl = upcast_lvl+lowest;
                    name.push_str(" ");
                    name.push_str(&lvl.to_string());
                    ret.insert(name, action);
                }
            }
        }
        ret
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
    pub fn average_dammage(&self) -> f32 {
        self.components
            .iter()
            .map(|component| component.average_dammage())
            .sum()
    }
    pub fn consume_resources(&self, resources: &mut HashMap<Resource, i32>) {
        for resource in &self.resources {
            resources.entry(*resource).and_modify(|qty| *qty -= 1);
        }
    }
    pub fn is_available(&self, resources: &HashMap<Resource, i32>) -> bool {
        self.resources.iter().all(|resource| {
            if let Some(qty) = resources.get(resource) {
                *qty > 0
            } else {
                false
            }
        })
    }
    pub fn has_charges(&self) -> bool {
        match self.charges {
            Charge::Infinite => true,
            Charge::Limited(qty) => qty > 0,
        }
    }
    pub fn use_charge(&mut self) {
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
