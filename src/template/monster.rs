use crate::template::ActionTemplate;
use crate::template::MonsterStatsTemplate;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MonsterTemplate {
    pub name: String,
    pub entity_stats: MonsterStatsTemplate,
    pub actions: Vec<ActionTemplate>,
}


