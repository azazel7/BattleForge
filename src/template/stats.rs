use crate::formula::Formula;
use crate::utils::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterStatsTemplate {
    pub abilities: StatsTemplate,
    pub saving_throws: StatsTemplate,
    pub initiative: i8,
    pub armor_class: i8,
    #[serde(deserialize_with = "string_or_struct")]
    pub hp: Formula,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StatsTemplate {
    pub strength: i8,
    pub dexterity: i8,
    pub constitution: i8,
    pub wisdom: i8,
    pub intelligence: i8,
    pub charisma: i8,
}
