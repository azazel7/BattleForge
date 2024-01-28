pub use self::monster::MonsterTemplate;
pub use self::stats::MonsterStatsTemplate;
pub use self::action::ActionTemplate;
pub use self::spell::SpellTemplate;
pub use self::template_builder::TemplateBuilder;

mod monster;
mod action;
mod stats;
mod spell;
mod template_builder;

pub trait FromTemplate<T>: Sized {
    fn from_template(builder: &TemplateBuilder, value: T) -> Self;
}

