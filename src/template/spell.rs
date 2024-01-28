use crate::action::ActionComponent;
use crate::resource::Charge;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpellTemplate {
    charges: Charge,
    resources: Vec<Resource>,
    level: i32,
    components: Vec<(ActionComponent, ActionComponent)>, //Spell/Attack/MultiAttack/Object/
    //TODO structure doesn't take more action components in accounts
    upcast: Vec<ActionComponent>,
    // upcast_new: Vec<ActionComponent>,
    //upcast to lvl N = (components + upcast_add * N) + N * upcast_new
    //upcast may affect charges of other item? Fame Arrows
    //
}
