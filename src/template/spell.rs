use crate::action::ActionComponent;
use crate::action::ActionStruct;
use crate::resource::Charge;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpellTemplate {
    charges: Charge,
    resources: Vec<Resource>,
    level: i32,
    components: Vec<(ActionComponent, ActionComponent)>, //Spell/Attack/MultiAttack/Object/
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
            let mut new_comp = *comp + *upcast * self.upcast_level;
            new_comp.set_save_dc(self.spell_dc);
            new_comp.set_hit_roll(self.spell_attack);
            action.add_component(new_comp);
        }
        for _ in 0..self.upcast_level {
            for comp in &self.upcast_components {
                let mut new_comp = *comp;
                new_comp.set_save_dc(self.spell_dc);
                new_comp.set_hit_roll(self.spell_attack);
                action.add_component(new_comp);
            }
        }
        action
    }
}
