use crate::{action::*, monster::Monster};
use core::cell::RefCell;
use std::collections::HashSet;

pub struct Fight {
    entities: Vec<RefCell<Monster>>,
}
impl Fight {
    pub fn new(entities: Vec<Monster>) -> Self {
        Self {
            entities: entities.into_iter().map(|e| RefCell::new(e)).collect(),
        }
    }
    pub fn advance_round(&mut self) {
        eprintln!("==== New Round ====");
        //The loop for each entity's turn
        for idx in 0..self.entities.len() {
            {
                //NOTE this thing *must* be *mut*
                let e = self.entities.get(idx).unwrap();
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

                if let Some(action) = action {
                    for act in action.get_components() {
                        let e = self.entities.get(idx).unwrap().borrow();
                        let targets = e.get_targets(self, act);
                        drop(e);
                        //TODO what about the action that affect the fight (turn into a wolf, add
                        //another monster or effect)
                        self.entities
                            .iter_mut()
                            .enumerate()
                            .filter(|(i, _)| targets.contains(i))
                            .for_each(|(_, m)| act.apply(m.get_mut()));
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
    pub fn get_entities(&self) -> &Vec<RefCell<Monster>> {
        &self.entities
    }
}
