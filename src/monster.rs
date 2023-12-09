use serde::{Deserialize, Serialize};
use crate::{event::*, fight::Fight};

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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Monster {
    name: String,
    entity_stats: MonsterStats,
    team_id: u8,
    actions: Vec<AA>,
}

impl Monster {
    pub fn team(&self) -> u8 {
        self.team_id
    }
    pub fn is_dead(&self) -> bool {
        self.entity_stats.hp <= 0
    }
    pub fn is_alive(&self) -> bool {
        self.entity_stats.hp > 0
    }
    pub fn ac(&self) -> i8 {
        self.entity_stats.armor_class
    }
    pub fn hp(&self) -> i8 {
        self.entity_stats.hp
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn decrease_hp(&mut self, dammage : i8) {
        self.entity_stats.hp -= dammage;
        if self.entity_stats.hp < 0 {
            self.entity_stats.hp = 0;
        }
    }
    pub fn take_action(&self, fight: &Fight) -> Event {
        let action = self.actions.first().unwrap_or(&AA::Nothing).clone();
        let targets = if let AA::Attack {
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
        Event::new(action, targets)
    }
}
