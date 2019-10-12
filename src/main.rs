mod character;
mod error;
mod item;
mod itemset;

use crate::character::{Align, Character, Class};
use crate::error::Result;
use crate::item::Item;

fn main() -> Result<()> {
    let items: Vec<Item> = serde_json::from_str(include_str!("../res/items.json")).unwrap();

    println!("Loaded {} items", items.len());

    let ch = Character::new(31, Class::Warrior, Align::Neutral, &items);
    let set = ch.find_best_item_set()?;
    println!("{}", set);

    Ok(())
}
