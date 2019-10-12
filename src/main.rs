mod character;
mod item;

use crate::character::{Align, Character, Class};
use crate::item::Item;

fn main() {
    let items: Vec<Item> = serde_json::from_str(include_str!("../res/items.json")).unwrap();

    println!("Loaded {} items", items.len());

    let ch = Character::new(31, Class::Warrior, Align::Neutral, &items);
    ch.solve();
}
