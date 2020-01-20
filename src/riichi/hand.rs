use std::fmt;

use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;
use super::shanten::ShantenFinder;
use crate::riichi::riichi_error::RiichiError;
use std::collections::HashMap;
use crate::riichi::shapes::Shape;

pub struct Hand {
    /// a hand consists of 13 tiles + 1 drawn tile
    /// it can also have kan, which are groups of 4 tiles that behave as 3 tiles
    /// so we should have a vector with 13 100% present tiles and 5 optional (4 from possible kans and 1 possible draw)
    tiles: Vec<Option<Tile>>,
    array_34: Option<[u8; 34]>,
    shapes: Option<Vec<Shape>>,
    shanten: i8,
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
        if tile_count > 18 || tile_count < 13 {
            return false;
        }

        if self.count_tiles() > 14 {
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
    /// force_return: will return even a partial hand
    pub fn from_text(representation: &str, force_return: bool) -> Result<Hand, RiichiError> {
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
                rep.push(ch);
                rep.push(color);
                match Tile::from_text(&rep[..]) {
                    Ok(mut tile) => {
                        if tiles.is_empty() {
                            // the last tile you write in your hand representation is your drawn tile
                            // TODO this is wrong: should only set for the 14th tile
                            // TODO check for kans!
                            tile.is_draw = true;
                        }
                        tiles.push(Option::Some(tile));
                    },
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }

        tiles.sort();

        let mut hand = Hand::new(tiles);

        if force_return || hand.validate() {
            return Result::Ok(hand);
        }

        Err(RiichiError::new(100, "Couldn't parse hand representation."))
    }

    /// Adds a tile to this hand
    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(Some(tile));
        self.tiles.sort();
    }

    pub fn remove_tile(&mut self, tile: &Tile) {
        let mut found: usize = 999;
        for (i, hand_tile) in self.tiles.iter().enumerate() {
            match hand_tile {
                Some(t) => {
                    if t.to_id() == tile.to_id() {
                        found = i;
                        break;
                    }
                },
                None => ()
            }
        }

        if found != 999 {
            self.tiles.remove(found);
        }
    }

    pub fn remove_tile_by_id(&mut self, tile_id: u8) {
        let tile = Tile::from_id(tile_id).unwrap();
        self.remove_tile(&tile);
    }

