use BattleForge::dice::Dice;
use BattleForge::fight::Fight;
use BattleForge::monster::Monster;
use BattleForge::monster::MonsterBuilder;
use BattleForge::monster::MonsterTemplate;
use std::fs::File;
use std::io::Read;


fn main() {
    println!("Hello, world!");
    let dice = Dice::from("3d6");
    println!("roll {}", dice.roll());
    let mut file = File::open("gobelin.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let parsed: Vec<MonsterTemplate> = serde_json::from_str(&data).unwrap();
    let builder = MonsterBuilder::new(parsed);
    println!("{:?}", builder);
    //TODO change their team
    let mut monsters = vec![builder.build("Gobelin"), builder.build("Gobelin"), builder.build("Black Bear")];

    monsters[0].set_team(0);
    monsters[1].set_team(0);
    monsters[2].set_team(1);
    // let parsed: Vec<Monster> = serde_json::from_str(&data).unwrap();
    // println!("{:?}", parsed);
    let mut fight = Fight::new(monsters);
    let winner = fight.play();
    println!("Winner is {winner:?}");
}


