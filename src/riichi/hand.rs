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
    tiles: Vec<Option<Tile>>
}

impl Hand {
    /// We need to identify "free" tiles in a hand - tiles that don't fit into any shape and need to be discarded / upgraded to
    /// upgrade our hand.
    pub fn shanten(&self) -> u8 {
        let mut shanten = 8; // max shanten ever
        let tiles_count = self.tiles.len();

        if tiles_count < 13 {
            panic!("Invalid hand");
        }

        let mut count = 0;
        
        let mut index = 0;

        // TODO:
        // 1. find all shapes in the hand
        // 2. make all possible valid hands from the found shapes
        // 3. which hand has the lowest shanten?

        for tile in self.tiles.iter() {
            
        }

        while index < tiles_count {
            // this returns an Option<Option<&<I as SliceIndex<[T]>>::Output> !!
            // TODO rewrite this
            let tile = self.tiles.get(index);

            match &tile {
                Option::Some(t) => {
                    count += 1;
                    if index + 1 < tiles_count {
                        let mut is_in_shape_with_next = false;
                        let mut is_in_shape_with_next_plus_one = false;

                        let x = t.unwrap();

                        // check if the next tile can be used in a shape with this one
                        if let Some(t2) = self.tiles.get(index + 1) {
                            is_in_shape_with_next = Shape::is_in_shape(t.unwrap().to_id(), t2.unwrap().to_id());
                        }

                        // and the next
                        if index + 2 < tiles_count {                        
                            if let Some(t2) = self.tiles.get(index + 2) {
                                is_in_shape_with_next_plus_one = Shape::is_in_shape(t.unwrap().to_id(), t2.unwrap().to_id());
                            }
                        }
                    }
                },
                Option::None => ()
            }

            index += 1;
        }

        99
    }

    /// TODO
    pub fn random_hand(count: u8) -> Hand {
        if count < 13 || count > 14 {
            panic!("Only 13 or 14 tile hands allowed");
        } else {
            Hand {
                tiles: vec!(Option::Some(Tile::new(TileType::Number(1, TileColor::Manzu))))
            }
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
            return Hand {
                tiles: tiles
            }
        }

        panic!("Couldn't parse hand representation.");
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

        assert_eq!(shanten, 1);
    }
}