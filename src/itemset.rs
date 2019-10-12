use crate::error::Error;
use crate::item::{Item, Location};
use std::convert::TryFrom;
use std::fmt;

pub struct ItemSet<'a> {
    pub value: i32,
    light: &'a Item,
    neck: &'a Item,
    head: &'a Item,
    spirit: &'a Item,
    aura: &'a Item,
}

impl<'a> TryFrom<Vec<&'a Item>> for ItemSet<'a> {
    type Error = crate::error::Error;

    fn try_from(items: Vec<&'a Item>) -> Result<Self, Self::Error> {
        let mut set = ItemSet {
            value: 0,
            light: items.get(0).ok_or(Error::MissingItem {
                loc: Location::Light,
            })?,
            neck: items.get(1).ok_or(Error::MissingItem {
                loc: Location::Neck,
            })?,
            head: items.get(2).ok_or(Error::MissingItem {
                loc: Location::Head,
            })?,
            spirit: items.get(3).ok_or(Error::MissingItem {
                loc: Location::Spirit,
            })?,
            aura: items.get(4).ok_or(Error::MissingItem {
                loc: Location::Aura,
            })?,
        };

        // TODO: Validate the set:
        // - Locations are correct
        // - Multislot
        // - No shield with 2H
        // - etc.

        set.value = set.light.value()
            + set.neck.value()
            + set.head.value()
            + set.spirit.value()
            + set.aura.value();

        Ok(set)
    }
}

impl<'a> fmt::Display for ItemSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "The set has a total value of {}", self.value)?;
        writeln!(f, "    Light:  {}", self.light)?;
        writeln!(f, "    Neck:   {}", self.neck)?;
        writeln!(f, "    Head:   {}", self.head)?;
        writeln!(f, "    Spirit: {}", self.spirit)?;
        writeln!(f, "    Aura:   {}", self.aura)
    }
}
