use crate::template::MonsterStatsTemplate;
use crate::{action::*, fight::Fight, resource::*, template::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::once;

#[derive(Default, Clone, Debug)]
pub struct Monster {
    name: String,
    entity_stats: MonsterStats,
    team_id: u8,
    actions: Vec<ActionStruct>,
    resources: HashMap<Resource, i32>,
}
impl Monster {
    pub fn from_template(builder: &TemplateBuilder, template: &MonsterTemplate) -> Self {
        //Create resources of the monster from normal resources + additional (spell slots, Ki, ...)
        //NOTE one monster may have multiple actions if the database says so
        let resources = template
            .resources
            .iter()
            .copied()
            .chain(once(Resource::Action))
            .chain(once(Resource::BonusAction))
            .chain(once(Resource::SpellAction))
            .fold(HashMap::new(), |mut hash, res| {
                hash.entry(res).and_modify(|e| *e += 1).or_insert(1);
                hash
            });
        //Build the monster
        let mut monster = Self {
            name: template.name.clone(),
            entity_stats: MonsterStats::from_template(builder, &template.entity_stats),
            team_id: 0,
            actions: vec![],
            resources,
        };
        //Create the action using monster to parametrize them
        let actions = template
            .actions
            .iter()
            .flat_map(|action_template| {
                ActionStruct::from_template(builder, &monster, action_template)
            })
            .collect();
        monster.actions = actions;
        monster
    }
    pub fn team(&self) -> u8 {
        self.team_id
    }
    pub fn set_team(&mut self, team: u8) {
        self.team_id = team;
    }
    pub fn is_dead(&self) -> bool {
        self.entity_stats.hp() <= 0
    }
    pub fn is_alive(&self) -> bool {
        self.entity_stats.hp() > 0
    }
    pub fn ac(&self) -> i32 {
        self.entity_stats.armor_class()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn hp(&self) -> i32 {
        self.entity_stats.hp()
    }
    pub fn set_hp(&mut self, hp: i32) {
        self.entity_stats.set_hp(hp);
        self.entity_stats.set_max_hp(hp);
    }
    pub fn decrease_hp(&mut self, amount: i32) {
        self.entity_stats.decrease_hp(amount);
    }
    pub fn increase_hp(&mut self, amount: i32) {
        self.entity_stats.increase_hp(amount);
    }
    pub fn get_targets(&self, fight: &Fight, action: &dyn Action) -> Vec<usize> {
        let target_count = action.target_count() as usize;
        fight
            .get_entities()
            .iter()
            .enumerate()
            .filter_map(|(i, monster)| {
                let monster = monster.borrow();
                if monster.team_id != self.team_id && monster.is_alive() {
                    Some(i)
                } else {
                    None
                }
            })
            .take(target_count as usize)
            .collect::<Vec<_>>()
    }
    pub fn take_action(&mut self, fight: &Fight) -> Option<ActionStruct> {
        let resources = &mut self.resources;

        let available_action = self.actions.iter_mut().find(|action| {
            println!("{} -> {resources:?} - {action:?}", self.name);
            action.is_available(&resources)
        });
        if let Some(action) = available_action {
            action.consume_resources(resources);
            action.use_charge();

            return Some(action.clone());
        } else {
            None
        }
    }
    pub fn new_turn(&mut self) {
        let resources = &mut self.resources;
        resources.entry(Resource::Action).and_modify(|e| *e = 1);
        resources
            .entry(Resource::BonusAction)
            .and_modify(|e| *e = 1);
        resources
            .entry(Resource::SneakAttack)
            .and_modify(|e| *e = 1);
        resources
            .entry(Resource::SpellAction)
            .and_modify(|e| *e = 1);
    }
    pub fn highest_spell_slot(&self) -> i32 {
        self.resources
            .keys()
            .filter_map(|res| match res {
                Resource::Spell(lvl) => Some(*lvl),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Stats {
    strength: i8,
    dexterity: i8,
    constitution: i8,
    wisdom: i8,
    intelligence: i8,
    charisma: i8,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterStats {
    ability: Stats,
    saving_throw: Stats,
    initiative: i8,
    armor_class: i8,
    hp: i32,
    max_hp: i32,
}

impl MonsterStats {
    fn from_template(builder: &TemplateBuilder, template: &MonsterStatsTemplate) -> Self {
        let hp = template.hp.roll();
        Self {
            ability: template.ability,
            saving_throw: template.saving_throw,
            initiative: template.initiative,
            armor_class: template.armor_class,
            hp,
            max_hp: hp,
        }
    }
    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn set_max_hp(&mut self, amount: i32) {
        self.max_hp = amount;
        self.hp = self.hp.min(self.max_hp);
    }
    pub fn set_hp(&mut self, amount: i32) {
        self.hp = amount;
        self.max_hp = self.max_hp.max(self.hp);
    }
    pub fn decrease_hp(&mut self, amount: i32) {
        self.hp -= amount;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    pub fn increase_hp(&mut self, amount: i32) {
        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
    pub fn armor_class(&self) -> i32 {
        self.armor_class as i32
    }
}
