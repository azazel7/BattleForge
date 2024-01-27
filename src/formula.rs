use crate::dice::Dice;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Formula {
    //TODO can be improved with more dices
    dice : Dice,
    fixed : i32,
}
impl Formula {
    pub fn roll(&self) -> i32 {
        self.dice.roll() + self.fixed
    }
    pub fn is_formula(s: &str) -> bool {
        let reg = Regex::new(r"[1-9][0-9]*d[1-9][0-9]*([+\-][1-9][0-9]*|)").unwrap();
        reg.find(s).is_some()
    }
}
impl From<&str> for Formula {
    fn from(item: &str) -> Self {
        /*
         * Regex capture 3d6+2, 3d6-2, 3d6
         */
        let reg_fixed = Regex::new(r"[1-9][0-9]*d[1-9][0-9]*([+\-][1-9][0-9]*|)").unwrap();
        if let Some(capture) = reg_fixed.captures(item) {
            let fixed = capture.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0);
            Self {
                dice : Dice::from(item),
                fixed, 
            }
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from() {
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
        assert!(Formula::is_formula("3d6"));
        assert!(Formula::is_formula("123d6"));
        assert!(Formula::is_formula("1d234"));
        assert!(Formula::is_formula("23d4-1"));
        assert!(Formula::is_formula("23d4-10"));
        assert!(Formula::is_formula("23d4+17"));
        assert!(Formula::is_formula("23d6+7"));
        assert!(Formula::is_formula("06d6+7"));
        assert!(Formula::is_formula("6d4+0"));
        assert!(!Formula::is_formula("0d6+7"));
        assert!(!Formula::is_formula("6d0+7"));
    }
}
