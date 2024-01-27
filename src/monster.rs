use crate::{action::*, fight::Fight, monster_template::*, resource::*, formula::Formula};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Monster {
    name: String,
    entity_stats: MonsterStats,
    team_id: u8,
    actions: Vec<ActionStruct>,
    resources: HashMap<Resource, i32>,
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
    pub fn set_hp(&mut self, hp: i8) {
        self.entity_stats.set_hp(hp);
        self.entity_stats.set_max_hp(hp);
    }
    pub fn decrease_hp(&mut self, amount: i8) {
        self.entity_stats.decrease_hp(amount);
    }
    pub fn increase_hp(&mut self, amount: i8) {
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

        let available_action = self
            .actions
            .iter_mut()
            .find(|action| action.is_available(&resources));
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
}
impl From<&MonsterTemplate> for Monster {
    fn from(template: &MonsterTemplate) -> Self {
        Self {
            name: template.name.clone(),
            entity_stats: MonsterStats::from(&template.entity_stats),
            team_id: 0,
            actions: template
                .actions
                .iter()
                .map(|action_template| ActionStruct::from(action_template))
                .collect(),
            resources: HashMap::from([(Resource::Action, 1), (Resource::BonusAction, 1)]),
        }
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
    hp: i8,
    max_hp: i8,
}

impl MonsterStats {
    pub fn hp(&self) -> i8 {
        self.hp
    }
    pub fn set_max_hp(&mut self, amount: i8) {
        self.max_hp = amount;
        self.hp = self.hp.min(self.max_hp);
    }
    pub fn set_hp(&mut self, amount: i8) {
        self.hp = amount;
        self.max_hp = self.max_hp.max(self.hp);
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
impl From<&MonsterStatsTemplate> for MonsterStats {
    fn from(template: &MonsterStatsTemplate) -> Self {
        let formula = Formula::from(&template.hp);
        let hp = formula.roll();
        Self {
            ability: template.ability,
            saving_throw: template.saving_throw,
            initiative: template.initiative,
            armor_class: template.armor_class,
            hp: hp as i8,
            max_hp: hp as i8,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct MonsterBuilder {
    database: HashMap<String, MonsterTemplate>,
    current_monster: Option<Monster>,
}

impl MonsterBuilder {
    pub fn new(monsters: Vec<MonsterTemplate>) -> Self {
        Self {
            database: monsters
                .into_iter()
                .map(|template| (template.name.clone(), template))
                .collect(),
            current_monster: None,
        }
    }
    pub fn create(&mut self, name: &str) -> &mut MonsterBuilder {
        assert!(self.database.contains_key(name));
        let template = self.database.get(name).unwrap();
        self.current_monster = Some(Monster::from(template));
        self
    }
    pub fn team(&mut self, team: i32) -> &mut MonsterBuilder {
        assert!(self.current_monster.is_some());
        if let Some(monster) = &mut self.current_monster {
            monster.set_team(team as u8);
        }
        self
    }
    pub fn hp(&mut self, hp: i32) -> &mut MonsterBuilder {
        assert!(self.current_monster.is_some());
        if let Some(monster) = &mut self.current_monster {
            monster.set_hp(hp as i8);
        }
        self
    }
    pub fn build(&mut self) -> Monster {
        if let Some(monster) = self.current_monster.take().to_owned() {
            monster
        } else {
            panic!("Error building a monster. No monster is being created");
        }
    }
}
