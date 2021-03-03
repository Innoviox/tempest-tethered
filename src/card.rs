use crate::effect::Effect;

pub enum CardType {
    Spell, Unit, Structure
}

pub enum UnitType {
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
    name: String,
    ctype: CardType,

    cost: usize,
    strength: usize,
    speed: usize,

    effect: Effect,

    class: UnitType,
    kingdom: Kingdom,
    rarity: Rarity,
    level: usize
}