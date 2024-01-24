use crate::{event::*, fight::Fight, action::*};
use core::cell::RefCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    name: String,
    entity_stats: MonsterStats,
    team_id: u8,
    actions: Vec<ActionEnum>,
    action_sequence: RefCell<Option<usize>>,
    action_used: RefCell<bool>,
}
impl Monster {
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
    pub fn ac(&self) -> i8 {
        self.entity_stats.armor_class()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn hp(&self) -> i8 {
        self.entity_stats.hp()
    }
    pub fn decrease_hp(&mut self, amount: i8) {
        self.entity_stats.decrease_hp(amount);
    }
    pub fn increase_hp(&mut self, amount: i8) {
        self.entity_stats.increase_hp(amount);
    }
    pub fn take_action(&self, fight: &Fight) -> Option<Event> {
        let mut action_sequence = self.action_sequence.borrow_mut();
        let mut action_used = self.action_used.borrow_mut();

        if *action_used && action_sequence.is_none() {
            return None;
        }
        let action = self.actions.first().unwrap_or(&ActionEnum::Nothing).clone();
        match action {
            ActionEnum::Nothing => {
                return None;
            }
            ActionEnum::Attack {
                attack_modifier: _,
                dammage: _,
                target_count,
            } => {
                let targets = fight
                    .get_entities()
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
                    .collect::<Vec<_>>();
                *action_used = true;
                return Some(Event::new(action, targets));
            }
            ActionEnum::MultiAttack { attacks } => {
                if let Some(idx) = *action_sequence {
                    *action_sequence = Some(idx+1);
                } else {
                    *action_sequence = Some(1);
                }
                *action_used = true;
                return None;
            }
        }
    }
    pub fn new_turn(&mut self) {
        *self.action_sequence.borrow_mut() = None;
        *self.action_used.borrow_mut() = false;
    }
}
impl From<&MonsterTemplate> for Monster {
    fn from(template: &MonsterTemplate) -> Self {
        Self {
            name: template.name.clone(),
            entity_stats: template.entity_stats.clone(),
            team_id: 0,
            actions: template
                .actions
                .iter()
                .map(|action_template| ActionEnum::from(action_template))
                .collect(),
            action_sequence: RefCell::new(None),
            action_used: RefCell::new(false),
        }
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
    max_hp: i8,
}
impl MonsterStats {
    pub fn hp(&self) -> i8 {
        self.hp
    }
    pub fn decrease_hp(&mut self, amount: i8) {
        self.hp -= amount;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
    pub fn increase_hp(&mut self, amount: i8) {
        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
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
    actions: Vec<ActionTemplate>,
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
    pub fn build(&self, name: &str) -> Monster {
        assert!(self.database.contains_key(name));
        let template = self.database.get(name).unwrap();
        Monster::from(template)
    }
}
