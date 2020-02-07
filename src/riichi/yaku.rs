use crate::riichi::hand::Hand;
use crate::riichi::shape_finder::ShapeFinder;

pub struct YakuFinder {

}

impl YakuFinder {
    pub fn new() -> YakuFinder {
        YakuFinder {
            ..Default::default()
        }
    }

    pub fn find(&self, hand: &mut Hand) {
        let mut sf = ShapeFinder::new();
        let variants = sf.find(hand);
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
    fn is_in_hand(&self, hand: &mut Hand) -> bool;
    fn get_han(&self) -> u8;
}

////////////////

struct Pinfu {

}

impl Pinfu {
    pub fn new(hand: &Hand) -> Pinfu {
        Pinfu {

        }
    }
}

impl Yaku for Pinfu {
    /// 4x shuntsu, 1x valueless pair
    fn is_in_hand(&self, hand: &mut Hand) -> bool {
        let array_34 = hand.get_34_array();
        let mut has_pair = false;

        for (i, count) in array_34.iter().enumerate() {
            // can't have any value honors
            // TODO table state with prevalent & seat wind
            if i >= 27 {
                return false;
            }
        }

        true
    }

    fn get_han(&self) -> u8 {
        // always 1 han
        1
    }
}
