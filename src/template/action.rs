use serde::{Deserialize, Serialize};

use crate::formula::Formula;
use crate::utils::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionTemplate {
    Attack {
        attack_modifier: i32,
        #[serde(deserialize_with = "string_or_struct")]
        dammage: Formula,
        target_count: i32,
    },
    MultiAttack {
        attacks: Vec<ActionTemplate>,
    },
    Spell {
        name: String,
        spell_attack : i32,
        spell_dc: i32,
    },
}
impl Default for ActionTemplate {
    fn default() -> Self {
        Self::Attack {
            attack_modifier: 0,
            dammage: Formula::from("1d4"),
            target_count: 1,
        }
    }
}
