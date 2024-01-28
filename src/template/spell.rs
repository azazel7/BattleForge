use crate::action::ActionComponent;
use crate::resource::Charge;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpellTemplate {
    charges: Charge,
    resources: Vec<Resource>,
    level: i32,
    components: Vec<ActionComponent>, //Spell/Attack/MultiAttack/Object/
    //TODO structure doesn't take more action components in accounts
    upcast: Vec<ActionComponent>,
}
