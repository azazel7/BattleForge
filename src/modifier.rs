use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};

use crate::{ability::Ability, formula::Formula};

#[derive(Default, Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ModifierType {
    Save(Ability),
    ReceiveDamage,
    DealDamage,
    #[default]
    Attack,
    Attacked,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Modifier {
    //NOTE formulas should probably include a modifier of the target? Maybe, maybe not.
    formulas: Vec<Formula>,
    formula_multipliers: Vec<i32>,
    mod_type: ModifierType,
    advantage: i32, //positive means has advantage and negative means has disadvantage
}

impl Modifier {
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..=20);
        let throw = die.sample(&mut rng);
        todo!("Modifier::roll");
    }
}
impl From<i32> for Modifier {
    fn from(value: i32) -> Self {
        Self {
            formulas: vec![Formula::from(value)],
            formula_multipliers: vec![1],
            mod_type: ModifierType::default(),
            advantage: 0,
        }
    }
}
impl AddAssign<&Modifier> for Modifier {
    fn add_assign(&mut self, other: &Self) {
        todo!("Modifier::add_assign");
    }
}
impl Add for Modifier {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        todo!("Modifier::add (Add trait)");
    }
}
