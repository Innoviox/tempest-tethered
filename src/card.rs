use crate::effect::Effect;

pub enum Type {
    Dragon, Elder, Hero,
    Satyr, Undead,
    Frostling, Dwarf,
    Construct, Rodent,
    Raven, Toad,
    Knights, Pirate, Feline
}

pub enum Kingdom {
    Swarm, Winter, Ironclad, Shadowfen, Neutral
}

pub enum Rarity {
    Common, Rare, Epic, Legendary
}

pub struct Card {
    cost: usize,
    strength: usize,
    speed: usize,

    effect: Effect,

    name: String,
    class: Type,
    kingdom: Kingdom,
    rarity: Rarity,
    level: usize
}