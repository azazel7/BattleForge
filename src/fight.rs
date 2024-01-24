use crate::{event::Event, monster::Monster};
use std::collections::HashSet;

pub struct Fight {
    entities: Vec<Monster>,
}
impl Fight {
    pub fn new(entities: Vec<Monster>) -> Self {
        Self { entities }
    }
    pub fn advance_round(&mut self) {
        eprintln!("==== New Round ====");
        //The loop for each entity's turn
        for idx in 0..self.entities.len() {
            {
                //NOTE this thing *must* be *mut*
                let mut e = self.entities.get_mut(idx).unwrap();
                e.new_turn();
            }
            loop {
                //TODO I'd like to be able to modify this entity for limited charge
                let e: &Monster = self.entities.get(idx).unwrap();
                let mut event = None;

                {
                    let is_alive = e.is_alive();
                    if is_alive {
                        event = e.take_action(self);
                    }
                }
                if let Some(event) = event {
                    eprintln!("Playing {} {idx} (hp: {})", e.name(), e.hp());
                    self.entities
                        .iter_mut()
                        .enumerate()
                        .filter(|(i, _)| event.is_target(*i as i8))
                        .for_each(|(_, m)| event.run(m));
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
            eprintln!("{} : {}", e.name(), e.hp());
            if e.is_alive() {
                teams.insert(e.team());
            }
        }
    }
    pub fn get_entities(&self) -> &Vec<Monster> {
        &self.entities
    }
}
