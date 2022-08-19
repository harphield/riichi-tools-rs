use super::hand::Hand;
use super::tile::Tile;
use crate::riichi::riichi_error::RiichiError;

pub struct ShantenFinder {
    pairs: i8,
    complete_melds: i8,
    incomplete_melds: i8,
    isolated_tiles: i8,
    hand_count: usize,
    min_found: i8,
}

impl ShantenFinder {
    pub fn new() -> ShantenFinder {
        ShantenFinder {
            ..Default::default()
        }
    }

    pub fn shanten(&mut self, hand: &Hand) -> Result<i8, RiichiError> {
        if !hand.validate() {
            return Err(RiichiError::new(101, "Invalid hand"));
        }
        self.hand_count = hand.count_tiles();

        let mut array_34 = hand.get_34_array(true);

        let mut kokushi_shanten = 99;
        let mut chiitoi_shanten = 99;

        // add kans to completed melds
        self.complete_melds += hand.get_closed_kans() as i8;
        self.complete_melds += hand.get_open_shapes().len() as i8;

        if hand.is_closed() {
            kokushi_shanten = self.kokushi_shanten(&array_34);
            chiitoi_shanten = self.chiitoitsu_shanten(&array_34);
        }

        let shanten = self.analyze(&mut array_34, 0);

        let shantens = [kokushi_shanten, chiitoi_shanten, shanten];

        Ok(*shantens.iter().min().unwrap())
    }

    /// Gets the hand's shanten to kokushi musou.
    fn kokushi_shanten(&self, array_34: &[u8; 34]) -> i8 {
        let mut shanten: i8 = 13;
        let mut pair_found = false;

        for (i, count) in array_34.iter().enumerate() {
            if [1, 9, 10, 18, 19, 27].contains(&(i + 1)) || (i + 1) >= 28 {
                // we only need 1 of each here + pair
                if *count > 1 {
                    if !pair_found {
                        shanten -= 2; // I'm keeping two of these as a pair
                        pair_found = true;
                    } else {
                        shanten -= 1;
                    }
                } else if *count > 0 {
                    shanten -= 1;
                }
            }
        }

        shanten
    }

    /// Gets the hand's shanten to chiitoitsu
    fn chiitoitsu_shanten(&self, array_34: &[u8; 34]) -> i8 {
        let mut pairs = 0;
        for count in array_34.iter() {
            if *count >= 2 {
                pairs += 1;
            }
        }

        6 - pairs // how many pairs am I missing to tenpai?
    }

    /// Recursive method to traverse a hand, removing shapes until only tiles that have to be
    /// discarded and changed remain - that is the shanten of a hand.
    fn analyze(&mut self, array_34: &mut [u8; 34], depth: usize) -> i8 {
        if (self.hand_count == 13 && self.min_found <= 0)
            || (self.hand_count == 14 && self.min_found < 0)
        {
            // println!("done {}", self.min_found);
            return 99;
        }

        if depth >= 34 {
            return self.final_calculations();
        }

        if array_34[depth] == 4 && (self.complete_melds + self.incomplete_melds < 4) {
            // use 3 as pon, leave one behind and try again
            self.add_set(array_34, depth);
            self.analyze(array_34, depth);
            self.remove_set(array_34, depth);

            // use 2 as pair
            self.add_pair(array_34, depth);
            self.analyze(array_34, depth);
            self.remove_pair(array_34, depth);

        // use 1 as isolated tile
        } else if array_34[depth] == 3 && (self.complete_melds + self.incomplete_melds < 4) {
            self.add_set(array_34, depth);
            self.analyze(array_34, depth + 1);
            self.remove_set(array_34, depth);

            self.add_pair(array_34, depth);
            self.analyze(array_34, depth);
            self.remove_pair(array_34, depth);
        } else if array_34[depth] == 2 {
            // if we don't have a pair yet, this will be our pair
            self.add_pair(array_34, depth);
            self.analyze(array_34, depth + 1);
            self.remove_pair(array_34, depth);
        }

        if array_34[depth] > 0 {
            if self.complete_melds + self.incomplete_melds < 4 {
                // use 1, check for a complete meld (3 tiles)
                let mut done = self.add_complete_meld(array_34, depth);

                if done {
                    if array_34[depth] > 0 {
                        self.analyze(array_34, depth);
                    } else {
                        self.analyze(array_34, depth + 1);
                    }

                    self.remove_complete_meld(array_34, depth);
                }

                // use 1, check for kanchan & penchan & ryanmen shapes (2 tiles)
                done = self.add_incomplete_meld_1(array_34, depth);
                if done {
                    if array_34[depth] > 0 {
                        self.analyze(array_34, depth);
                    } else {
                        self.analyze(array_34, depth + 1);
                    }

                    self.remove_incomplete_meld_1(array_34, depth);
                }

                done = self.add_incomplete_meld_2(array_34, depth);
                if done {
                    if array_34[depth] > 0 {
                        self.analyze(array_34, depth);
                    } else {
                        self.analyze(array_34, depth + 1);
                    }

                    self.remove_incomplete_meld_2(array_34, depth);
                }
            }

            self.add_isolated_tile(array_34, depth);
            self.analyze(array_34, depth + 1);
            self.remove_isolated_tile(array_34, depth);
        } else {
            self.analyze(array_34, depth + 1);
        }

        self.min_found
    }

