use crate::monster::Monster;
use crate::template::MonsterTemplate;
use std::collections::HashMap;

use super::SpellTemplate;

#[derive(Default, Clone, Debug)]
pub struct TemplateBuilder {
    monster_database: HashMap<String, MonsterTemplate>,
    spell_database: HashMap<String, SpellTemplate>,
    current_monster: Option<Monster>,
}

impl TemplateBuilder {
    pub fn new(monsters: Vec<MonsterTemplate>, spells : HashMap<String, SpellTemplate>) -> Self {
        Self {
            monster_database: monsters
                .into_iter()
                .map(|template| (template.name.clone(), template))
                .collect(),
            spell_database: spells,
            current_monster: None,
        }
    }
    pub fn create(&mut self, name: &str) -> &mut Self {
        assert!(self.monster_database.contains_key(name));
        let template = self.monster_database.get(name).unwrap();
        self.current_monster = Some(Monster::from_template(self, template));
        self
    }
    pub fn get_spell_template(&self, name: &str) -> SpellTemplate {
        assert!(self.spell_database.contains_key(name));
        let template = self.spell_database.get(name).unwrap();
        template.clone()
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
            monster.set_hp(hp);
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
