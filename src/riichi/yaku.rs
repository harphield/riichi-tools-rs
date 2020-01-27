use crate::riichi::hand::Hand;

pub struct YakuFinder {

}

impl YakuFinder {
    pub fn new() -> YakuFinder {
        YakuFinder {
            ..Default::default()
        }
    }

    pub fn find(&self, hand: &Hand) {

    }
}

impl Default for YakuFinder {
    fn default() -> YakuFinder {
        YakuFinder {
        }
    }
}

////////////////

pub trait Yaku {
    fn is_in_hand(&self) -> bool;
    fn get_han(&self) -> u8;
}

////////////////

struct Pinfu<'a> {
    hand: &'a Hand
}

impl Pinfu<'_> {
    pub fn new(hand: &Hand) -> Pinfu {
        Pinfu {
            hand
        }
    }
}

impl Yaku for Pinfu {
    fn is_in_hand(&self) -> bool {
        // TODO check if there are no bonus fu in the hand
        false
    }

    fn get_han(&self) -> u8 {
        // always 1 han
        1
    }
}
