use crate::error::{Error, Result};
use crate::item::{Item, Location};
use crate::itemset::ItemSet;
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
    // TODO: Use an array indexed by enums if/when it becomes usable.
    lights: Vec<Item>,
    necks: Vec<Item>,
    heads: Vec<Item>,
    auras: Vec<Item>,
    spirits: Vec<Item>,
}

impl Character {
    pub fn new(level: u8, class: Class, align: Align, items: &[Item]) -> Self {
        let mut ch = Character {
            level,
            class,
            align,
            lights: vec![Item::none()],
            necks: vec![Item::none()],
            heads: vec![Item::none()],
            auras: vec![Item::none()],
            spirits: vec![Item::none()],
        };

        for item in items {
            if ch.can_use(&item) {
                for loc in &item.locations {
                    match loc {
                        Location::Light => ch.lights.push(item.clone()),
                        Location::Neck => ch.necks.push(item.clone()),
                        Location::Head => ch.heads.push(item.clone()),
                        Location::Aura => ch.auras.push(item.clone()),
                        Location::Spirit => ch.spirits.push(item.clone()),
                        _ => {}
                    }
                }
            }
        }

        ch
    }

    // TODO: Switch to Result, and remove `.ok()?` calls.
    pub fn find_best_item_set(&self) -> Result<ItemSet> {
        println!(
            "Usable items per slot for a level {} {:?} {:?}:",
            self.level, self.align, self.class
        );
        println!("    Light:  {}", self.lights.len() - 1);
        println!("    Neck:   {}", self.necks.len() - 1);
        println!("    Head:   {}", self.heads.len() - 1);
        println!("    Spirit: {}", self.spirits.len() - 1);
        println!("    Aura:   {}", self.auras.len() - 1);

        let usable = vec![
            &self.lights,
            &self.necks,
            &self.heads,
            &self.auras,
            &self.spirits,
        ];

        let upper_limit = usable
            .iter()
            .fold(0, |acc, vec| std::cmp::max(acc, 1) * vec.len());

        println!("There are at most {} combinations", upper_limit);

        let mut combinations = usable.into_iter().multi_cartesian_product().clone();

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
