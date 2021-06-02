use crate::riichi::hand::Hand;
use crate::riichi::table::Table;
use crate::riichi::yaku::Yaku;
use crate::riichi::scores::Score;

/// Let's find potential final hands from an incomplete hand
/// 1. get ukeire tiles
/// 2. switch tile for a better one
/// 3. check shanten - if not complete, do step 1 again
/// 4. if complete, get value of hand, save it somewhere
/// 5. finish all paths (do some pruning somehow?)

pub struct PotentialFinder {}

impl PotentialFinder {
    pub fn find(&self, mut table: &mut Table) -> Vec<(Hand, Option<(Vec<Yaku>, Score)>)> {
        let mut final_hands = vec![];

        let mut hand = table.get_my_hand().to_owned();

        if hand.get_shanten() == -1 {
            let yaku = table.yaku();
            final_hands.push((hand.clone(), yaku));

            return final_hands;
        }

        let ukeire = hand.find_shanten_improving_tiles(None);

        if ukeire.is_empty() {
            // finished
            return final_hands;
        }

        for (discard_tile_o, imp_tiles, _count) in ukeire.iter() {
            if let Some(discard_tile) = discard_tile_o {
                hand.remove_tile(discard_tile);
            }

            for (tile, _c) in imp_tiles {
                hand.add_tile(*tile);

                table.set_my_hand(hand.clone());
                final_hands.append(&mut self.find(&mut table));

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
    use crate::riichi::potential::PotentialFinder;
    use crate::riichi::table::Table;
    use serde_json::{Map, Value};

    #[test]
    fn find_potential_2_shanten() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("237m13478s45699p"));

        let finder = PotentialFinder {};
        let hands = finder.find(&mut Table::from_map(&map).unwrap());

        let hands_strings: Vec<String> = hands.iter().map(|(h, _o)| {
            h.to_string()
        }).collect();

        println!("{:#?}", hands_strings);

        assert!(hands.len() > 0);
    }

    #[test]
    fn find_potential_3_shanten() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("123459m378p39s26z5p"));

        let finder = PotentialFinder {};
        let hands = finder.find(&mut Table::from_map(&map).unwrap());

        let hands_strings: Vec<String> = hands.iter().map(|(h, _o)| {
            h.to_string()
        }).collect();

        println!("{:#?}", hands_strings);

        assert!(hands.len() > 0);
    }
}
