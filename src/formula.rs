use crate::dice::Dice;
use crate::utils::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Formula {
    //TODO can be improved with more dices
    #[serde(deserialize_with = "string_or_struct")]
    dice: Dice,
    fixed: i32,
}
impl Formula {
    pub fn roll(&self) -> i32 {
        self.dice.roll() + self.fixed
    }
    pub fn add_dice(&mut self, amount: i32) {
        self.dice.add_dice(amount);
    }
    pub fn add_fixed(&mut self, amount: i32) {
        self.fixed += amount;
    }
    pub fn is_formula(s: &str) -> bool {
        let reg = Regex::new(r"^([+\-]?[0-9]+)$|^[0-9]+d[1-9][0-9]*([+\-][0-9]*|)").unwrap();
        reg.find(s).is_some()
    }
    pub fn average_roll(&self) -> f32 {
        self.dice.average_roll() + self.fixed as f32
    }
}
impl From<&str> for Formula {
    fn from(item: &str) -> Self {
        /*
         * Regex capture 3d6+2, 3d6-2, 3d6
         * Since there is 2 capture zone, then capture will have 2 capture area, one for each
         */
        let reg_fixed = Regex::new(r"^([+\-]?[0-9]+)$|^[0-9]+d[1-9][0-9]*([+\-][0-9]*|)").unwrap();
        if let Some(capture) = reg_fixed.captures(item) {
            let (fixed, dice) = if let Some(m) = capture.get(1) {
                (m.as_str().parse::<i32>().unwrap_or(0), Dice::new(0, 1))
            } else if let Some(m) = capture.get(2) {
                (m.as_str().parse::<i32>().unwrap_or(0), Dice::from(item))
            } else {
                unreachable!("Formula::from: regex matched bu no pattern are available.");
            };
            Self { dice, fixed }
        } else {
            panic!("The Formula string has the wrong format : {item}");
        }
    }
}
impl From<&String> for Formula {
    fn from(item: &String) -> Self {
        Self::from(item.as_str())
    }
}
impl FromStr for Formula {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}
impl Add for Formula {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            dice: self.dice + other.dice,
            fixed: self.fixed + other.fixed,
        }
    }
}
impl AddAssign for Formula {
    fn add_assign(&mut self, other: Self) {
        self.dice += other.dice;
        self.fixed += other.fixed;
    }
}
impl Sub for Formula {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            dice: self.dice - other.dice,
            fixed: self.fixed - other.fixed,
        }
    }
}
impl SubAssign for Formula {
    fn sub_assign(&mut self, other: Self) {
        self.dice -= other.dice;
        self.fixed -= other.fixed;
    }
}
impl Mul<i32> for Formula {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            dice: self.dice * other,
            fixed: self.fixed * other,
        }
    }
}
impl MulAssign<i32> for Formula {
    fn mul_assign(&mut self, other: i32) {
        self.dice *= other;
        self.fixed *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from() {
        let f = Formula::from("-31");
        assert_eq!(f.fixed, -31);
        assert_eq!(f.dice.dice_count(), 0);
        let f = Formula::from("7");
        assert_eq!(f.fixed, 7);
        assert_eq!(f.dice.dice_count(), 0);
        let f = Formula::from("3d6");
        assert_eq!(f.fixed, 0);
        assert_eq!(f.dice.face_count(), 6);
        assert_eq!(f.dice.dice_count(), 3);
        let f = Formula::from("2d7+8");
        assert_eq!(f.fixed, 8);
        assert_eq!(f.dice.face_count(), 7);
        assert_eq!(f.dice.dice_count(), 2);
        let f = Formula::from("30d20-10");
        assert_eq!(f.fixed, -10);
        assert_eq!(f.dice.face_count(), 20);
        assert_eq!(f.dice.dice_count(), 30);
    }
    #[test]
    fn is_formula() {
        assert!(Formula::is_formula("27"));
        assert!(Formula::is_formula("3d6"));
        assert!(Formula::is_formula("123d6"));
        assert!(Formula::is_formula("1d234"));
        assert!(Formula::is_formula("23d4-1"));
        assert!(Formula::is_formula("23d4-10"));
        assert!(Formula::is_formula("23d4+17"));
        assert!(Formula::is_formula("23d6+7"));
        assert!(Formula::is_formula("06d6+7"));
        assert!(Formula::is_formula("6d4+0"));
        assert!(Formula::is_formula("0d6+7"));
        assert!(Formula::is_formula("0d6+0"));
        assert!(!Formula::is_formula("6d0+7"));
    }
}
