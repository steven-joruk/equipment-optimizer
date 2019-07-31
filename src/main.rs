use std::{
    cmp::{Ord, Ordering},
    fmt::{self, Display},
};

use itertools::Itertools;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
enum Location {
    Light,
    Finger,
    Neck,
    Body,
    Head,
    Legs,
    Feet,
    Hands,
    Arms,
    Shield,
    About,
    Waist,
    Wrist,
    Wielded,
    Held,
    Aura,
    Spirit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
enum Align {
    Evil,
    Good,
    Neutral,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
enum Class {
    Cleric,
    Mage,
    Thief,
    Warrior,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
struct Equipment {
    name: String,
    locations: Vec<Location>,
    #[serde(default)]
    level: u8,
    #[serde(default)]
    hp: i8,
    #[serde(default)]
    mana: i8,
    #[serde(default)]
    hr: i8,
    #[serde(default)]
    dr: i8,
    #[serde(default)]
    ss: i8,
    #[serde(default)]
    sbr: i8,
    #[serde(default)]
    spet: i8,
    #[serde(default)]
    ac_apply: i8,
    #[serde(default)]
    align_restrictions: Vec<Align>,
    #[serde(default)]
    class_restrictions: Vec<Class>,
}

impl Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.value(), self.name)
    }
}

impl Equipment {
    fn value(&self) -> i32 {
        i32::from(self.hr * 5 + self.dr * 10 + self.ss * 4)
    }
}

impl Ord for Equipment {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = self.value();
        let other_value = other.value();

        if self_value < other_value {
            Ordering::Greater
        } else if self_value > other_value {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Equipment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Character {
    level: u8,
    class: Class,
    align: Align,
    // TODO: Use an array indexed by enums if/when it becomes usable.
    lights: Vec<Equipment>,
    necks: Vec<Equipment>,
    heads: Vec<Equipment>,
    auras: Vec<Equipment>,
    spirits: Vec<Equipment>,
}

impl Character {
    fn new(level: u8, class: Class, align: Align, items: &[Equipment]) -> Self {
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

    fn solve(&self) {
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

        let mut best: Option<Vec<&Equipment>> = None;
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

    fn can_use(&self, eq: &Equipment) -> bool {
        self.level >= eq.level
            && !eq.align_restrictions.contains(&self.align)
            && !eq.class_restrictions.contains(&self.class)
    }
}

fn main() {
    let items: Vec<Equipment> =
        serde_json::from_str(include_str!("../res/equipment.json")).unwrap();

    println!("Loaded {} items", items.len());

    let ch = Character::new(31, Class::Warrior, Align::Neutral, &items);

    ch.solve();
}
