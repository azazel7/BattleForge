use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use BattleForge::fight::Fight;
use BattleForge::resource::{Charge, Resource};
use BattleForge::template::MonsterTemplate;
use BattleForge::template::SpellTemplate;
use BattleForge::template::TemplateBuilder;

fn main() {
    let j = serde_json::to_string(&Charge::Infinite).unwrap();
    println!("{}", j);
    let j = serde_json::to_string(&Charge::Limited(3)).unwrap();
    println!("{}", j);
    let j = serde_json::to_string(&Resource::Action).unwrap();
    println!("{}", j);
    let j = serde_json::to_string(&(1, Resource::Action)).unwrap();
    println!("{}", j);

    let mut file = File::open("gobelin.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let monster_database: Vec<MonsterTemplate> = serde_json::from_str(&data).unwrap();

    let mut file = File::open("spells.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let spell_database: HashMap<String, SpellTemplate> = serde_json::from_str(&data).unwrap();
    println!("{:?}", spell_database);
    // println!("{:?}", monster_database);

    let mut builder = TemplateBuilder::new(monster_database, spell_database);

    // let kelani = builder.create("Kelani").team(0).build();
    // println!("{:?}", kelani);
    //TODO change their team
    let monsters = vec![
        builder.create("Gobelin").hp(7).team(0).build(),
        builder.create("Gobelin").hp(8).team(0).build(),
        // builder.create("Black Bear").team(1).build(),
        builder.create("Kelani").team(1).build(),
    ];
    let mut fight = Fight::new(monsters);
    let winner = fight.play();
    println!("Winner is {winner:?}");
}
