#![allow(clippy::type_complexity)]

use std::fmt;

#[cfg(not(feature = "fast_shanten"))]
use super::shanten::ShantenFinder;
use super::tile::Tile;
#[cfg(feature = "fast_shanten")]
use crate::riichi::fast_hand_calculator::hand_calculator::HandCalculator;
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::rules::Rules;
use crate::riichi::shapes::{ClosedShape, CompleteShape, OpenKan, OpenShape, Shape, ShapeType};
use crate::riichi::tile::{TileColor, TileType};
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;

#[derive(Clone)]
pub struct Hand {
    /// a hand consists of 13 tiles + 1 drawn tile
    /// it can also have kan, which are groups of 4 tiles that behave as 3 tiles
    /// so we should have a vector with 13 100% present tiles and 5 optional (4 from possible kans and 1 possible draw)
    tiles: Vec<Option<Tile>>,
    array_34: Option<[u8; 34]>,
    shapes: Vec<CompleteShape>,
    shanten: i8,
}

impl Hand {
    pub fn new(mut tiles: Vec<Option<Tile>>) -> Hand {
        tiles.sort();

        Hand {
            tiles,
            ..Default::default()
        }
    }

    /// Checks the hand for invalid things (wrong number of tiles, > 4 same tiles...)
    pub fn validate(&self) -> bool {
        let mut tile_count = 0;
        let array34 = self.get_34_array(false);

        for count in array34.iter() {
            tile_count += *count;
            if *count > 4 {
                return false;
            }
        }

        // 13 tiles + 5 optional from kans & draw
        if !(13..=18).contains(&tile_count) {
            return false;
        }

        if self.count_tiles() > 14 {
            return false;
        }

        true
    }

    pub fn get_tiles(&self) -> &Vec<Option<Tile>> {
        &self.tiles
    }

    pub fn get_shapes(&self) -> &Vec<CompleteShape> {
        &self.shapes
    }

    /// Returns a vector of OpenShape from shapes that we have identified in the hand.
    pub fn get_open_shapes(&self) -> Vec<OpenShape> {
        let mut open_shapes = vec![];

        for complete_shape in self.shapes.iter() {
            match complete_shape {
                CompleteShape::Closed(_) => {}
                CompleteShape::Open(open_shape) => open_shapes.push(*open_shape),
            }
        }

        open_shapes
    }

    /// Converts our tiles vector to an array of 34 counts, since riichi has 34 different tiles.
    /// remove_open_tiles: ignores chi, pon and kanned tiles (also closed kans)
    pub fn get_34_array(&self, remove_open_tiles: bool) -> [u8; 34] {
        let mut array_34 = [0; 34];
        for tile in self.tiles.iter().flatten() {
            // ignoring open tiles and kanned tiles
            if !(remove_open_tiles && (tile.is_open || tile.is_kan)) {
                array_34[(tile.to_id() - 1) as usize] += 1;
            }
        }

        array_34
    }

    /// Generate a 14 tile hand that is complete
    /// TODO fix kan generation
    /// TODO add open hand generation
    pub fn random_complete_hand(closed: bool, kans: bool) -> Hand {
        // we are looking to generate 4 shapes + 1 pair, so 5 shapes
        // ignoring kokushi and chiitoitsu for now

        let mut rng = rand::thread_rng();
        let mut pair_found = false;
        let mut used_tiles: [u8; 34] = [0; 34];
        let mut tiles: Vec<Tile> = vec![];

        for i in 0..5 {
            if i == 4 && !pair_found {
                // last shape must be a pair now
                Hand::generate_toitsu(&mut used_tiles, &mut tiles);
                break;
            }

            // open or closed shape?
            let open_or_closed: bool;
            if !closed {
                open_or_closed = rng.gen_bool(0.5);
            } else {
                open_or_closed = false;
            }

            if open_or_closed {
                // TODO open
                let open_shape_type = rng.gen_range(0, 3);
                match open_shape_type {
                    // Chi
                    0 => {}
                    // Pon
                    1 => {}
                    // Kan
                    2 => {}
                    _ => {}
                }
            } else {
                // closed
                let mut max = 4;
                if pair_found {
                    // only 1 pair needed
                    max = 3;
                }

                let mut closed_shape_type;
                loop {
                    closed_shape_type = rng.gen_range(0, max);
                    if closed_shape_type == 2 && !kans {
                        continue;
                    }

                    break;
                }

                match closed_shape_type {
                    // Shuntsu
                    0 => {
                        let mut tile_id: u8;

                        loop {
                            tile_id = rng.gen_range(0, 27); // we don't need honors

                            if used_tiles[tile_id as usize] == 4 {
                                continue;
                            }

                            let tile = Tile::from_id((tile_id + 1) as u8).unwrap();
                            if tile.next(false).is_none() {
                                // 9, we go backwards (789)
                                if used_tiles[(tile_id - 1) as usize] < 4
                                    && used_tiles[(tile_id - 2) as usize] < 4
                                {
                                    // ok
                                    used_tiles[tile_id as usize] += 1;
                                    used_tiles[(tile_id - 1) as usize] += 1;
                                    used_tiles[(tile_id - 2) as usize] += 1;

                                    tiles.push(Tile::from_id(tile_id + 1).unwrap());
                                    tiles.push(Tile::from_id(tile_id).unwrap());
                                    tiles.push(Tile::from_id(tile_id - 1).unwrap());

                                    break;
                                }
                            } else if tile.next(false).unwrap().next(false).is_none() {
                                // 8, we do 678 or 789
                                if rng.gen_bool(0.5) {
                                    // 678
                                    if used_tiles[(tile_id - 1) as usize] < 4
                                        && used_tiles[(tile_id - 2) as usize] < 4
                                    {
                                        // ok
                                        used_tiles[tile_id as usize] += 1;
                                        used_tiles[(tile_id - 1) as usize] += 1;
                                        used_tiles[(tile_id - 2) as usize] += 1;

                                        tiles.push(Tile::from_id(tile_id + 1).unwrap());
                                        tiles.push(Tile::from_id(tile_id).unwrap());
                                        tiles.push(Tile::from_id(tile_id - 1).unwrap());

                                        break;
                                    }
                                } else {
                                    // 789
                                    if used_tiles[(tile_id - 1) as usize] < 4
                                        && used_tiles[(tile_id + 1) as usize] < 4
                                    {
                                        // ok
                                        used_tiles[tile_id as usize] += 1;
                                        used_tiles[(tile_id - 1) as usize] += 1;
                                        used_tiles[(tile_id + 1) as usize] += 1;

                                        tiles.push(Tile::from_id(tile_id).unwrap());
                                        tiles.push(Tile::from_id(tile_id + 1).unwrap());
                                        tiles.push(Tile::from_id(tile_id + 2).unwrap());

                                        break;
                                    }
                                }
                            } else {
                                // others do next next
                                if used_tiles[(tile_id + 1) as usize] < 4
                                    && used_tiles[(tile_id + 2) as usize] < 4
                                {
                                    // ok
                                    used_tiles[tile_id as usize] += 1;
                                    used_tiles[(tile_id + 1) as usize] += 1;
                                    used_tiles[(tile_id + 2) as usize] += 1;

                                    tiles.push(Tile::from_id(tile_id + 1).unwrap());
                                    tiles.push(Tile::from_id(tile_id + 2).unwrap());
                                    tiles.push(Tile::from_id(tile_id + 3).unwrap());

                                    break;
                                }
                            }
                        }
                    }
                    // Koutsu
                    1 => {
                        let mut tile_id: u8;

                        loop {
                            tile_id = rng.gen_range(0, 34);

                            if used_tiles[tile_id as usize] > 1 {
                                continue;
                            }

                            used_tiles[tile_id as usize] += 3;
                            tiles.push(Tile::from_id(tile_id + 1).unwrap());
                            tiles.push(Tile::from_id(tile_id + 1).unwrap());
                            tiles.push(Tile::from_id(tile_id + 1).unwrap());
                            break;
                        }
                    }
                    // Kantsu
                    2 => {
                        let mut tile_id: u8;

                        loop {
                            tile_id = rng.gen_range(0, 34);

                            if used_tiles[tile_id as usize] > 0 {
                                continue;
                            }

                            used_tiles[tile_id as usize] += 4;

                            let mut tile = Tile::from_id(tile_id + 1).unwrap();
                            tile.is_kan = true;

                            tiles.push(tile);
                            tiles.push(tile);
                            tiles.push(tile);
                            tiles.push(tile);
                            break;
                        }
                    }
                    // Toitsu
                    3 => {
                        Hand::generate_toitsu(&mut used_tiles, &mut tiles);
                        pair_found = true;
                    }
                    _ => {}
                }
            }
        }

        let mut final_tiles = vec![];
        let mut found_draw = false;

        tiles.shuffle(&mut rng);

        for tile in tiles.iter_mut() {
            if !found_draw && !tile.is_open && !tile.is_kan {
                tile.is_draw = true;
                found_draw = true;
            }
            final_tiles.push(Some(*tile));
        }

        Hand::new(final_tiles)
    }

