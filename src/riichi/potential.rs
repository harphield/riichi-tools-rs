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

        if hand.get_shanten() == -1 {
            final_hands.push(hand.clone());

            return final_hands;
        }

        let ukeire = hand.find_shanten_improving_tiles(None);

        if ukeire.is_empty() {
            // finished
            return final_hands;
        }

        // TODO continue
        let mut hand = hand.clone();

        for (discard_tile_o, imp_tiles, _count) in ukeire.iter() {
            if let Some(discard_tile) = discard_tile_o {
                hand.remove_tile(discard_tile);
            }

            for (tile, _c) in imp_tiles {
                hand.add_tile(*tile);

                final_hands.append(&mut self.find(&hand));

                hand.remove_tile(tile);
            }

            if let Some(discard_tile) = discard_tile_o {
                hand.add_tile(*discard_tile);
            }
        }

        final_hands
    }
}

mod tests {
    use crate::riichi::hand::Hand;
    use crate::riichi::potential::PotentialFinder;

    #[test]
    fn find_potential() {
        let hand = Hand::from_text("237m13478s45699p", false).unwrap();
        let finder = PotentialFinder {};
        let hands = finder.find(&hand);

        let hands_strings: Vec<String> = hands.iter().map(|h| {
            h.to_string()
        }).collect();

        println!("{:#?}", hands_strings);

        assert!(hands.len() > 0);
    }
}
