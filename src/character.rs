use crate::error::Result;
use crate::item::{Item, Location};
use crate::itemset::ItemSet;

use enum_map::EnumMap;
use indicatif::{ProgressBar, ProgressIterator};
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
    pub fn new(level: u8, class: Class, align: Align, items: Vec<Item>) -> Self {
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
                items.push(Item::none());
            }
        }

        ch
    }

    pub fn find_best_item_set(&self) -> Result<ItemSet> {
        println!(
            "Usable items per slot for a level {} {:?} {:?}:",
            self.level, self.align, self.class
        );

        let combos = vec![
            self.usable[Location::Light].iter(),
            self.usable[Location::Finger].iter(),
            self.usable[Location::Finger].iter(),
            self.usable[Location::Neck].iter(),
            self.usable[Location::Neck].iter(),
            self.usable[Location::Body].iter(),
            self.usable[Location::Head].iter(),
            self.usable[Location::Legs].iter(),
            self.usable[Location::Feet].iter(),
            self.usable[Location::Hands].iter(),
            self.usable[Location::Arms].iter(),
            self.usable[Location::Offhand].iter(),
            self.usable[Location::About].iter(),
            self.usable[Location::Waist].iter(),
            self.usable[Location::Wrist].iter(),
            self.usable[Location::Wrist].iter(),
            self.usable[Location::Wielded].iter(),
            self.usable[Location::Held].iter(),
            self.usable[Location::Aura].iter(),
            self.usable[Location::Spirit].iter(),
        ]
        .into_iter()
        .multi_cartesian_product();

        let pbar = ProgressBar::new(combos.size_hint().1.unwrap() as u64);

        let best = combos
            .progress_with(pbar)
            .map(ItemSet::try_from)
            .map(|s| s.unwrap())
            .max_by_key(|s| s.value)
            .unwrap();

        Ok(best)
    }

    fn can_use(&self, eq: &Item) -> bool {
        self.level >= eq.level
            && !eq.align_restrictions.contains(&self.align)
            && !eq.class_restrictions.contains(&self.class)
    }
}
