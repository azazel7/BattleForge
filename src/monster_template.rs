use crate::{monster::*, action::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterTemplate {
    name: String,
    entity_stats: MonsterStats,
    actions: Vec<ActionTemplate>,
}


