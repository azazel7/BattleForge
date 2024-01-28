use crate::{monster::*, formula::Formula};
use serde::{Deserialize, Serialize};
use crate::utils::*;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterStatsTemplate {
    pub ability: Stats,
    pub saving_throw: Stats,
    pub initiative: i8,
    pub armor_class: i8,
    #[serde(deserialize_with = "string_or_struct")]
    pub hp: Formula,
}