    fn final_calculations(&mut self) -> i8 {
        // TODO add open melds and closed kans to complete_melds

        let mut over = 0;
        if self.complete_melds + self.incomplete_melds + self.pairs > 5 {
            over = 5 - self.complete_melds + self.incomplete_melds + self.pairs;
        }

        let s = (8 - self.complete_melds * 2 - self.incomplete_melds - self.pairs + over) as i8;

        if s < self.min_found {
            // println!("{} {} {} {}", self.complete_melds, self.incomplete_melds, self.pairs, over);
            self.min_found = s;
        }

        s
    }

    fn add_set(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] -= 3;
        self.complete_melds += 1;
    }

    fn remove_set(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] += 3;
        self.complete_melds -= 1;
    }

    fn add_pair(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] -= 2;

        // println!("{}", array_34[depth]);

        self.pairs += 1;
    }

    fn remove_pair(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] += 2;
        self.pairs -= 1;
    }

    fn add_isolated_tile(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] -= 1;
        self.isolated_tiles += 1;
    }

    fn remove_isolated_tile(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] += 1;
        self.isolated_tiles -= 1;
    }

    fn add_complete_meld(&mut self, array_34: &mut [u8; 34], depth: usize) -> bool {
        // let tile;
        // match Tile::from_id((depth + 1) as u8) {
        //     Ok(t) => {
        //         tile = t;
        //     }
        //     Err(_) => {
        //         return false;
        //     }
        // };

        let tile =
            match Tile::from_id((depth + 1) as u8) {
                Ok(t) => {
                    t
                }
                Err(_) => {
                    return false;
                }
            };

        let second = tile.next(false);

        if let Some(t2) = second {
            if array_34[(t2.get_id() - 1) as usize] > 0 {
                let third = t2.next(false);
                if let Some(t3) = third {
                    if array_34[(t3.get_id() - 1) as usize] > 0 {
                        // found a complete meld!
                        array_34[depth] -= 1;
                        array_34[depth + 1] -= 1;
                        array_34[depth + 2] -= 1;
                        self.complete_melds += 1;

                        return true;
                    }
                }
            }
        }

        false
    }

    fn remove_complete_meld(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] += 1;
        array_34[depth + 1] += 1;
        array_34[depth + 2] += 1;
        self.complete_melds -= 1;
    }

    /// ryanmen or penchan wait
    fn add_incomplete_meld_1(&mut self, array_34: &mut [u8; 34], depth: usize) -> bool {
        // let tile;
        // match Tile::from_id((depth + 1) as u8) {
        //     Ok(t) => tile = t,
        //     Err(_) => return false,
        // }

        // how does this work? Clippy pls
        let tile =
            match Tile::from_id((depth + 1) as u8) {
                Ok(t) => t,
                Err(_) => return false,
            };

        let second = tile.next(false);

        if let Some(t2) = second {
            if array_34[(t2.get_id() - 1) as usize] > 0 {
                // found an incomplete meld!
                array_34[depth] -= 1;
                array_34[depth + 1] -= 1;
                self.incomplete_melds += 1;

                return true;
            }
        }

        false
    }

    fn remove_incomplete_meld_1(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] += 1;
        array_34[depth + 1] += 1;
        self.incomplete_melds -= 1;
    }

    /// kanchan wait
    fn add_incomplete_meld_2(&mut self, array_34: &mut [u8; 34], depth: usize) -> bool {
        // let tile;
        // match Tile::from_id((depth + 1) as u8) {
        //     Ok(t) => tile = t,
        //     Err(_) => return false,
        // }

        // clippy told me to do it like this, but I'm not sure about it. It compiles, but...
        let tile =
            match Tile::from_id((depth + 1) as u8) {
                Ok(t) => t,
                Err(_) => return false,
            };
        let second = tile.next(false);

        if let Some(t2) = second {
            let third = t2.next(false);
            if let Some(t3) = third {
                if array_34[(t3.get_id() - 1) as usize] > 0 {
                    // found an incomplete meld!
                    array_34[depth] -= 1;
                    array_34[depth + 2] -= 1;
                    self.incomplete_melds += 1;
                    return true;
                }
            }
        }

        false
    }

    fn remove_incomplete_meld_2(&mut self, array_34: &mut [u8; 34], depth: usize) {
        array_34[depth] += 1;
        array_34[depth + 2] += 1;
        self.incomplete_melds -= 1;
    }
}

