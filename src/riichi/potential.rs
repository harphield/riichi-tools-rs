use crate::riichi::hand::Hand;
use crate::riichi::scores::Score;
use crate::riichi::table::Table;
use crate::riichi::yaku::Yaku;
use num_bigint::ToBigUint;
use num_integer::binomial;
use num_traits::cast::ToPrimitive;
use std::cmp::Ordering;

/// Let's find potential final hands from an incomplete hand
/// 1. get ukeire tiles
/// 2. switch tile for a better one
/// 3. check shanten - if not complete, do step 1 again
/// 4. if complete, get value of hand, save it somewhere
/// 5. finish all paths (do some pruning somehow?)

type PotentialList = Vec<(Hand, Option<(Vec<Yaku>, Score)>, f32)>;

pub struct PotentialFinder {}

impl PotentialFinder {
    pub fn find_potential(&self, table: &Table) -> Option<PotentialList> {
        let mut table = table.clone();

        // just don't
        if table.get_my_hand().get_shanten() > 3 {
            return None;
        }

        let mut results = self.find(&mut table, 0f32, 0);

        // sort results by value + speed
        // TODO speed
        results.sort_by(|a, b| {
            if a.1.is_none() && b.1.is_none() {
                return Ordering::Equal;
            }

            if a.1.is_none() {
                return Ordering::Less;
            }

            if b.1.is_none() {
                return Ordering::Greater;
            }

            let a_score = a.1.as_ref().unwrap();
            let b_score = b.1.as_ref().unwrap();

            if a_score.1.total_points() < b_score.1.total_points() {
                return Ordering::Less;
            }

            if a_score.1.total_points() > b_score.1.total_points() {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        results.reverse();

        Some(results)
    }

    /// Recursive search through improving tiles until tenpai.
    fn find(&self, mut table: &mut Table, chances: f32, depth: u8) -> PotentialList {
        let mut final_hands = vec![];

        let hand = table.get_my_hand().to_owned();

        if hand.get_shanten() == -1 {
            let yaku = table.yaku();
            final_hands.push((hand, yaku, chances / depth as f32));

            return final_hands;
        }

        let ukeire = hand.find_shanten_improving_tiles(Some(&table.get_visible_tiles()));

        if ukeire.is_empty() {
            // finished
            return final_hands;
        }

        // save table data
        let old_visible_tiles = *table.get_visible_tiles();
        let old_remaining_tiles = table.get_tiles_remaining();

        for (discard_tile_o, imp_tiles, _count) in ukeire.iter() {
            if let Some(discard_tile) = discard_tile_o {
                table.my_hand_discard(&discard_tile);
            }

            for (tile, count) in imp_tiles {
                table.my_hand_draw(&tile);

                // table.set_my_hand(hand.clone());
                let draw_chance = self.draw_chance(
                    *count,
                    table.get_tiles_remaining(),
                    table.get_visible_tiles_count(),
                );
                final_hands.append(&mut self.find(&mut table, chances + draw_chance, depth + 1));

                table.my_hand_discard(&tile);

                // restore table data
                table.set_visible_tiles(&old_visible_tiles);
                table.set_tiles_remaining(old_remaining_tiles);
            }

            if let Some(discard_tile) = discard_tile_o {
                table.my_hand_draw(&discard_tile);

                table.set_visible_tiles(&old_visible_tiles);
                table.set_tiles_remaining(old_remaining_tiles);
            }
        }

        final_hands
    }

    /// count = number of tiles that I can find in the wall (based on visible tiles, so if you see 2, you can find 2)
    /// remaining_tiles = how many tiles are left in the wall
    /// visible_tiles = number of visible tiles
    ///
    /// We are doing https://en.wikipedia.org/wiki/Hypergeometric_distribution for the probability
    /// So the formula is:
    /// ((<count> choose <need>) * (<invisible_tiles> - <count> choose <remaining_draws> - <need>)) / (<invisible_tiles> choose <count>)
    fn draw_chance(&self, count: u8, remaining_tiles: u8, visible_tiles: u8) -> f32 {
        // println!("count: {}, remaining tiles: {}, visible tiles: {}", count, remaining_tiles, visible_tiles);

        let count = count.to_biguint().unwrap();
        let need = 1.to_biguint().unwrap(); // let's say we need 1

        let remaining_draws = ((remaining_tiles as f32 / 4.0f32).floor())
            .to_biguint()
            .unwrap();
        let invisible_tiles = (136 - &visible_tiles).to_biguint().unwrap();

        (binomial(count.clone(), need.clone())
            * binomial(
                invisible_tiles.clone() - count,
                remaining_draws.clone() - need,
            ))
        .to_f32()
        .unwrap()
            / binomial(invisible_tiles, remaining_draws).to_f32().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    fn test_hand(hand: &str) -> Option<PotentialList> {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from(hand));
        map.insert("dora".to_string(), Value::from("6m"));

        let finder = PotentialFinder {};
        let hands = finder.find_potential(&mut Table::from_map(&map).unwrap());

        match hands {
            None => None,
            Some(hands) => {
                let hands_strings: Vec<String> = hands
                    .iter()
                    .map(|(h, o, p)| {
                        format!(
                            "{}{} {}",
                            h.to_string(),
                            match o {
                                None => "".to_string(),
                                Some(yakus) => {
                                    if yakus.1.han == 0 {
                                        " (no yaku)".to_string()
                                    } else {
                                        format!(
                                            " ({} {} : {})",
                                            yakus.1.han,
                                            yakus.1.fu,
                                            yakus.1.total_points()
                                        )
                                    }
                                }
                            },
                            p
                        )
                    })
                    .collect();

                println!("{:#?}", hands_strings);

                Some(hands)
            }
        }
    }

    #[test]
    fn find_potential_2_shanten() {
        let hands = test_hand("347m13478s34599p");
        assert!(hands.unwrap().len() > 0);
    }

    #[test]
    fn find_potential_3_shanten() {
        let hands = test_hand("123459m378p39s26z5p");
        assert!(hands.unwrap().len() > 0);
    }

    #[test]
    fn find_potential_4_shanten() {
        let hands = test_hand("277m1459p699s346z6m");
        assert!(hands.is_none());
    }

    #[test]
    fn draw_chance() {
        let p = PotentialFinder {};
        let res = p.draw_chance(4, 69, 0);

        println!("{}", res);

        let part1 = binomial(4, 1);
        let part2 = binomial(67 - 4, 5 - 1);
        let part3 = binomial(67, 5);

        println!(
            "{} {} {}, {}, {}",
            part1,
            part2,
            part3,
            part1 * part2,
            (part1 * part2).to_f64().unwrap() / part3.to_f64().unwrap()
        );

        let test = ((binomial(4, 1) * binomial(67 - 4, 5 - 1)) / binomial(67, 5))
            .to_f64()
            .unwrap();

        println!("{}", test);

        assert!(res > 0f32);
    }
}
