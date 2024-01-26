use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Resource {
    #[default]
    Action,
    BonusAction,
    SneakAttack,
    SpellAction,
    Ki,
    Spell1,
    Spell2,
    Spell3,
    Spell4,
}

#[derive(Default, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum Charge {
    #[default]
    Infinite,
    Limited(i32),
}
