use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{event::*, fight::Fight};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    name: String,
    entity_stats: MonsterStats,
    team_id: u8,
    actions: Vec<ActionEnum>,
}

impl Monster {
    pub fn from_template(template : &MonsterTemplate) -> Monster{
        Monster::default()
    }
    pub fn team(&self) -> u8 {
        self.team_id
    }
    pub fn is_dead(&self) -> bool {
        self.entity_stats.hp() <= 0
    }
    pub fn is_alive(&self) -> bool {
        self.entity_stats.hp() > 0
    }
    pub fn ac(&self) -> i8 {
        self.entity_stats.armor_class()
    }
    pub fn hp(&self) -> i8 {
        self.entity_stats.hp()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn decrease_hp(&mut self, dammage : i8) {
        self.entity_stats.decrease_hp(dammage);
    }
    pub fn take_action(&self, fight: &Fight) -> Option<Event> {
        let action = self.actions.first().unwrap_or(&ActionEnum::Nothing).clone();
        let targets = if let ActionEnum::Attack {
            attack_modifier: _,
            dammage: _,
            target_count,
        } = action
        {
            fight.get_entities()
                .iter()
                .enumerate()
                .filter_map(|(i, monster)| {
                    if monster.team_id != self.team_id && monster.is_alive() {
                        Some(i as i8)
                    } else {
                        None
                    }
                })
                .take(target_count as usize)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        Some(Event::new(action, targets))
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
    hp: i8,
}
impl MonsterStats {
    pub fn hp(&self) -> i8 {
        self.hp
    }
    pub fn decrease_hp(&mut self, amount : i8){
        self.hp -= amount;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    pub fn armor_class(&self) -> i8 {
        self.armor_class
    }
    
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterTemplate {
    name: String,
    entity_stats: MonsterStats,
    actions: Vec<ActionEnum>,
}

#[derive(Default, Clone, Debug)]
pub struct MonsterBuilder {
    database: HashMap<String, MonsterTemplate>,
}

impl MonsterBuilder {
    pub fn new(monsters: Vec<MonsterTemplate>) -> Self {
        Self {
            database: monsters
                .into_iter()
                .map(|template| (template.name.clone(), template))
                .collect(),
        }
    }
    pub fn build(&self, name : &str) -> Monster {
        assert!(self.database.contains_key(name));
        let template = self.database.get(name).unwrap();
        Monster::from_template(template)
    }
}
