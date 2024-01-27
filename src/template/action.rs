use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionTemplate {
    Attack {
        attack_modifier: i8,
        dammage: String,
        target_count: i8,
    },
    MultiAttack {
        attacks: Vec<ActionTemplate>,
    },
    // Spell {
    //     name: String,
    //     lvl: i32,
    // },
}
impl Default for ActionTemplate {
    fn default() -> Self {
        Self::Attack {
            attack_modifier: 0,
            dammage: "1d4".to_string(),
            target_count: 1,
        }
    }
}
