use crate::action::ActionComponent;
use crate::action::ActionStruct;
use crate::resource::Charge;
use crate::resource::Resource;
use crate::formula::Formula;
use crate::utils::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpellTemplate {
    charges: Charge,
    resources: Vec<Resource>,
    level: i32,
    components: Vec<(ActionComponent, ActionComponentModifier)>, //Spell/Attack/MultiAttack/Object/
    upcast_components: Vec<ActionComponent>,
    // upcast_new: Vec<ActionComponent>,
    //upcast to lvl N = (components + upcast_add * N) + N * upcast_new
    //upcast may affect charges of other item? Fame Arrows
    #[serde(default)]
    upcast_level: i32,
    #[serde(default)]
    spell_dc: i32,
    #[serde(default)]
    spell_attack: i32,
}
#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ActionComponentModifier {
    #[serde(default)]
    target_count: i32,
    #[serde(deserialize_with = "string_or_struct", default)]
    damage: Formula,
}
impl Mul<i32> for ActionComponentModifier {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            target_count: self.target_count * other,
            damage: self.damage * other,
        }
    }
}

impl SpellTemplate {
    pub fn spell_attack(&mut self, amount: i32) -> &mut Self {
        self.spell_attack = amount;
        self
    }
    pub fn spell_dc(&mut self, amount: i32) -> &mut Self {
        self.spell_dc = amount;
        self
    }
    pub fn upcast(&mut self, lvl: i32) -> &mut Self {
        self.upcast_level = lvl;
        self
    }
    pub fn get_base_level(&self) -> i32 {
        self.level
    }
    pub fn build(&self) -> ActionStruct {
        let mut action = ActionStruct::default();
        action.set_charge(self.charges);
        for rsce in &self.resources {
            action.add_resource(*rsce);
        }
        action.add_resource(Resource::SpellAction);
        action.add_resource(Resource::Spell(self.level + self.upcast_level));
        for (comp, upcast) in &self.components {
            let upcast = *upcast * self.upcast_level;
            let mut new_comp = comp.clone();
            new_comp.increase_damage(&upcast.damage);
            new_comp.increase_target_count(upcast.target_count);
            new_comp.set_save_dc(self.spell_dc);
            new_comp.set_hit_roll(self.spell_attack);
            action.add_component(new_comp);
        }
        for _ in 0..self.upcast_level {
            for comp in &self.upcast_components {
                let mut new_comp = comp.clone();
                new_comp.set_save_dc(self.spell_dc);
                new_comp.set_hit_roll(self.spell_attack);
                action.add_component(new_comp);
            }
        }
        action
    }
}
