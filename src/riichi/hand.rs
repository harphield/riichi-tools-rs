use std::fmt;

use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;
use super::shanten::ShantenFinder;
use crate::riichi::riichi_error::RiichiError;

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

    /// Converts our tiles vector to an array of 34 counts, since riichi has 34 different tiles.
    /// TODO automatically remove open shapes, so it doesn't interfere with shanten calculation?
    pub fn get_34_array(&mut self) -> [u8; 34] {
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
    pub fn from_text(representation: &str) -> Result<Hand, RiichiError> {
        // let's read the hand from the back, because colors are written after the numbers
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
                match Tile::from_text(&rep[..]) {
                    Ok(tile) => {
                        tiles.push(Option::Some(tile));
                    },
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }

        tiles.sort();

        if tiles.len() >= 13 {
            return Result::Ok(Hand::new(tiles));
        }

        Err(RiichiError::new(100, "Couldn't parse hand representation."))
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

    /// Get shanten of this hand (and also set it if it's not calculated yet)
    pub fn shanten(&mut self) -> u8 {
        if self.shanten == 99 {
            match ShantenFinder::new().shanten(self) {
                Ok(shanten) => {
                    self.shanten = shanten;
                },
                Err(error) => ()
            }
        }

        self.shanten
    }

    /// Reset shanten to 99 when we change the hand somehow
    pub fn reset_shanten(&mut self) {
        self.shanten = 99;
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
        let hand = Hand::from_text(rep).unwrap();

        let rep2 = hand.to_string();
        assert_eq!(rep2, rep);
    }

    #[test]
    fn validation_ok() {
        let rep = "123m123p12345s22z";
        let mut hand = Hand::from_text(rep).unwrap();

        assert!(hand.validate());
    }

    #[test]
    fn validation_bad_5_same_tiles() {
        let rep = "123m123p11111s22z";
        let mut hand = Hand::from_text(rep).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn validation_bad_too_many_tiles() {
        let rep = "123456789m123456789p12345s22z";
        let mut hand = Hand::from_text(rep).unwrap();

        assert!(!hand.validate());
    }
}