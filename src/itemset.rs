use crate::item::Item;
use std::convert::TryFrom;
use std::fmt;

pub struct ItemSet<'a> {
    pub value: i32,
    light: &'a Item,
    finger1: &'a Item,
    finger2: &'a Item,
    neck1: &'a Item,
    neck2: &'a Item,
    body: &'a Item,
    head: &'a Item,
    legs: &'a Item,
    feet: &'a Item,
    hands: &'a Item,
    arms: &'a Item,
    offhand: &'a Item,
    about: &'a Item,
    waist: &'a Item,
    wrist1: &'a Item,
    wrist2: &'a Item,
    wielded: &'a Item,
    held: &'a Item,
    spirit: &'a Item,
    aura: &'a Item,
}

impl<'a> TryFrom<Vec<&'a Item>> for ItemSet<'a> {
    type Error = crate::error::Error;

    fn try_from(items: Vec<&'a Item>) -> Result<Self, Self::Error> {
        let mut set = ItemSet {
            value: 0,
            light: items[0],
            finger1: items[1],
            finger2: items[2],
            neck1: items[3],
            neck2: items[4],
            body: items[5],
            head: items[6],
            legs: items[7],
            feet: items[8],
            hands: items[9],
            arms: items[10],
            offhand: items[11],
            about: items[12],
            waist: items[13],
            wrist1: items[14],
            wrist2: items[15],
            wielded: items[16],
            held: items[17],
            aura: items[18],
            spirit: items[19],
        };

        // TODO: Validate the set:
        // - Locations are correct
        // - Multislot
        // - No shield with 2H
        // - etc.

        set.value = set.light.value()
            + set.finger1.value()
            + set.finger2.value()
            + set.neck1.value()
            + set.neck2.value()
            + set.body.value()
            + set.head.value()
            + set.legs.value()
            + set.feet.value()
            + set.hands.value()
            + set.arms.value()
            // TODO: Offhand attributes are calculated differently if dual wielding.
            + set.offhand.value()
            + set.about.value()
            + set.waist.value()
            + set.wrist1.value()
            + set.wrist2.value()
            + set.wielded.value()
            + set.held.value()
            + set.spirit.value()
            + set.aura.value();

        Ok(set)
    }
}

impl<'a> fmt::Display for ItemSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "The set has a total value of {}", self.value)?;
        writeln!(f, "    Light:   {}", self.light)?;
        writeln!(f, "    Finger:  {}", self.finger1)?;
        writeln!(f, "    Finger:  {}", self.finger2)?;
        writeln!(f, "    Neck:    {}", self.neck1)?;
        writeln!(f, "    Neck:    {}", self.neck2)?;
        writeln!(f, "    Body:    {}", self.body)?;
        writeln!(f, "    Head:    {}", self.head)?;
        writeln!(f, "    Legs:    {}", self.legs)?;
        writeln!(f, "    Feet:    {}", self.feet)?;
        writeln!(f, "    Hands:   {}", self.hands)?;
        writeln!(f, "    Arms:    {}", self.arms)?;
        writeln!(f, "    Offhand: {}", self.offhand)?;
        writeln!(f, "    About:   {}", self.about)?;
        writeln!(f, "    Waist:   {}", self.waist)?;
        writeln!(f, "    Wrist:   {}", self.wrist1)?;
        writeln!(f, "    Wrist:   {}", self.wrist2)?;
        writeln!(f, "    Wielded: {}", self.wielded)?;
        writeln!(f, "    Held:    {}", self.held)?;
        writeln!(f, "    Spirit:  {}", self.spirit)?;
        writeln!(f, "    Aura:    {}", self.aura)
    }
}
