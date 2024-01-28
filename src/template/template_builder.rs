use std::collections::HashMap;
use crate::template::MonsterTemplate;
use crate::monster::Monster;

#[derive(Default, Clone, Debug)]
pub struct TemplateBuilder {
    monster_database: HashMap<String, MonsterTemplate>,
    current_monster: Option<Monster>,
}

impl TemplateBuilder {
    pub fn new(monsters: Vec<MonsterTemplate>) -> Self {
        Self {
            monster_database: monsters
                .into_iter()
                .map(|template| (template.name.clone(), template))
                .collect(),
            current_monster: None,
        }
    }
    pub fn create(&mut self, name: &str) -> &mut Self {
        assert!(self.monster_database.contains_key(name));
        let template = self.monster_database.get(name).unwrap();
        self.current_monster = Some(Monster::from(template));
        self
    }
    pub fn team(&mut self, team: i32) -> &mut Self {
        assert!(self.current_monster.is_some());
        if let Some(monster) = &mut self.current_monster {
            monster.set_team(team as u8);
        }
        self
    }
    pub fn hp(&mut self, hp: i32) -> &mut Self {
        assert!(self.current_monster.is_some());
        if let Some(monster) = &mut self.current_monster {
            monster.set_hp(hp as i8);
        }
        self
    }
    pub fn build(&mut self) -> Monster {
        if let Some(monster) = self.current_monster.take().to_owned() {
            monster
        } else {
            panic!("Error building a monster. No monster is being created");
        }
    }
}

