use std::fs::File;
use std::io::Read;
use BattleForge::fight::Fight;
use BattleForge::monster::MonsterBuilder;
use BattleForge::template::MonsterTemplate;
use BattleForge::resource::{Charge, Resource};

fn main() {
    let mut file = File::open("gobelin.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let parsed: Vec<MonsterTemplate> = serde_json::from_str(&data).unwrap();
    let mut builder = MonsterBuilder::new(parsed);
    println!("{:?}", builder);
    //TODO change their team
    let monsters = vec![
        builder.create("Gobelin").hp(7).team(0).build(),
        builder.create("Gobelin").hp(8).team(0).build(),
        builder.create("Black Bear").team(1).build(),
    ];
    let mut fight = Fight::new(monsters);
    let winner = fight.play();
    println!("Winner is {winner:?}");
}
