use crate::dice::Dice;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Formula {
    dice : Dice,
    fixed : i32,
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
