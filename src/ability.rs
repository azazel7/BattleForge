use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Ability {
    #[default]
    Strength = 0,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
