use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;
use super::hand::Hand;

pub struct ShantenFinder {
    pairs: u8,
    triplets: u8,
    complete_melds: u8,
    incomplete_melds: u8,
    isolated_tiles: u8
}

impl ShantenFinder {
    pub fn new() -> ShantenFinder {
        ShantenFinder {
            ..Default::default()
        }
    }

    pub fn shanten(&mut self, hand : &mut Hand) -> u8 {
        if !hand.validate() {
            panic!("Invalid hand");
        }
        let mut shanten: u8 = 8; // max shanten ever ???
        let mut array_34 = hand.get_34_array();
        let kokushi_shanten = self.kokushi_shanten(&array_34);
        let chiitoi_shanten = self.chiitoitsu_shanten(&array_34);

        shanten = self.analyze(&mut array_34, 0);

        let shantens = [kokushi_shanten, chiitoi_shanten, shanten];

        *shantens.iter().min().unwrap()
    }

    /// Gets the hand's shanten to kokushi musou.
    fn kokushi_shanten(&self, mut array_34: &[u8; 34]) -> u8 {
        let mut shanten = 0;
        let mut pair_found = false;

        for (i, count) in array_34.iter().enumerate() {
            if ([1, 9, 10, 18, 19, 27].contains(&(i + 1)) || (i + 1) >= 28) && *count == 1 {
                // we only need 1 of each here + pair
                if *count > 1 {
                    if pair_found {
                        shanten += count - 1; // I'm only keeping one of them, the others need to be discarded
                    } else {
                        shanten += count - 2; // I'm keeping two of these as a pair
                        pair_found = true;
                    }
                }
            } else {
                shanten += *count;
            }
        }

        shanten
    }

    /// Gets the hand's shanten to chiitoitsu
    fn chiitoitsu_shanten(&self, mut array_34: &[u8; 34]) -> u8 {
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
    fn analyze(&self, array_34: &mut [u8; 34], depth: usize) -> u8 {
        let mut shanten = 100;

        // got 4 tiles
        if array_34[depth] == 4 {
            // use 3 as pon, leave one behind and try again
            array_34[depth] -= 3;
            self.triplets += 1;
            shanten = self.analyze(array_34, depth);
            self.triplets -= 1;
            array_34[depth] += 3;

            // use 2 as pair
            array_34[depth] -= 2;
            self.pairs += 1;
            shanten = self.analyze(array_34, depth);
            self.pairs -= 1;
            array_34[depth] += 2;

            // use 1, check for a complete meld (3 tiles)

            // use 1, check for kanchan & penchan & ryanmen shapes (2 tiles)

            // use 1 as isolated tile
        } else if array_34[depth] == 3 {
            array_34[depth] -= 3;
            shanten = self.analyze(array_34, depth + 1);
            array_34[depth] += 3;
        } else if array_34[depth] == 2 {
            // if we don't have a pair yet, this will be our pair
        }

        shanten
    }
}

impl Default for ShantenFinder {
    fn default() -> ShantenFinder {
        ShantenFinder {
            pairs: 0,
            triplets: 0,
            complete_melds: 0,
            incomplete_melds: 0,
            isolated_tiles: 0
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn kokushi_tenpai() {
        let mut hand = Hand::from_text("19m19s19p1234567z");
        let array34 = hand.get_34_array();

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.kokushi_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn kokushi_iishanten() {
        let mut hand = Hand::from_text("18m19s19p1234567z");
        let array34 = hand.get_34_array();

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.kokushi_shanten(&array34);

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_tenpai() {
        let mut hand = Hand::from_text("1133557799p22s3z");
        let array34 = hand.get_34_array();

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn chiitoitsu_iishanten() {
        let mut hand = Hand::from_text("113355779p22s34z");
        let array34 = hand.get_34_array();

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_6shanten() {
        let mut hand = Hand::from_text("123456789m123p1s");
        let array34 = hand.get_34_array();

        let shanten_finder = ShantenFinder::new();
        let shanten = shanten_finder.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 6);
    }
}