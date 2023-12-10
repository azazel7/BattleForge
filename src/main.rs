use BattleForge::fight::Fight;
use BattleForge::monster::Monster;
use BattleForge::monster::MonsterBuilder;
use BattleForge::monster::MonsterTemplate;
use std::fs::File;
use std::io::Read;


fn main() {
    println!("Hello, world!");
    let mut file = File::open("gobelin.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let parsed: Vec<MonsterTemplate> = serde_json::from_str(&data).unwrap();
    let builder = MonsterBuilder::new(parsed);
    println!("{:?}", builder);
    let parsed: Vec<Monster> = serde_json::from_str(&data).unwrap();
    println!("{:?}", parsed);
    let mut fight = Fight::new(parsed);
    // let winner = fight.play();
    // println!("Winner is {winner:?}");
}