    /// Returns the size of a hand - usually 13 or 14 tiles, depending on the situation.
    pub fn count_tiles(&self) -> usize {
        let mut hand_size = 0;
        let mut kan_tiles = 0;

        for tile in self.tiles.iter() {
            match tile {
                Some(t) => {
                    hand_size += 1;
                    if t.is_kan {
                        kan_tiles += 1;
                    }
                },
                None => ()
            }
        }

        // subtract 1 tile for each kan
        hand_size -= (kan_tiles / 4);

        hand_size
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

    pub fn to_array_of_strings(&self) -> Vec<String> {
        let mut tile_vec = vec!();
        let mut color = 'x';
        let mut last_tile: Option<String> = Option::None;

        for tile in self.tiles.iter() {
            match &tile {
                Option::Some(some_tile) => {
                    let mut tile_string = String::from("");
                    if color != some_tile.get_type_char() {
                        color = some_tile.get_type_char();
                    }

                    if color != 'x' {
                        tile_string.push(color);
                    }
                    tile_string.push_str(&format!("{}", some_tile.get_value())[..]);

                    if some_tile.is_draw {
                        last_tile = Option::Some(tile_string);
                    } else {
                        tile_vec.push(tile_string);
                    }
                },
                Option::None => ()
            }
        }

        // tsumo tile will always be the last in the array
        match last_tile {
            Option::Some(tile_repr) => {
                tile_vec.push(tile_repr)
            },
            Option::None => ()
        }

        tile_vec
    }

    /// Get shanten of this hand (and also set it if it's not calculated yet)
    pub fn shanten(&mut self) -> i8 {
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
        self.array_34 = None;
    }

    /// Returns tiles that can be used to improve this hand.
    /// For 13 tile hands, there is only one option.
    /// For 14 tile hands, we list options for all discards that don't lower our shanten.
    pub fn find_shanten_improving_tiles(&mut self) -> HashMap<Option<Tile>, Vec<Tile>> {
        let mut imp_tiles = HashMap::new();

        let current_shanten = self.shanten();

        // tenpai / finished hand has no improving tiles
        if current_shanten <= 0 {
            return imp_tiles;
        }

        // for 13 tile hands, the Option for the discard tile is None
        let hand_count = self.count_tiles();

        if hand_count == 13 {
            let tiles = self.get_shanten_improving_tiles_13(current_shanten);

            imp_tiles.insert(None, tiles);
        } else if hand_count == 14 {
            // first we choose a tile to discard, then we look at our tiles
            let original_shanten = self.shanten();
            let mut hand_tiles = vec!();

            hand_tiles = self.tiles.to_vec();

            for o_tile in hand_tiles.iter() {
                match o_tile {
                    Some(t) => {
                        self.remove_tile(t);
                        self.reset_shanten();
                        let new_shanten = self.shanten();

                        if new_shanten <= original_shanten {
                            // only cares about tiles that don't raise our shanten
                            imp_tiles.insert(Some(t.clone()), self.get_shanten_improving_tiles_13(current_shanten));
                        }

                        self.add_tile(*t);
                    },
                    None => ()
                }
            }
        }

        self.reset_shanten();

        imp_tiles
    }

    fn get_shanten_improving_tiles_13(&mut self, current_shanten: i8) -> Vec<Tile> {
        let mut try_tiles: Vec<u8> = vec!();
        let mut tiles: Vec<Tile> = vec!();

        // we don't need to try all tiles:
        // - the same tile
        // - next tile
        // - next + 1
        // - previous tile
        // - previous - 1
        for o_tile in self.tiles.iter() {
            match o_tile {
                Some(t) => {
                    // get this tile, -1, -2, +1, +2
                    let t_id = t.to_id();
                    if !try_tiles.contains(&t_id) {
                        try_tiles.push(t_id);
                    }

                    let t_prev = t.prev_id(false, 1);
                    if t_prev > 0 && !try_tiles.contains(&t_prev) {
                        try_tiles.push(t_prev);
                    }

                    let t_prev_2 = t.prev_id(false, 2);
                    if t_prev_2 > 0 && !try_tiles.contains(&t_prev_2) {
                        try_tiles.push(t_prev_2);
                    }

                    let t_next = t.next_id(false, 1);
                    if t_next > 0 && !try_tiles.contains(&t_next) {
                        try_tiles.push(t_next);
                    }

                    let t_next_2 = t.next_id(false, 2);
                    if t_next_2 > 0 && !try_tiles.contains(&t_next_2) {
                        try_tiles.push(t_next_2);
                    }
                },
                None => ()
            }
        }

        // we draw a tile and count shanten - if it improves, we add it to the tiles
        for i in try_tiles.iter() {
            let drawn_tile = Tile::from_id(*i).unwrap();
            let tile_str = drawn_tile.to_string();
            self.add_tile(drawn_tile);

            self.reset_shanten();
            let new_shanten = self.shanten();

            if new_shanten < current_shanten {
                tiles.push(Tile::from_id(*i).unwrap());
            }

            self.remove_tile(&Tile::from_id(*i).unwrap());
        }

        tiles
    }
}

impl Default for Hand {
    fn default() -> Hand {
        Hand {
            tiles: vec!(),
            array_34: None,
            shapes: None,
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
        let hand = Hand::from_text(rep, false).unwrap();

        let rep2 = hand.to_string();
        assert_eq!(rep2, rep);
    }

    #[test]
    fn validation_ok() {
        let rep = "123m123p12345s22z";
        let mut hand = Hand::from_text(rep, false).unwrap();

        assert!(hand.validate());
    }

    #[test]
    fn validation_bad_5_same_tiles() {
        let rep = "123m123p11111s22z";
        let mut hand = Hand::from_text(rep, true).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn validation_bad_too_many_tiles() {
        let rep = "123456789m123456789p12345s22z";
        let mut hand = Hand::from_text(rep, true).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn validation_bad_not_enough_tiles() {
        let rep = "123456m";
        let mut hand = Hand::from_text(rep, true).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn find_improving_tiles_2_shanten() {
        let mut hand = Hand::from_text("237m13478s45699p", false).unwrap();

        let tiles = hand.find_shanten_improving_tiles();

        assert_eq!(tiles.get(&None).unwrap().len(), 6);
    }

    #[test]
    fn find_improving_tiles_2_shanten_14() {
        let mut hand = Hand::from_text("237m13478s45699p1z", false).unwrap();

        let map = hand.find_shanten_improving_tiles();

        assert_eq!(map.len(), 4);

        for (o_tile, tiles) in map.iter() {
            match o_tile {
                Some(tile) => {
                    if tile.to_string() == "7m" {
                        assert_eq!(tiles.len(), 6);
                    } else if tile.to_string() == "1s" {
                        assert_eq!(tiles.len(), 6);
                    } else if tile.to_string() == "1z" {
                        assert_eq!(tiles.len(), 6);
                    } else if tile.to_string() == "4s" {
                        assert_eq!(tiles.len(), 5);
                    } else {
                        panic!("Test failed: wrong tiles found");
                    }
                },
                None => ()
            }
        }
    }

    #[test]
    fn find_improving_tiles_14_tenpai() {
        let mut hand = Hand::from_text("123456789p12345m", false).unwrap();
        let map = hand.find_shanten_improving_tiles();

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn count_hand_normal_13() {
        let mut hand = Hand::from_text("237m13478s45699p", false).unwrap();

        assert_eq!(hand.count_tiles(), 13);
    }

    #[test]
    fn count_hand_normal_14() {
        let mut hand = Hand::from_text("1237m13478s45699p", false).unwrap();

        assert_eq!(hand.count_tiles(), 14);
    }

    #[test]
    fn remove_tile() {
        let mut hand = Hand::from_text("1237m13478s45699p", false).unwrap();
        let tile = Tile::from_text("1m").unwrap();
        hand.remove_tile(&tile);

        assert_eq!(hand.count_tiles(), 13);
        assert_eq!(hand.to_string(), "237m45699p13478s")
    }

    #[test]
    fn remove_tile_by_id() {
        let mut hand = Hand::from_text("1237m13478s45699p", false).unwrap();
        let tile_id = 1;
        hand.remove_tile_by_id(tile_id);

        assert_eq!(hand.count_tiles(), 13);
        assert_eq!(hand.to_string(), "237m45699p13478s")
    }
}
