use crate::{monster::*, action::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterTemplate {
    pub name: String,
    pub entity_stats: MonsterStats,
    pub actions: Vec<ActionTemplate>,
}


