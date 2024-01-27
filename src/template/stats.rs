use crate::monster::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterStatsTemplate {
    pub ability: Stats,
    pub saving_throw: Stats,
    pub initiative: i8,
    pub armor_class: i8,
    pub hp: String,
}

