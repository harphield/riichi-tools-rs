use std::fmt;

use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;
use super::shapes::Shape;

#[derive(Debug)]
pub struct Hand {
    // a hand consists of 13 tiles + 1 drawn tile
    // it can also have kan, which are groups of 4 tiles that behave as 3 tiles
    // so we should have a vector with 13 100% present tiles and 5 optional (4 from possible kans and 1 possible draw)
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

    pub fn from_text(representation: &str) -> Hand {
        if representation.len() % 2 != 0 {
            panic!("String representation of a hand must be even length");
        }

        let mut tiles : Vec<Option<Tile>> = Vec::with_capacity(representation.len());
        let mut iter = representation.chars();
        let mut pos = 0;
        let mut len;

        while pos < representation.len() {
            len = 0;
            for ch in iter.by_ref().take(2) {
                len += ch.len_utf8();
            }
            let tile_string = &representation[pos..pos + len];

            let tile = Tile::from_text(tile_string);

            tiles.push(Option::Some(tile));
            pos += len;
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
    fn kokushi_iishanten() {
        let hand = Hand::from_text("m1m9s1s9p1p9z1z2z3z4z5z6z7");

        let shanten = hand.shanten();

        assert_eq!(hand.shanten, 1);
    }
}