    /// Generate a random closed hand
    pub fn random_hand(rules: Option<&Rules>) -> Hand {
        let mut rng = rand::thread_rng();
        let mut used_tiles: [u8; 34] = [0; 34];
        let mut tiles: Vec<Option<Tile>> = vec![];

        // if I generated a red 5, save it here.
        // 0 = m, 1 = p, 2 = s
        let mut red5s: [bool; 3] = [false; 3];

        for i in 0..14 {
            let mut tile_id;
            loop {
                tile_id = rng.gen_range(1, 35);

                if used_tiles[tile_id - 1_usize] == 4 {
                    continue;
                }

                break;
            }

            used_tiles[tile_id - 1_usize] += 1;

            let mut tile = Tile::from_id(tile_id as u8).unwrap();
            if i == 0 {
                tile.is_draw = true;
            }

            if let Some(r) = rules {
                if r.aka_ari {
                    if let TileType::Number(value, color) = tile.tile_type {
                        // 25% chance of having a red 5
                        if value == 5 {
                            match color {
                                TileColor::Manzu => {
                                    if !red5s[0] && rng.gen_range(0, 4) == 0 {
                                        tile.is_red = true;
                                        red5s[0] = true;
                                    }
                                }
                                TileColor::Pinzu => {
                                    if !red5s[1] && rng.gen_range(0, 4) == 0 {
                                        tile.is_red = true;
                                        red5s[1] = true;
                                    }
                                }
                                TileColor::Souzu => {
                                    if !red5s[2] && rng.gen_range(0, 4) == 0 {
                                        tile.is_red = true;
                                        red5s[2] = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            tiles.push(Some(tile));
        }

        Hand::new(tiles)
    }

    fn generate_toitsu(used_tiles: &mut [u8; 34], tiles: &mut Vec<Tile>) {
        let mut rng = rand::thread_rng();
        let mut tile_id: u8;
        loop {
            tile_id = rng.gen_range(0, 34);
            if used_tiles[tile_id as usize] < 3 {
                break;
            }
        }

        used_tiles[tile_id as usize] += 2;
        tiles.push(Tile::from_id(tile_id + 1).unwrap());
        tiles.push(Tile::from_id(tile_id + 1).unwrap());
    }

    /// Parses a hand from its text representation.
    /// force_return: will return even a partial/invalid hand
    /// TODO: red 5
    pub fn from_text(representation: &str, force_return: bool) -> Result<Hand, RiichiError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<closed>[0-9mspz]+)|\((?P<chi>[0-9]{3}[msp][0-2])\)|\((?P<pon>p[0-9][mspz][1-3]r?)\)|\((?P<kan>k[0-9][mspz][1-3]?r?)\)|\((?P<shouminkan>s[0-9][mspz][1-3]?r?)\)").unwrap();
        }

        let mut closed = vec![];
        let mut shapes: Vec<CompleteShape> = vec![];
        let mut called_tiles = vec![];

        for cap in RE.captures_iter(representation) {
            match cap.name("closed") {
                None => {}
                Some(value) => closed.push(value.as_str()),
            }
            match cap.name("chi") {
                None => {}
                Some(chi) => {
                    if let Ok(mut result) = Hand::parse_chi(chi.as_str()) {
                        called_tiles.append(&mut result.0);
                        shapes.push(result.1);
                    }
                }
            }
            match cap.name("pon") {
                None => {}
                Some(pon) => {
                    if let Ok(mut result) = Hand::parse_pon(pon.as_str()) {
                        called_tiles.append(&mut result.0);
                        shapes.push(result.1);
                    }
                }
            }
            match cap.name("kan") {
                None => {}
                Some(value) => {
                    if let Ok(mut result) = Hand::parse_kan(value.as_str()) {
                        called_tiles.append(&mut result.0);
                        shapes.push(result.1);
                    }
                }
            }
            match cap.name("shouminkan") {
                None => {}
                Some(value) => {
                    if let Ok(mut result) = Hand::parse_kan(value.as_str()) {
                        called_tiles.append(&mut result.0);
                        shapes.push(result.1);
                    }
                }
            }
        }

        if closed.len() != 1 {
            return Err(RiichiError::new(333, "Closed hand not defined correctly"));
        }

        let mut tiles = match Hand::parse_closed_hand(closed.get(0).unwrap()) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        tiles.append(&mut called_tiles);

        let mut hand = Hand::new(tiles);

        for shape in shapes {
            match shape {
                CompleteShape::Closed(closed_kan) => {
                    hand.add_closed_kan(closed_kan);
                }
                CompleteShape::Open(open_shape) => {
                    hand.add_open_shape(&open_shape);
                }
            }
        }

        if force_return || hand.validate() {
            return Result::Ok(hand);
        }

        Err(RiichiError::new(100, "Couldn't parse hand representation."))
    }

    /// A chi looks like this:
    /// (XYZCN) where
    /// X, Y, Z: consecutive tile numbers (0-9) * 3
    /// C = color (msp)
    /// N = which tile has been called? Index 0-2
    ///
    /// Only insides of the brackets are in the chis vector.
    fn parse_chi(chi: &str) -> Result<(Vec<Option<Tile>>, CompleteShape), RiichiError> {
        let mut tiles: Vec<Option<Tile>> = Vec::new();
        let c = chi.chars().nth(3).unwrap();
        let n = chi.chars().nth(4).unwrap();

        let mut got_called = false;

        let mut tile_1 =
            Tile::from_text(&format!("{}{}", chi.chars().next().unwrap(), c)[..]).unwrap();
        if n.eq(&'0') {
            tile_1.called_from = 3;
            got_called = true;
        }
        let mut tile_2 =
            Tile::from_text(&format!("{}{}", chi.chars().nth(1).unwrap(), c)[..]).unwrap();
        if n.eq(&'1') {
            tile_2.called_from = 3;
            got_called = true;
        }
        let mut tile_3 =
            Tile::from_text(&format!("{}{}", chi.chars().nth(2).unwrap(), c)[..]).unwrap();
        if n.eq(&'2') {
            tile_3.called_from = 3;
            got_called = true;
        }

        if !got_called {
            return Err(RiichiError::new(780, "Called tile was not specified"));
        }

        tiles.push(Some(tile_1));
        tiles.push(Some(tile_2));
        tiles.push(Some(tile_3));

        let shape = CompleteShape::Open(OpenShape::Chi([tile_1, tile_2, tile_3]));

        Ok((tiles, shape))
    }

    /// A pon looks like this:
    /// (pNCPr) where
    /// p = pon
    /// N = number 0-9
    /// C = color (mpsz)
    /// P = player who was ponned
    /// r = optional: if the pon has a red 5 (0m, 0p, 0s), the r signifies it was the red 5 that was called.
    ///
    /// Only insides of the brackets are in the pons vector.
    fn parse_pon(pon: &str) -> Result<(Vec<Option<Tile>>, CompleteShape), RiichiError> {
        let mut tiles: Vec<Option<Tile>> = Vec::new();
        // number
        let mut n = pon.chars().nth(1).unwrap();
        // color
        let c = pon.chars().nth(2).unwrap();
        // player
        let p = pon.chars().nth(3).unwrap();
        // red 5 called?
        let r = match pon.chars().nth(4) {
            None => false,
            Some(red) => red == 'r',
        };

        if n != '0' && r {
            return Err(RiichiError::new(589, "Only 0 can have r"));
        }

        let mut first_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();

        if n == '0' {
            // red 5, change to 5 for the other tiles
            n = '5';
        }

        let mut second_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();
        let third_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();

        // TODO maybe sometimes specify exactly which one was called with id_136?
        if r {
            first_tile.called_from = p.to_digit(10).unwrap() as u8;
        } else {
            second_tile.called_from = p.to_digit(10).unwrap() as u8;
        }

        let shape = CompleteShape::Open(OpenShape::Pon([first_tile, second_tile, third_tile]));

        tiles.push(Some(first_tile));
        tiles.push(Some(second_tile));
        tiles.push(Some(third_tile));

        Ok((tiles, shape))
    }

    /// A kan looks like this:
    /// ([k|s]NCPr) where
    /// k|s = kan|shouminkan
    /// N = number 0-9
    /// C = color (mpsz)
    /// P = player who was kanned (or ponned originally, if the kan is upgraded from pon). Optional - closed kans don't have this.
    /// r = optional with open kans: if the kan has a red 5 (0m, 0p, 0s), the r signifies it was the red 5 that was called.
    ///
    /// Only insides of the brackets are in the kans vector.
    fn parse_kan(kan: &str) -> Result<(Vec<Option<Tile>>, CompleteShape), RiichiError> {
        let mut tiles: Vec<Option<Tile>> = Vec::new();

        //type
        let kan_type = kan.chars().next().unwrap();
        // number
        let mut n = kan.chars().nth(1).unwrap();
        // color
        let c = kan.chars().nth(2).unwrap();
        // player
        let p = match kan.chars().nth(3) {
            None => 0,
            Some(value) => value.to_digit(10).unwrap() as u8,
        } as u8;
        // red 5 called?
        let r = match kan.chars().nth(4) {
            None => false,
            Some(red) => red == 'r',
        };

        if n != '0' && r {
            return Err(RiichiError::new(589, "Only 0 can have r"));
        }

        let mut first_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();

        if n == '0' {
            // red 5, change to 5 for the other tiles
            n = '5';
        }

        let mut second_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();
        let third_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();
        let fourth_tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();

        let shape = if p > 0 {
            if r {
                first_tile.called_from = p;
            } else {
                second_tile.called_from = p;
            }

            let open_kan_type = if kan_type == 's' {
                OpenKan::Shouminkan([first_tile, second_tile, third_tile, fourth_tile])
            } else {
                OpenKan::Daiminkan([first_tile, second_tile, third_tile, fourth_tile])
            };

            CompleteShape::Open(OpenShape::Kan(open_kan_type))
        } else {
            CompleteShape::Closed(ClosedShape::Kantsu([
                first_tile,
                second_tile,
                third_tile,
                fourth_tile,
            ]))
        };

        tiles.push(Some(first_tile));
        tiles.push(Some(second_tile));
        tiles.push(Some(third_tile));
        tiles.push(Some(fourth_tile));

        Ok((tiles, shape))
    }

    /// Closed part of the hand (or the whole hand, if we have no open tiles / kans)
    fn parse_closed_hand(closed: &str) -> Result<Vec<Option<Tile>>, RiichiError> {
        // let's read the hand from the back, because colors are written after the numbers
        let iter = closed.chars().rev();
        let mut tiles: Vec<Option<Tile>> = Vec::new();

        let mut color: char = 'x';
        let mut rep: String;
        let mut found_draw: bool = false;

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
                        if !found_draw && !tile.is_open && !tile.is_kan {
                            // the last tile you write in your hand representation is your drawn tile
                            tile.is_draw = true;
                            found_draw = true;
                        }
                        tiles.push(Option::Some(tile));
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }

        Ok(tiles)
    }

    /// Adds a tile to this hand and sets it as drawn
    pub fn draw_tile(&mut self, tile: &Tile) {
        self.reset_drawn_tiles();
        let mut tile = tile.clone();
        tile.is_draw = true;
        self.add_tile(tile);
    }

    /// Adds a tile to this hand
    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(Some(tile));
        self.tiles.sort();
    }

    /// Removes a tile from this hand
    pub fn remove_tile(&mut self, tile: &Tile) {
        let mut found: usize = 999;
        for (i, hand_tile) in self.tiles.iter().enumerate() {
            match hand_tile {
                Some(t) => {
                    if !t.is_open && !t.is_kan && t.to_id() == tile.to_id() {
                        found = i;
                        break;
                    }
                }
                None => (),
            }
        }

        if found != 999 {
            self.tiles.remove(found);
            self.reset_shanten();
        }
    }

    /// Removes a tile by ID
    pub fn remove_tile_by_id(&mut self, tile_id: u8) {
        let tile = Tile::from_id(tile_id).unwrap();
        self.remove_tile(&tile);
    }

    // /// Do a closed kan with these tiles, if it has them
    // pub fn ankan_tiles(&mut self, mut tile: Tile) {
    //     let array_34 = self.get_34_array(true);
    //     if array_34[(tile.to_id() - 1) as usize] != 4 {
    //         panic!("Trying to kan, but don't have 4 tiles!");
    //     }
    //
    //     // remove the kanned tiles
    //     self.tiles.retain(|x| match x {
    //         None => true,
    //         Some(t) => {
    //             if t.to_id() == tile.to_id() {
    //                 return false;
    //             }
    //
    //             true
    //         }
    //     });
    //
    //     // add them as kanned
    //     tile.is_kan = true;
    //
    //     self.add_tile(tile);
    //     self.add_tile(tile);
    //     self.add_tile(tile);
    //     self.add_tile(tile);
    // }

    /// Goes through the tiles and dedicates them to an open shape
    pub fn add_open_shape(&mut self, shape: &OpenShape) {
        match shape {
            OpenShape::Chi(tiles) => {
                for tile in tiles.iter() {
                    let mut found = false;
                    for (i, t) in self.tiles.iter().enumerate() {
                        match t {
                            None => {}
                            Some(mut hand_tile) => {
                                if hand_tile.eq(tile) && !hand_tile.is_open && !hand_tile.is_kan {
                                    hand_tile.is_open = true;
                                    hand_tile.is_chi = true;
                                    self.tiles[i] = Some(hand_tile);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        panic!("Invalid tiles in open shape");
                    }
                }
            }
            OpenShape::Pon(tiles) => {
                for tile in tiles.iter() {
                    let mut found = false;
                    for (i, t) in self.tiles.iter().enumerate() {
                        match t {
                            None => {}
                            Some(mut hand_tile) => {
                                if hand_tile.eq(tile) && !hand_tile.is_open && !hand_tile.is_kan {
                                    hand_tile.is_open = true;
                                    hand_tile.is_pon = true;
                                    self.tiles[i] = Some(hand_tile);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        panic!("Invalid tiles in open shape");
                    }
                }
            }
            OpenShape::Kan(open_kan) => match open_kan {
                OpenKan::Daiminkan(tiles) | OpenKan::Shouminkan(tiles) => {
                    for tile in tiles.iter() {
                        let mut found = false;
                        for (i, t) in self.tiles.iter().enumerate() {
                            match t {
                                None => {}
                                Some(mut hand_tile) => {
                                    if hand_tile.eq(tile) && !hand_tile.is_open && !hand_tile.is_kan
                                    {
                                        hand_tile.is_open = true;
                                        hand_tile.is_kan = true;
                                        self.tiles[i] = Some(hand_tile);
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if !found {
                            panic!("Invalid tiles in open shape");
                        }
                    }
                }
            },
        }

        self.shapes.push(CompleteShape::Open(*shape));
    }

    pub fn add_closed_kan(&mut self, kan: ClosedShape) {
        match kan {
            ClosedShape::Kantsu(tiles) => {
                for tile in tiles.iter() {
                    let mut found = false;
                    for (i, t) in self.tiles.iter().enumerate() {
                        match t {
                            None => {}
                            Some(mut hand_tile) => {
                                if hand_tile.eq(tile) && !hand_tile.is_open && !hand_tile.is_kan {
                                    hand_tile.is_open = false;
                                    hand_tile.is_kan = true;
                                    self.tiles[i] = Some(hand_tile);
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !found {
                        panic!("Invalid tiles in open shape");
                    }
                }

                self.shapes.push(CompleteShape::Closed(kan))
            }
            _ => panic!("This is not a kan"),
        }
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
                }
                None => (),
            }
        }

        // subtract 1 tile for each kan
        hand_size -= kan_tiles / 4;

        hand_size
    }

    #[cfg(not(feature = "fast_shanten"))]
    fn get_tile_count_by_id(&self, tile_id: u8) -> u8 {
        self.get_34_array(false)[(tile_id - 1) as usize]
    }

    pub fn get_closed_kans(&self) -> u8 {
        let mut array_34 = [0u8; 34];
        let mut cnt = 0;
        for t_o in self.tiles.iter() {
            match t_o {
                None => {}
                Some(tile) => {
                    if !tile.is_open && tile.is_kan {
                        array_34[(tile.to_id() - 1) as usize] += 1;
                        if array_34[(tile.to_id() - 1) as usize] == 4 {
                            cnt += 1;
                        }
                    }
                }
            }
        }

        cnt
    }

    pub fn is_closed(&self) -> bool {
        self.get_open_shapes().is_empty()
    }

    /// Renders the hand as a string representation.
    /// The notation looks like this:
    /// Tiles are written as NC (number + color)
    /// More tiles of the same color can be written consecutively (1234p)
    /// z = honors - 1-4 = winds, 5-7 = dragons
    /// 0mps = red 5 (TODO)
    /// Open shapes are in brackets, (123s2) (p1s1) (k2p3) etc. See parse_* functions for more info
    pub fn convert_to_string(&self) -> String {
        let mut out = String::new();
        let mut color = 'x';
        let mut last_tile: Option<&Tile> = Option::None;

        let mut tiles = self.tiles.clone();

        for complete_shape in self.shapes.iter() {
            match complete_shape {
                CompleteShape::Closed(closed_shape) => {
                    if let ClosedShape::Kantsu(closed_kan) = closed_shape {
                        self.remove_meld_from_tiles(&closed_kan.to_vec(), &mut tiles);
                    }
                }
                CompleteShape::Open(open_shape) => match open_shape {
                    OpenShape::Chi(tls) | OpenShape::Pon(tls) => {
                        self.remove_meld_from_tiles(&tls.to_vec(), &mut tiles);
                    }
                    OpenShape::Kan(open_kan) => match open_kan {
                        OpenKan::Daiminkan(tls) | OpenKan::Shouminkan(tls) => {
                            self.remove_meld_from_tiles(&tls.to_vec(), &mut tiles)
                        }
                    },
                },
            }
        }

        for tile in tiles.iter() {
            match &tile {
                Option::Some(some_tile) => {
                    if color != some_tile.get_type_char() {
                        if color != 'x' {
                            out.push_str(&color.to_string()[..]);
                        }
                        color = some_tile.get_type_char();
                    }

                    if some_tile.is_draw {
                        last_tile = Option::Some(some_tile);
                    } else {
                        out.push_str(&some_tile.get_value().to_string()[..]);
                    }
                }
                Option::None => (),
            }
        }

        match last_tile {
            Option::Some(ltt) => {
                if ltt.get_type_char() == color {
                    out.push_str(&ltt.get_value().to_string()[..]);
                    out.push_str(&color.to_string()[..]);
                } else {
                    out.push_str(&color.to_string()[..]);
                    out.push_str(&ltt.to_string());
                }
            }
            Option::None => out.push_str(&color.to_string()[..]),
        }

        for complete_shape in self.shapes.iter() {
            match complete_shape {
                CompleteShape::Closed(closed_shape) => {
                    if let ClosedShape::Kantsu(closed_kan) = closed_shape {
                        out.push_str(
                            &Shape::new(
                                ShapeType::Complete(CompleteShape::Closed(ClosedShape::Kantsu(
                                    *closed_kan,
                                ))),
                                4,
                                true,
                            )
                            .to_string()[..],
                        );
                    }
                }
                CompleteShape::Open(open_shape) => match open_shape {
                    OpenShape::Chi(tls) => out.push_str(
                        &Shape::new(
                            ShapeType::Complete(CompleteShape::Open(OpenShape::Chi(*tls))),
                            3,
                            true,
                        )
                        .to_string()[..],
                    ),
                    OpenShape::Pon(tls) => out.push_str(
                        &Shape::new(
                            ShapeType::Complete(CompleteShape::Open(OpenShape::Pon(*tls))),
                            3,
                            true,
                        )
                        .to_string()[..],
                    ),
                    OpenShape::Kan(open_kan) => {
                        let shape_type = match open_kan {
                            OpenKan::Daiminkan(tls) => ShapeType::Complete(CompleteShape::Open(
                                OpenShape::Kan(OpenKan::Daiminkan(*tls)),
                            )),
                            OpenKan::Shouminkan(tls) => ShapeType::Complete(CompleteShape::Open(
                                OpenShape::Kan(OpenKan::Shouminkan(*tls)),
                            )),
                        };

                        out.push_str(&Shape::new(shape_type, 4, true).to_string()[..])
                    }
                },
            }
        }

        out
    }

    fn remove_meld_from_tiles(&self, meld_tiles: &[Tile], tiles: &mut Vec<Option<Tile>>) {
        for meld_tile in meld_tiles.iter() {
            let mut index = 0;
            for (i, tile) in tiles.iter().enumerate() {
                match tile {
                    None => {}
                    Some(t) => {
                        if t.eq(meld_tile) {
                            index = i;
                            break;
                        }
                    }
                }
            }

            tiles.remove(index);
        }
    }

    /// Returns the tiles of the hand as a vec of their string representations.
    /// Can be returned without open/closed shapes.
    pub fn to_vec_of_strings(&self, no_shapes: bool) -> Vec<String> {
        let mut tile_vec = vec![];
        let mut color = 'x';
        let mut last_tile: Option<String> = Option::None;

        for tile in self.tiles.iter() {
            match &tile {
                Option::Some(some_tile) => {
                    if no_shapes && (some_tile.is_open || some_tile.is_kan) {
                        continue;
                    }

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
                }
                Option::None => (),
            }
        }

        // tsumo tile will always be the last in the array
        match last_tile {
            Option::Some(tile_repr) => tile_vec.push(tile_repr),
            Option::None => (),
        }

        tile_vec
    }

    /// Get shanten of this hand (and also set it if it's not calculated yet)
    #[cfg(not(feature = "fast_shanten"))]
    pub fn shanten(&mut self) -> i8 {
        if self.shanten == 99 {
            match ShantenFinder::new().shanten(self) {
                // match FastShantenFinder::new().shanten(self) {
                Ok(shanten) => {
                    self.shanten = shanten;
                }
                Err(_error) => (),
            }
        }

        self.shanten
    }

    /// Get shanten of this hand
    #[cfg(not(feature = "fast_shanten"))]
    pub fn get_shanten(&self) -> i8 {
        match ShantenFinder::new().shanten(&self) {
            Ok(shanten) => shanten,
            Err(_error) => 99,
        }
    }

    /// Get shanten of this hand (and also set it if it's not calculated yet)
    #[cfg(feature = "fast_shanten")]
    pub fn shanten(&mut self) -> i8 {
        if self.shanten == 99 {
            let mut hc = HandCalculator::new();
            hc.init(&self);

            self.shanten = hc.shanten();
        }

        self.shanten
    }

    /// Get shanten of this hand
    #[cfg(feature = "fast_shanten")]
    pub fn get_shanten(&self) -> i8 {
        let mut hc = HandCalculator::new();
        hc.init(&self);

        hc.shanten()
    }

    /// Reset shanten to 99 when we change the hand somehow
    pub fn reset_shanten(&mut self) {
        self.shanten = 99;
        self.array_34 = None;
    }

    /// Returns tiles that can be used to improve this hand.
    /// For 13 tile hands, there is only one option.
    /// For 14 tile hands, we list options for all discards that don't lower our shanten.
    /// You can set visible_tiles that you can see on the table and it will remove them from the final list / ukeire count
    #[cfg(not(feature = "fast_shanten"))]
    pub fn find_shanten_improving_tiles(
        &self,
        visible_tiles: Option<&[u8; 34]>,
    ) -> Vec<(Option<Tile>, Vec<(Tile, u8)>, u8)> {
        let mut imp_tiles = vec![];
        let count_total_ukeire =
            |ukeires: &Vec<(Tile, u8)>| ukeires.iter().map(|u| u.1).sum::<u8>();

        let mut hand = self.clone();

        let current_shanten = hand.shanten();

        // for 13 tile hands, the Option for the discard tile is None
        let hand_count = hand.count_tiles();

        if hand_count == 13 {
            let mut result = hand.get_shanten_improving_tiles_13(current_shanten, &visible_tiles);

            result.sort();
            imp_tiles.push((None, result.clone(), count_total_ukeire(&result)));
        } else if hand_count == 14 {
            // finished hand has no improving tiles
            if current_shanten < 0 {
                return imp_tiles;
            }

            // first we choose a tile to discard, then we look at our tiles
            let original_shanten = hand.shanten();
            let hand_tiles = hand.tiles.to_vec();

            let mut tried = vec![];
            for o_tile in hand_tiles.iter() {
                match o_tile {
                    Some(t) => {
                        if t.is_open || t.is_kan {
                            continue;
                        }

                        if tried.contains(&t.to_id()) {
                            continue;
                        }

                        tried.push(t.to_id());
                        hand.remove_tile(t);
                        hand.reset_shanten();
                        let new_shanten = hand.shanten();

                        if new_shanten <= original_shanten {
                            // only cares about tiles that don't raise our shanten
                            let mut result = hand
                                .get_shanten_improving_tiles_13(current_shanten, &visible_tiles);
                            result.sort();
                            let cnt = count_total_ukeire(&result);
                            imp_tiles.push((Some(*t), result, cnt));
                        }

                        hand.add_tile(*t);
                    }
                    None => (),
                }
            }
        }

        hand.reset_shanten();

        imp_tiles.sort_by(|a, b| b.2.cmp(&a.2));
        imp_tiles
    }

    #[cfg(feature = "fast_shanten")]
    pub fn find_shanten_improving_tiles(
        &self,
        visible_tiles: Option<&[u8; 34]>,
    ) -> Vec<(Option<Tile>, Vec<(Tile, u8)>, u8)> {
        let mut hc = HandCalculator::new();
        hc.init(&self);

        let mut imp_tiles = vec![];
        let count_total_ukeire =
            |ukeires: &Vec<(Tile, u8)>| ukeires.iter().map(|u| u.1).sum::<u8>();

        let add_uke_ire_reuslt_to_tiles =
            |tile_id: &usize, uke_count: &i32, tiles: &mut Vec<(Tile, u8)>| {
                if *uke_count > 0 {
                    let uke_count = match visible_tiles {
                        None => *uke_count,
                        Some(vt) => 4 - vt[*tile_id] as i32,
                    };

                    if uke_count > 0 {
                        tiles.push((Tile::from_id(*tile_id as u8 + 1).unwrap(), uke_count as u8));
                    }
                }
            };

        let current_shanten = hc.shanten();

        // for 13 tile hands, the Option for the discard tile is None
        let hand_count = self.count_tiles();

        if hand_count == 13 {
            let results = hc.get_uke_ire_for_13();

            let mut tiles = vec![];
            for (tile_id, uke_count) in results.iter().enumerate() {
                add_uke_ire_reuslt_to_tiles(&tile_id, &uke_count, &mut tiles);
            }

            tiles.sort();
            let cnt = count_total_ukeire(&tiles);
            imp_tiles.push((None, tiles, cnt))
        } else if hand_count == 14 {
            // finished hand has no improving tiles
            if current_shanten < 0 {
                return imp_tiles;
            }

            // first we choose a tile to discard, then we look at our tiles
            let hand_tiles = self.tiles.to_vec();

            let mut tried = vec![];
            for o_tile in hand_tiles.iter() {
                match o_tile {
                    Some(t) => {
                        if t.is_open || t.is_kan {
                            continue;
                        }

                        if tried.contains(&t.to_id()) {
                            continue;
                        }

                        tried.push(t.to_id());
                        hc.discard(&t);

                        let new_shanten = hc.shanten();

                        if new_shanten <= current_shanten {
                            // only cares about tiles that don't raise our shanten
                            let results = hc.get_uke_ire_for_13();

                            let mut tiles = vec![];
                            for (tile_id, uke_count) in results.iter().enumerate() {
                                add_uke_ire_reuslt_to_tiles(&tile_id, &uke_count, &mut tiles);
                            }

                            tiles.sort();
                            let cnt = count_total_ukeire(&tiles);
                            imp_tiles.push((Some(*t), tiles, cnt));
                        }

                        hc.draw(&t);
                    }
                    None => (),
                }
            }
        }

        imp_tiles
    }

    #[cfg(not(feature = "fast_shanten"))]
    fn get_shanten_improving_tiles_13(
        &self,
        current_shanten: i8,
        visible_tiles: &Option<&[u8; 34]>,
    ) -> Vec<(Tile, u8)> {
        let mut try_tiles: Vec<u8> = vec![];
        let mut tiles_and_counts = vec![];

        let mut hand = self.clone();

        // we don't need to try all tiles:
        // - the same tile
        // - next tile
        // - next + 1
        // - previous tile
        // - previous - 1
        // - all terminals and honors because kokushi

        for o_tile in hand.tiles.iter() {
            match o_tile {
                Some(t) => {
                    if t.is_open || t.is_kan {
                        continue;
                    }

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
                }
                None => (),
            }
        }

        // terminals and honors check
        if hand.is_closed() {
            for tile_id in [1, 9, 10, 18, 19, 27, 28, 29, 30, 31, 32, 33, 34].iter() {
                if !try_tiles.contains(&tile_id) {
                    try_tiles.push(*tile_id);
                }
            }
        }

        let array_34 = hand.get_34_array(true);

        // we draw a tile and count shanten - if it improves, we add it to the tiles
        for i in try_tiles.iter() {
            if hand.get_tile_count_by_id(*i) == 4 {
                continue;
            }
            let drawn_tile = Tile::from_id(*i).unwrap();
            // let tile_str = drawn_tile.to_string();
            hand.add_tile(drawn_tile);

            hand.reset_shanten();
            let new_shanten = hand.shanten();
            // println!("new shanten with {} = {}", drawn_tile.to_string(), new_shanten);

            if new_shanten < current_shanten {
                tiles_and_counts.push((
                    Tile::from_id(*i).unwrap(),
                    match visible_tiles {
                        None => 4 - array_34[*i as usize - 1],
                        Some(v_t) => 4 - v_t[*i as usize - 1],
                    },
                ));
                // we remove tiles that are visible in the hand from ukeire
            }

            hand.remove_tile(&Tile::from_id(*i).unwrap());
        }

        tiles_and_counts
    }

    pub fn reset_drawn_tiles(&mut self) {
        let mut new_tiles = vec![];
        for p_tile in self.tiles.iter() {
            match p_tile {
                Some(mut tile) => {
                    tile.is_draw = false;
                    new_tiles.push(Some(tile));
                }
                None => (),
            }
        }

        new_tiles.sort();
        self.tiles = new_tiles;
    }

    pub fn get_drawn_tile(&self) -> Option<&Tile> {
        for p_tile in self.tiles.iter() {
            match p_tile {
                Some(tile) => {
                    if tile.is_draw {
                        return Some(tile);
                    }
                }
                None => (),
            }
        }

        None
    }
}

impl Default for Hand {
    fn default() -> Hand {
        Hand {
            tiles: vec![],
            array_34: None,
            shapes: vec![],
            shanten: 99,
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.convert_to_string())
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
    fn from_text_hand_add_chi() {
        let rep = "123m123p12345s22z";
        let mut hand = Hand::from_text(rep, false).unwrap();

        let mut called_tile = Tile::from_text("2m").unwrap();
        called_tile.called_from = 3; // always called from 3
        hand.add_open_shape(&OpenShape::Chi([
            Tile::from_text("1m").unwrap(),
            called_tile,
            Tile::from_text("3m").unwrap(),
        ]));

        let mut open_tiles_count = 0u8;
        for rt in hand.get_tiles().iter() {
            match rt {
                None => {}
                Some(tile) => {
                    if tile.is_open {
                        open_tiles_count += 1;
                    }
                }
            }
        }

        let rep2 = hand.to_string();
        assert_eq!(rep2, "123p12345s22z(123m1)");

        assert_eq!(open_tiles_count, 3);

        assert_eq!(hand.get_open_shapes().len(), 1);
    }

    #[test]
    fn from_text_hand_add_pon() {
        let rep = "444m123p12345s22z";
        let mut hand = Hand::from_text(rep, false).unwrap();

        let mut tile = Tile::from_text("4m").unwrap();
        tile.called_from = 1;
        hand.add_open_shape(&OpenShape::Pon([tile, tile, tile]));

        let mut open_tiles_count = 0u8;
        for rt in hand.get_tiles().iter() {
            match rt {
                None => {}
                Some(tile) => {
                    if tile.is_open {
                        open_tiles_count += 1;
                    }
                }
            }
        }

        let rep2 = hand.to_string();
        assert_eq!(rep2, "123p12345s22z(p4m1)");

        assert_eq!(open_tiles_count, 3);

        assert_eq!(hand.get_open_shapes().len(), 1);
    }

    #[test]
    fn validation_ok() {
        let rep = "123m123p12345s22z";
        let hand = Hand::from_text(rep, false).unwrap();

        assert!(hand.validate());
    }

    #[test]
    fn validation_bad_5_same_tiles() {
        let rep = "123m123p11111s22z";
        let hand = Hand::from_text(rep, true).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn validation_bad_too_many_tiles() {
        let rep = "123456789m123456789p12345s22z";
        let hand = Hand::from_text(rep, true).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn validation_bad_not_enough_tiles() {
        let rep = "123456m";
        let hand = Hand::from_text(rep, true).unwrap();

        assert!(!hand.validate());
    }

    #[test]
    fn find_improving_tiles_2_shanten() {
        let hand = Hand::from_text("237m13478s45699p", false).unwrap();

        let tiles = hand.find_shanten_improving_tiles(None);

        assert_eq!(tiles.get(0).unwrap().1.len(), 6);
    }

    #[test]
    fn find_improving_tiles_tenpai_13_open() {
        let mut hand = Hand::from_text("4m23334p1m(789s1)(p2m1)", false).unwrap();

        assert_eq!(hand.shanten(), 1);

        let result = hand.find_shanten_improving_tiles(None);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn find_improving_tiles_tenpai_14_open() {
        let mut hand = Hand::from_text("44m23334p1m(789s1)(p2m1)", false).unwrap();

        assert_eq!(hand.shanten(), 0);

        let result = hand.find_shanten_improving_tiles(None);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn find_improving_tiles_2_shanten_14() {
        let hand = Hand::from_text("237m13478s45699p1z", false).unwrap();

        let result = hand.find_shanten_improving_tiles(None);

        assert_eq!(result.len(), 4);

        for row in result.iter() {
            match row.0 {
                Some(tile) => {
                    if tile.to_string() == "7m" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 6);
                    } else if tile.to_string() == "1s" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 6);
                    } else if tile.to_string() == "1z" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 6);
                    } else if tile.to_string() == "4s" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 5);
                        assert_eq!(row.1[0].1, 4);
                        assert_eq!(row.1[1].1, 4);
                        assert_eq!(row.1[2].1, 4);
                        assert_eq!(row.1[3].1, 4);
                        assert_eq!(row.1[4].1, 4);
                        assert_eq!(row.2, 20);
                    } else {
                        panic!("Test failed: wrong tiles found");
                    }
                }
                None => (),
            }
        }
    }

    #[test]
    fn find_improving_tiles_2_shanten_14_open() {
        let hand = Hand::from_text("237m13478s99p1z(456p0)", false).unwrap();

        let result = hand.find_shanten_improving_tiles(None);

        assert_eq!(result.len(), 4);

        for row in result.iter() {
            match row.0 {
                Some(tile) => {
                    if tile.to_string() == "7m" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 6);
                    } else if tile.to_string() == "1s" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 6);
                    } else if tile.to_string() == "1z" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 6);
                    } else if tile.to_string() == "4s" {
                        println!("tajl: {} count: {}", tile.to_string(), row.2);
                        //                        println!("{:#?}", row.1);
                        assert_eq!(row.1.len(), 5);
                        assert_eq!(row.1[0].0.to_string(), "1m");
                        assert_eq!(row.1[0].1, 4);
                        assert_eq!(row.1[1].0.to_string(), "4m");
                        assert_eq!(row.1[1].1, 4);
                        assert_eq!(row.1[2].1, 4);
                        assert_eq!(row.1[3].1, 4);
                        assert_eq!(row.1[4].1, 4);
                        assert_eq!(row.2, 20);
                    } else {
                        panic!("Test failed: wrong tiles found");
                    }
                }
                None => (),
            }
        }
    }

    #[test]
    fn find_improving_tiles_13_tenpai() {
        let hand = Hand::from_text("888p333s12345m77z", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        let waiting_tiles = map.get(0).unwrap();

        println!("{:#?}", waiting_tiles);

        assert_eq!(map.len(), 1);
        assert_eq!(waiting_tiles.1.len(), 2);
    }

    #[test]
    fn find_improving_tiles_14_tenpai() {
        let hand = Hand::from_text("123456789p12345m", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        assert_eq!(map.len(), 4);
    }

    #[test]
    fn find_improving_tiles_14_tenpai_daiminkan() {
        let mut hand = Hand::from_text("111m222s333p56z(k4z1)", false).unwrap();
        // let mut hand = Hand::from_text("111m222s333p56z444z", false).unwrap();

        assert_eq!(hand.count_tiles(), 14);
        assert_eq!(hand.shanten(), 0);

        let map = hand.find_shanten_improving_tiles(None);

        let waiting_tiles = map.get(0).unwrap();

        assert_eq!(map.len(), 2);
        assert_eq!(waiting_tiles.1.len(), 1);
    }

    #[test]
    fn find_improving_tiles_14_complete() {
        let hand = Hand::from_text("123456789p12344m", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn find_improving_tiles_14_kokushi() {
        let hand = Hand::from_text("129m19s19p1234566z", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        println!("{:#?}", map);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn find_improving_tiles_13_3() {
        let hand = Hand::from_text("1234s123p999m456z", false).unwrap();
        let result = hand.find_shanten_improving_tiles(None);

        println!("{:#?}", result);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn find_improving_tiles_13_with_some_visible_tiles() {
        let hand = Hand::from_text("1234s123p999m456z", false).unwrap();

        let mut visible_tiles = hand.get_34_array(false);
        visible_tiles[Tile::from_text("1s").unwrap().to_id() as usize - 1] += 2; // 1s is also somewhere else

        let result = hand.find_shanten_improving_tiles(Some(&visible_tiles));

        println!("{:#?}", result);

        assert_eq!(result.len(), 1);

        // count of 1s that can come to my hand is 3
        assert_eq!(result[0].1[0].1, 1);
    }

    #[test]
    fn find_improving_tiles_14_repeating() {
        let hand = Hand::from_text("12356m12333s4499p", false).unwrap();
        let result = hand.find_shanten_improving_tiles(None);

        println!("{:#?}", result);

        assert_eq!(result.len(), 7);
    }

    #[test]
    fn count_hand_normal_13() {
        let hand = Hand::from_text("237m13478s45699p", false).unwrap();

        assert_eq!(hand.count_tiles(), 13);
    }

    #[test]
    fn count_hand_normal_14() {
        let hand = Hand::from_text("1237m13478s45699p", false).unwrap();

        assert_eq!(hand.count_tiles(), 14);
    }

    #[test]
    fn remove_tile() {
        let mut hand = Hand::from_text("1237m13478s45699p", false).unwrap();
        let tile = Tile::from_text("1m").unwrap();
        hand.remove_tile(&tile);

        assert_eq!(hand.count_tiles(), 13);
        assert_eq!(hand.to_string(), "237m4569p13478s9p")
    }

    #[test]
    fn remove_tile_by_id() {
        let mut hand = Hand::from_text("1237m13478s45699p", false).unwrap();
        let tile_id = 1;
        hand.remove_tile_by_id(tile_id);

        assert_eq!(hand.count_tiles(), 13);
        assert_eq!(hand.to_string(), "237m4569p13478s9p")
    }

    #[test]
    fn reset_drawn_tiles() {
        let mut hand = Hand::from_text("1237m13478s4569p1z", false).unwrap();
        let dt = hand.get_drawn_tile().unwrap();

        assert_eq!(dt.is_draw, true);

        hand.reset_drawn_tiles();

        let dt = hand.get_drawn_tile();
        match dt {
            None => {}
            Some(_) => {
                panic!("I should not have found a drawn tile here!");
            }
        }
    }

    #[test]
    fn tile_counting_with_kan() {
        let mut hand = Hand::from_text("23m456s678p22z", true).unwrap();
        let mut tile = Tile::from_id(1).unwrap();

        // 4 kan tiles = 3 tiles
        tile.is_kan = true;
        hand.add_tile(tile);
        hand.add_tile(tile);
        hand.add_tile(tile);
        hand.add_tile(tile);

        assert_eq!(hand.count_tiles(), 13);
    }

    #[test]
    fn tile_counting_with_kan_method() {
        let mut hand = Hand::from_text("111123m456s678p22z", false).unwrap();
        let tile = Tile::from_id(1).unwrap();

        hand.add_closed_kan(ClosedShape::Kantsu([tile, tile, tile, tile]));

        assert_eq!(hand.count_tiles(), 13);
    }

    #[test]
    fn shanten_with_kan_method() {
        let mut hand = Hand::from_text("111123m456s678p22z", false).unwrap();
        let tile = Tile::from_id(1).unwrap();

        hand.add_closed_kan(ClosedShape::Kantsu([tile, tile, tile, tile]));

        println!("{}", hand);

        assert_eq!(hand.shanten(), 0);
    }

    /// I think this should be tenpai. We need to check with others.
    // #[test]
    // fn karaten_tenpai() {
    //     let mut hand = Hand::from_text("13m456m44466p(k2m)", false).unwrap();
    //
    //     assert_eq!(hand.shanten(), 0);
    // }

    #[test]
    fn random_complete_hand() {
        let mut hand = Hand::random_complete_hand(true, false);

        println!("{}", hand.to_string());
        println!("{}", hand.count_tiles());

        assert!(hand.validate());
        assert_eq!(hand.count_tiles(), 14);
        assert_eq!(hand.shanten(), -1);
    }

    #[test]
    fn random_hand() {
        let hand = Hand::random_hand(None);

        println!("{}", hand.to_string());
        println!("{}", hand.count_tiles());

        assert!(hand.validate());
        assert_eq!(hand.count_tiles(), 14);
    }

    #[test]
    fn parse_open_hand_two_pons() {
        let mut hand = Hand::from_text("123456m11p(p7s1)(p4p2)", false).unwrap();

        println!("{}", hand.to_string());
        println!("{}", hand.count_tiles());

        assert!(hand.validate());
        assert_eq!(hand.count_tiles(), 14);
        assert_eq!(hand.shanten(), -1);
    }

    #[test]
    fn parse_open_hand_two_chis() {
        let mut hand = Hand::from_text("123456m11p(123s0)(345s1)", false).unwrap();

        println!("{}", hand.to_string());
        println!("{}", hand.count_tiles());

        assert_eq!(hand.to_string(), "123456m11p(123s0)(345s1)");
        assert!(hand.validate());
        assert_eq!(hand.count_tiles(), 14);
        assert_eq!(hand.shanten(), -1);
    }

    #[test]
    fn parse_open_hand_kans() {
        let mut hand = Hand::from_text("123456m11p(k1s)(k2s1)", false).unwrap();

        println!("{}", hand.to_string());
        println!("{}", hand.count_tiles());

        assert_eq!(hand.to_string(), "123456m11p(k1s)(k2s1)");
        assert!(hand.validate());
        assert_eq!(hand.count_tiles(), 14);
        assert_eq!(hand.shanten(), -1);
    }

    #[test]
    fn parse_open_hand_kans_shouminkan() {
        let mut hand = Hand::from_text("123456m11p(s2s1)(789p1)", false).unwrap();

        println!("{}", hand.to_string());
        println!("{}", hand.count_tiles());

        assert_eq!(hand.to_string(), "123456m11p(s2s1)(789p1)");
        assert!(hand.validate());
        assert_eq!(hand.count_tiles(), 14);
        assert_eq!(hand.shanten(), -1);
    }

    #[test]
    fn parse_hand_with_red_5_closed() {
        let hand = Hand::from_text("123406m111p222s33z", false).unwrap();

        assert_eq!(hand.to_string(), "123406m111p222s33z");
    }

    #[test]
    fn parse_hand_with_red_5_chi() {
        let hand = Hand::from_text("123m111p222s33z(406m1)", false).unwrap();

        assert_eq!(hand.to_string(), "123m111p222s33z(406m1)");
    }

    #[test]
    fn parse_hand_with_red_5_pon() {
        let hand = Hand::from_text("123m111p222s33z(p0m1)", false).unwrap();

        assert_eq!(hand.to_string(), "123m111p222s33z(p0m1)");

        let mut reds = 0;
        for tile in hand.get_tiles().iter().filter(|t| match t {
            None => false,
            Some(tile) => tile.get_value() == 5 || tile.get_value() == 0,
        }) {
            match tile {
                None => {}
                Some(t) => {
                    if t.is_red {
                        reds += 1;
                    }
                }
            }
        }

        assert_eq!(reds, 1);
    }

    #[test]
    fn parse_hand_with_red_5_kan() {
        let hand = Hand::from_text("123m111p222s33z(k0m1)", false).unwrap();

        assert_eq!(hand.to_string(), "123m111p222s33z(k0m1)");

        let mut reds = 0;
        for tile in hand.get_tiles().iter().filter(|t| match t {
            None => false,
            Some(tile) => tile.get_value() == 5 || tile.get_value() == 0,
        }) {
            match tile {
                None => {}
                Some(t) => {
                    if t.is_red {
                        reds += 1;
                    }
                }
            }
        }

        assert_eq!(reds, 1);
    }
}
