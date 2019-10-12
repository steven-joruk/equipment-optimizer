use crate::character::{Align, Class};
use serde::Deserialize;

use std::{
    cmp::{Ord, Ordering},
    fmt::{self, Display},
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum Location {
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Item {
    pub name: String,
    pub locations: Vec<Location>,
    #[serde(default)]
    pub level: u8,
    #[serde(default)]
    pub hp: i8,
    #[serde(default)]
    pub mana: i8,
    #[serde(default)]
    pub hr: i8,
    #[serde(default)]
    pub dr: i8,
    #[serde(default)]
    pub ss: i8,
    #[serde(default)]
    pub sbr: i8,
    #[serde(default)]
    pub spet: i8,
    #[serde(default)]
    pub ac_apply: i8,
    #[serde(default)]
    pub align_restrictions: Vec<Align>,
    #[serde(default)]
    pub class_restrictions: Vec<Class>,
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Show its stats
        write!(f, "{}", self.name)
    }
}

impl Item {
    pub fn value(&self) -> i32 {
        i32::from(self.hr * 5 + self.dr * 10 + self.ss * 4)
    }
}

impl Ord for Item {
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

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
