use crate::item::{Item, Location};
use itertools::Itertools;
use serde::Deserialize;

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
            lights: Vec::new(),
            necks: Vec::new(),
            heads: Vec::new(),
            auras: Vec::new(),
            spirits: Vec::new(),
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

    pub fn solve(&self) {
        println!("Usable items per slot:");
        println!("    Light: {}", self.lights.len());
        println!("    Neck:  {}", self.necks.len());
        println!("    Head:  {}", self.heads.len());
        println!("    Aura:  {}", self.auras.len());
        println!("    Spirit:  {}", self.spirits.len());

        let usable = vec![&self.heads, &self.necks];

        let upper_limit = usable
            .iter()
            .fold(0, |acc, vec| std::cmp::max(acc, 1) * vec.len());

        println!("There are at most {} combinations", upper_limit);

        let combinations = usable.into_iter().multi_cartesian_product().clone();

        let mut best: Option<Vec<&Item>> = None;
        let mut best_value = 0;

        for set in combinations {
            // TODO: Check it's usable
            let value = set.iter().fold(0, |acc, item| acc + item.value());
            if best.is_none() || value > best_value {
                best = Some(set);
                best_value = value;
            }
        }

        match best {
            Some(set) => println!("Found a set with a value of {}: {:?}", best_value, set),
            None => println!("Didn't find any sets"),
        }
    }

    fn can_use(&self, eq: &Item) -> bool {
        self.level >= eq.level
            && !eq.align_restrictions.contains(&self.align)
            && !eq.class_restrictions.contains(&self.class)
    }
}
