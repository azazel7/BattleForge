use crate::{monster::*, action::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterTemplate {
    pub name: String,
    pub entity_stats: MonsterStatsTemplate,
    pub actions: Vec<ActionTemplate>,
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterStatsTemplate {
    pub ability: Stats,
    pub saving_throw: Stats,
    pub initiative: i8,
    pub armor_class: i8,
    pub hp: String,
}


