use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dice {
    dice_count: i32,
    face_count: i32,
}

impl Dice {
    pub fn new(dice_count: i32, face_count: i32) -> Self {
        Self {
            dice_count,
            face_count,
        }
    }
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut sum = 0;
        for _ in 0..self.dice_count {
            sum += rng.gen_range(1..=self.face_count);
        }
        sum
    }
    pub fn is_dice(s: &str) -> bool {
        let reg = Regex::new(r"[0-9]+d([1-9][0-9]*)").unwrap();
        reg.captures(s).is_some()
    }
    pub fn add_dice(&mut self, amount: i32) {
        self.dice_count += amount;
        self.dice_count = self.dice_count.max(0);
    }
    pub fn dice_count(&self) -> i32 {
        self.dice_count
    }
    pub fn face_count(&self) -> i32 {
        self.face_count
    }
}

impl From<&str> for Dice {
    fn from(item: &str) -> Self {
        let reg = Regex::new(r"([0-9]+)d([1-9][0-9]*)").unwrap();
        if let Some(capture) = reg.captures(item) {
            let dice_count = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let face_count = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            Self {
                face_count,
                dice_count,
            }
        } else {
            panic!("The Dice string has the wrong format : {item}");
        }
    }
}
impl From<&String> for Dice {
    fn from(item: &String) -> Self {
        Self::from(item.as_str())
    }
}
impl FromStr for Dice {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

