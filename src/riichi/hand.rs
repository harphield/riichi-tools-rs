use std::fmt;

use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;
use super::shapes::Shape;

#[derive(Debug)]
pub struct Hand {
    /// a hand consists of 13 tiles + 1 drawn tile
    /// it can also have kan, which are groups of 4 tiles that behave as 3 tiles
    /// so we should have a vector with 13 100% present tiles and 5 optional (4 from possible kans and 1 possible draw)
    tiles: Vec<Option<Tile>>,
    shanten: u8
}

impl Hand {
    pub fn new(tiles : Vec<Option<Tile>>) -> Hand {
        Hand {
            tiles,
            ..Default::default()
        }
    }

    /// 
    /// 
    pub fn shanten(&self) {
        let mut shanten = 8; // max shanten ever ???
        let tiles_count = self.tiles.len();

        if tiles_count < 13 {
            panic!("Invalid hand");
        }

        let mut array_34 = self.to_34_array();

        self.analyze(&array_34, 0);
    }

    fn analyze(&self, mut array_34 : &[u8; 34], depth : usize) {

    }

    fn find_complete_shapes(&self, array_34 : &[u8; 34], depth : usize) {

    }

    fn to_34_array(&self) -> [u8; 34] {
        let mut out = [0; 34];

        for tile in self.tiles.iter() {
            if let Option::Some(t) = tile {
                out[(t.to_id() - 1) as usize] += 1;
            }
        }

        out
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
        let mut tiles : Vec<Option<Tile>> = Vec::new();

        let mut color : char = 'x';
        let mut rep : String;
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
}

impl Default for Hand {
    fn default() -> Hand {
        Hand {
            tiles: vec!(),
            shanten: 99
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for tile in self.tiles.iter() {
            match &tile {
                Option::Some(some_tile) => out.push_str(&some_tile.to_string()[..]),
                Option::None => ()
            }
        }
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_text_hand() {
        let rep = "123m123s12345p22z";
        let hand = Hand::from_text(rep);

        
    }

    #[test]
    fn kokushi_iishanten() {
        let hand = Hand::from_text("19m19s19p1234567z");

        let shanten = hand.shanten();

        assert_eq!(hand.shanten, 1);
    }
}