impl Default for ShantenFinder {
    fn default() -> ShantenFinder {
        ShantenFinder {
            pairs: 0,
            complete_melds: 0,
            incomplete_melds: 0,
            isolated_tiles: 0,
            hand_count: 0,
            min_found: 99,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riichi::shapes::OpenShape;

    #[test]
    fn kokushi_tenpai_13_waits() {
        let hand = Hand::from_text("19m19s19p1234567z", false).unwrap();
        let array34 = hand.get_34_array(true);

        let mut shanten_finder = ShantenFinder::new();
        shanten_finder.hand_count = 13;
        let shanten = shanten_finder.kokushi_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn kokushi_complete() {
        let hand = Hand::from_text("19m19s19p11234567z", false).unwrap();
        let array34 = hand.get_34_array(true);

        let mut shanten_finder = ShantenFinder::new();
        shanten_finder.hand_count = 14;
        let shanten = shanten_finder.kokushi_shanten(&array34);

        assert_eq!(shanten, -1);
    }

    #[test]
    fn kokushi_tenpai_1_wait() {
        let hand = Hand::from_text("19m19s19p1234566z", false).unwrap();
        let array34 = hand.get_34_array(true);

        let mut shanten_finder = ShantenFinder::new();
        shanten_finder.hand_count = 13;
        let shanten = shanten_finder.kokushi_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn kokushi_iishanten() {
        let hand = Hand::from_text("129m19s19p123456z", false).unwrap();
        let array34 = hand.get_34_array(true);

        let mut shanten_finder = ShantenFinder::new();
        shanten_finder.hand_count = 13;
        let shanten = shanten_finder.kokushi_shanten(&array34);

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_tenpai() {
        let hand = Hand::from_text("1133557799p22s3z", false).unwrap();
        let array34 = hand.get_34_array(true);

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn chiitoitsu_iishanten() {
        let hand = Hand::from_text("113355779p22s34z", false).unwrap();
        let array34 = hand.get_34_array(true);

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_iishanten_with_shanten() {
        let mut hand = Hand::from_text("113355779p22s34z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_6shanten() {
        let hand = Hand::from_text("123456789m123p1s", false).unwrap();
        let array34 = hand.get_34_array(true);

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 6);
    }

    #[test]
    fn pinfu_tenpai() {
        let mut hand = Hand::from_text("123456789m23p11s", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn pinfu_2_shanten() {
        let mut hand = Hand::from_text("1235689m23p11s14z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 2);
    }

    #[test]
    fn chinitsu_tenpai() {
        let mut hand = Hand::from_text("1112344478999m", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn chinitsu_iishanten() {
        let mut hand = Hand::from_text("1112224457889m", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 1);
    }

    #[test]
    fn honors_suuankou_tenpai() {
        let mut hand = Hand::from_text("1112223334445z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn nonhonors_suuankou_tenpai() {
        let mut hand = Hand::from_text("111m222s333444p5s", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn weird_hand_shanten() {
        let mut hand = Hand::from_text("37m13478s45699p1z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 3);
    }

    #[test]
    fn with_14_tiles_iishanten() {
        let mut hand = Hand::from_text("237m45699p123478s", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 1);
    }

    #[test]
    fn with_14_tiles_ryanshanten() {
        let mut hand = Hand::from_text("2357m13478s45699p", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 2);
    }

    #[test]
    fn with_14_tiles_ryanshanten_2() {
        let mut hand = Hand::from_text("2377m13478s45699p", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 2);
    }

    #[test]
    fn with_14_tiles_ryanshanten_3() {
        let mut hand = Hand::from_text("1234s123p999m3456z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 2);
    }

    #[test]
    fn with_14_tiles_tenpai() {
        let mut hand = Hand::from_text("123456789m239p11s", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn with_14_tiles_tenpai_no_pair() {
        let mut hand = Hand::from_text("12345m567s111222z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn with_14_tiles_tenpai_no_pair_2() {
        let mut hand = Hand::from_text("123456789p12345m", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn with_14_tiles_complete() {
        let mut hand = Hand::from_text("123456789m234p11s", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, -1);
    }

    #[test]
    fn honors_shanpon_tenpai() {
        let mut hand = Hand::from_text("1112223334455z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn with_14_two_pairs_and_stuff() {
        let mut hand = Hand::from_text("23m13478s45699p11z", false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 2);
    }

    #[test]
    fn with_14_open_hand_chi_tenpai() {
        let rep = "123m123p12345s222z";
        let mut hand = Hand::from_text(rep, false).unwrap();

        hand.add_open_shape(&OpenShape::Chi([
            Tile::from_text("1m").unwrap(),
            Tile::from_text("2m").unwrap(),
            Tile::from_text("3m").unwrap(),
        ]));

        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn with_14_open_hand_chi_tenpai_parsed() {
        let rep = "123459m11p(123s0)(345s1)";
        let mut hand = Hand::from_text(rep, false).unwrap();
        let shanten = hand.shanten();

        assert_eq!(shanten, 0);
    }
}
