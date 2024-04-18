use crate::{
    action::*,
    effect::Effect,
    modifier::{Modifier, ModifierType},
    monster::Monster,
};
use core::cell::RefCell;
use std::collections::HashSet;

pub struct Fight {
    next_id: i32,
    entities: Vec<RefCell<Monster>>,
    effects: Vec<Effect>,
}
impl Fight {
    pub fn new(mut entities: Vec<Monster>) -> Self {
        Self {
            next_id: entities.len() as i32,
            entities: entities
                .iter_mut()
                .enumerate()
                .map(|(i, e)| {
                    e.set_id(i as i32);
                    RefCell::new(e.to_owned())
                })
                .collect(),
            effects: Vec::new(),
        }
    }
    pub fn advance_round(&mut self) {
        eprintln!("==== New Round ====");
        //The loop for each entity's turn
        for idx in 0..self.entities.len() {
            let current_id;
            {
                //NOTE this thing *must* be *mut*
                let e = self.entities.get(idx).unwrap();
                current_id = e.borrow().id();
                e.borrow_mut().new_turn();
            }
            loop {
                let mut action = None;
                {
                    let mut e = self.entities.get(idx).unwrap().borrow_mut();
                    let is_alive = e.is_alive();
                    if is_alive {
                        action = e.take_action(self);
                        if action.is_some() {
                            eprintln!("Playing {} {idx} (hp: {})", e.name(), e.hp());
                        }
                    }
                }

                if let Some(mut action) = action {
                    action.ready_for_apply();
                    for act in action.get_components() {
                        let e = self.entities.get(idx).unwrap().borrow();
                        let targets = e.get_targets(self, act);
                        drop(e);
                        //TODO what about the action that affect the fight (turn into a wolf, add/another monster or effect)
                        for id in targets {
                            act.apply(current_id, id as i32, self);
                        }

                        // self.entities
                        //     .iter_mut()
                        //     .enumerate()
                        //     .filter(|(i, _)| targets.contains(i))
                        //     .for_each(|(_, m)| act.apply(m.get_mut(), self));
                    }
                } else {
                    break;
                }
            }
        }
    }
    pub fn play(&mut self) -> Option<u8> {
        let mut teams = HashSet::new();
        self.team_alive(&mut teams);

        //Loop over the turn and round
        while teams.len() > 1 {
            teams.drain();
            self.advance_round();
            self.team_alive(&mut teams);
        }

        //Return the winner team
        if teams.len() == 0 {
            None
        } else {
            Some(*teams.iter().last().unwrap())
        }
    }
    pub fn team_alive(&self, teams: &mut HashSet<u8>) {
        for e in self.entities.iter() {
            let e = e.borrow();
            eprintln!("{} : {}", e.name(), e.hp());
            if e.is_alive() {
                teams.insert(e.team());
            }
        }
    }
    pub fn decrease_hp(&mut self, id: i32, amount: i32) {
        todo!("Fight::inflict_damage");
    }
    pub fn get_hp(&mut self, id: i32) -> i32 {
        todo!("Fight::get_hp");
    }
    pub fn get_ac(&mut self, id: i32) -> i32 {
        todo!("Fight::get_ac");
    }
    // pub fn add_condition(&mut self, source_id : i32, target_id : i32, ) NOTE must be breakable
    pub fn get_modifier(&self, id: i32, mod_type: ModifierType) -> Modifier {
        let modifier = self.effects
            .iter()
            .filter_map(|effect| { //Get effects that targets id and that affect my Modtype
                if effect.targets(id) && effect.affects(mod_type) {
                    effect.get_modifier(mod_type)
                } else {
                    None
                }
            })
            .fold(Modifier::default(), |mut acc, e| {
                acc += e;
                acc
            });


        todo!("Fight::get_modifier | Add mod of the target for the saving throws.");
        modifier
    }
    pub fn get_entities(&self) -> &Vec<RefCell<Monster>> {
        &self.entities
    }
}
