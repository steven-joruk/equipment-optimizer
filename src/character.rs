use crate::error::{Error, Result};
use crate::item::{Item, Location};
use crate::itemset::ItemSet;
use enum_map::EnumMap;
use itertools::Itertools;
use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Align {
    Evil,
    Good,
    Neutral,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum Class {
    Cleric,
    Mage,
    Thief,
    Warrior,
}

pub struct Character {
    level: u8,
    class: Class,
    align: Align,
    usable: EnumMap<Location, Vec<Item>>,
}

impl Character {
    pub fn new(level: u8, class: Class, align: Align, items: &[Item]) -> Self {
        let mut ch = Character {
            level,
            class,
            align,
            usable: Default::default(),
        };

        for item in items {
            if ch.can_use(&item) {
                for loc in &item.locations {
                    ch.usable[*loc].push(item.clone());
                }
            }
        }

        // Itertools multi_cartesian_product will produce no combinations if
        // any of the sub-iterators have no items, so fill in the gaps with a
        // placeholder.
        for items in ch.usable.values_mut() {
            if items.is_empty() {
                // TODO: Overwrite items which have matching (or worse) stats
                // in the same slot.
                items.push(Item::none())
            }
        }

        ch
    }

    pub fn find_best_item_set(&self) -> Result<ItemSet> {
        println!(
            "Usable items per slot for a level {} {:?} {:?}:",
            self.level, self.align, self.class
        );

        let mut upper_limit = 0;
        for (k, v) in &self.usable {
            println!("    {:?}: {}", k, v.len());
            upper_limit = if upper_limit == 0 { 1 } else { upper_limit } * v.len();
        }

        println!("There are at most {} combinations", upper_limit);

        let mut combinations = vec![
            &self.usable[Location::Light],
            &self.usable[Location::Finger],
            &self.usable[Location::Finger],
            &self.usable[Location::Neck],
            &self.usable[Location::Neck],
            &self.usable[Location::Body],
            &self.usable[Location::Head],
            &self.usable[Location::Legs],
            &self.usable[Location::Feet],
            &self.usable[Location::Hands],
            &self.usable[Location::Arms],
            &self.usable[Location::Offhand],
            &self.usable[Location::About],
            &self.usable[Location::Waist],
            &self.usable[Location::Wrist],
            &self.usable[Location::Wrist],
            &self.usable[Location::Wielded],
            &self.usable[Location::Held],
            &self.usable[Location::Aura],
            &self.usable[Location::Spirit],
        ]
        .into_iter()
        .multi_cartesian_product();

        let mut best = match combinations.next() {
            Some(c) => ItemSet::try_from(c)?,
            None => return Err(Error::NoCombinations),
        };

        for combination in combinations {
            let set = ItemSet::try_from(combination)?;

            if set.value > best.value {
                best = set;
            }
        }

        Ok(best)
    }

    fn can_use(&self, eq: &Item) -> bool {
        self.level >= eq.level
            && !eq.align_restrictions.contains(&self.align)
            && !eq.class_restrictions.contains(&self.class)
    }
}
