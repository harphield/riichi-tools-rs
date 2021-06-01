use crate::riichi::hand::Hand;

/// Let's find potential final hands from an incomplete hand
/// 1. get ukeire tiles
/// 2. switch tile for a better one
/// 3. check shanten - if not complete, do step 1 again
/// 4. if complete, get value of hand, save it somewhere
/// 5. finish all paths (do some pruning somehow?)

pub struct PotentialFinder {}

impl PotentialFinder {
    pub fn find(&self, hand: &Hand) -> Vec<Hand> {
        let mut final_hands = vec![];

        let ukeire = hand.find_shanten_improving_tiles(None);

        if ukeire.is_empty() {
            // finished
            return final_hands;
        }

        // TODO continue

        final_hands
    }
}
