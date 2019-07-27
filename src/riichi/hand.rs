use std::fmt;

use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;
use super::shapes::Shape;

pub struct Hand {
    /// a hand consists of 13 tiles + 1 drawn tile
    /// it can also have kan, which are groups of 4 tiles that behave as 3 tiles
    /// so we should have a vector with 13 100% present tiles and 5 optional (4 from possible kans and 1 possible draw)
    tiles: Vec<Option<Tile>>,
    array_34: Option<[u8; 34]>,
    shanten: u8,
}

impl Hand {
    pub fn new(tiles: Vec<Option<Tile>>) -> Hand {
        Hand {
            tiles,
            ..Default::default()
        }
    }

    /// Checks the hand for invalid things (wrong number of tiles, > 4 same tiles...)
    pub fn validate(&mut self) -> bool {
        let mut tile_count = 0;
        let array34 = self.get_34_array();

        for count in array34.iter() {
            tile_count += *count;
            if *count > 4 {
                return false;
            }
        }

        // 13 tiles + 5 optional from kans & draw
        if tile_count > 18 {
            return false;
        }

        true
    }

    /// 
    /// 
    pub fn shanten(&mut self) -> u8 {
        if !self.validate() {
            panic!("Invalid hand");
        }
        let mut shanten: u8 = 8; // max shanten ever ???
        let mut array_34 = self.get_34_array();
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

    fn analyze(&self, array_34: &mut [u8; 34], depth: usize) -> u8 {
        let mut shanten = 100;

        // got 4 tiles
        if array_34[depth] == 4 {
            // use 3 as pon, leave one behind and try again
            array_34[depth] -= 3;
            shanten = self.analyze(array_34, depth);
            array_34[depth] += 3;
        } else if array_34[depth] == 3 {
            array_34[depth] -= 3;
            shanten = self.analyze(array_34, depth + 1);
            array_34[depth] += 3;
        } else if array_34[depth] == 2 {

        }

        shanten
    }

    fn find_complete_shapes(&self, array_34: &[u8; 34], depth: usize) {}

    /// Converts our tiles vector to an array of 34 counts, since riichi has 34 different tiles.
    /// TODO automatically remove open shapes, so it doesn't interfere with shanten calculation?
    fn get_34_array(&mut self) -> [u8; 34] {
        match self.array_34 {
            Some(array_34) => return array_34,
            None => {
                let mut array_34 = [0; 34];
                for tile in self.tiles.iter() {
                    if let Option::Some(t) = tile {
                        array_34[(t.to_id() - 1) as usize] += 1;
                    }
                }
                self.array_34 = Some(array_34);
                array_34
            }
        }
    }

    /// TODO
    pub fn random_hand(count: u8) -> Hand {
        if count < 13 || count > 14 {
            panic!("Only 13 or 14 tile hands allowed");
        } else {
            Hand::new(vec!(Option::Some(Tile::new(TileType::Number(1, TileColor::Manzu)))))
        }
    }

    /// Parses a hand from its text representation.
    pub fn from_text(representation: &str) -> Hand {
        let iter = representation.chars().rev();
        let mut tiles: Vec<Option<Tile>> = Vec::new();

        let mut color: char = 'x';
        let mut rep: String;
        for ch in iter {
            if ch.is_alphabetic() {
                // type
                color = ch;
            }

            if color != 'x' && ch.is_numeric() {
                // tile value
                rep = String::from("");
                rep.push(color);
                rep.push(ch);
                tiles.push(Option::Some(Tile::from_text(&rep[..])))
            }
        }

        tiles.sort();

        if tiles.len() >= 13 {
            return Hand::new(tiles);
        }

        panic!("Couldn't parse hand representation.");
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut color = 'x';

        for tile in self.tiles.iter() {
            match &tile {
                Option::Some(some_tile) => {
                    if color != some_tile.get_type_char() {
                        if color != 'x' {
                            out.push_str(&color.to_string()[..]);
                        }
                        color = some_tile.get_type_char();
                    }

                    out.push_str(&some_tile.get_value().to_string()[..]);
                }
                Option::None => ()
            }
        }

        out.push_str(&color.to_string()[..]);

        out
    }
}

impl Default for Hand {
    fn default() -> Hand {
        Hand {
            tiles: vec!(),
            array_34: None,
            shanten: 99,
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_text_hand() {
        let rep = "123m123p12345s22z";
        let hand = Hand::from_text(rep);

        let rep2 = hand.to_string();
        assert_eq!(rep2, rep);
    }

    #[test]
    fn validation_ok() {
        let rep = "123m123p12345s22z";
        let mut hand = Hand::from_text(rep);

        assert!(hand.validate());
    }

    #[test]
    fn validation_bad_5_same_tiles() {
        let rep = "123m123p11111s22z";
        let mut hand = Hand::from_text(rep);

        assert!(!hand.validate());
    }

    #[test]
    fn validation_bad_too_many_tiles() {
        let rep = "123456789m123456789p12345s22z";
        let mut hand = Hand::from_text(rep);

        assert!(!hand.validate());
    }

    #[test]
    fn kokushi_tenpai() {
        let mut hand = Hand::from_text("19m19s19p1234567z");
        let array34 = hand.get_34_array();

        let shanten = hand.kokushi_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn kokushi_iishanten() {
        let mut hand = Hand::from_text("18m19s19p1234567z");
        let array34 = hand.get_34_array();

        let shanten = hand.kokushi_shanten(&array34);

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_tenpai() {
        let mut hand = Hand::from_text("1133557799p22s3z");
        let array34 = hand.get_34_array();

        let shanten = hand.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 0);
    }

    #[test]
    fn chiitoitsu_iishanten() {
        let mut hand = Hand::from_text("113355779p22s34z");
        let array34 = hand.get_34_array();

        let shanten = hand.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 1);
    }

    #[test]
    fn chiitoitsu_6shanten() {
        let mut hand = Hand::from_text("123456789m123p1s");
        let array34 = hand.get_34_array();

        let shanten = hand.chiitoitsu_shanten(&array34);

        assert_eq!(shanten, 6);
    }
}