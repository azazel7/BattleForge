use BattleForge::fight::Fight;
use BattleForge::monster::Monster;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;


fn main() {
    println!("Hello, world!");
    let mut file = File::open("gobelin.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    println!("{data}");
    let parsed: Vec<Monster> = serde_json::from_str(&data).unwrap();
    println!("{:?}", parsed);
    let mut fight = Fight::new(parsed);
    let winner = fight.play();
    println!("Winner is {winner:?}");
}

trait Action {
    fn apply(m: &mut Monster);
}


