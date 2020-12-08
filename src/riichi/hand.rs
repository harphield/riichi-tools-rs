use std::fmt;

use super::shanten::ShantenFinder;
use super::tile::Tile;
use super::tile::TileColor;
use super::tile::TileType;
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::shapes::{OpenShape, Shape, ClosedShape, CompleteShape, ShapeType};
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
    pub fn validate(&mut self) -> bool {
        let mut tile_count = 0;
        let array34 = self.get_34_array(false);

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

    pub fn get_tiles(&self) -> &Vec<Option<Tile>> {
        &self.tiles
    }

    /// Returns a vector of OpenShape from shapes that we have identified in the hand.
    pub fn get_open_shapes(&self) -> Vec<OpenShape> {
        let mut open_shapes = vec![];

        for complete_shape in self.shapes.iter() {
            match complete_shape {
                CompleteShape::Closed(_) => {}
                CompleteShape::Open(open_shape) => open_shapes.push(*open_shape)
            }
        }

        open_shapes
    }

    /// Converts our tiles vector to an array of 34 counts, since riichi has 34 different tiles.
    /// remove_open_tiles: ignores chi, pon and kanned tiles (also closed kans)
    pub fn get_34_array(&self, remove_open_tiles: bool) -> [u8; 34] {
        let mut array_34 = [0; 34];
        for tile in self.tiles.iter() {
            if let Option::Some(t) = tile {
                // ignoring open tiles and kanned tiles
                if !(remove_open_tiles && (t.is_open || t.is_kan)) {
                    array_34[(t.to_id() - 1) as usize] += 1;
                }
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

            if open_or_closed == true {
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
            static ref RE: Regex = Regex::new(r"(?P<closed>[0-9mspz]+)|\((?P<chi>[0-9]{3}[msp][0-2])\)|\((?P<pon>p[0-9][mspz][1-3])\)|\((?P<kan>k[0-9][mspz][1-3]?)\)").unwrap();
        }

        let mut closed = vec![];
        let mut chis = vec![];
        let mut pons = vec![];
        let mut kans = vec![];
        for cap in RE.captures_iter(representation) {
            match cap.name("closed") {
                None => {}
                Some(value) => closed.push(value.as_str())
            }
            match cap.name("chi") {
                None => {}
                Some(value) => chis.push(value.as_str())
            }
            match cap.name("pon") {
                None => {}
                Some(value) => pons.push(value.as_str())
            }
            match cap.name("kan") {
                None => {}
                Some(value) => kans.push(value.as_str())
            }
        }

        if closed.len() != 1 {
            return Err(RiichiError::new(333, "Closed hand not defined correctly"));
        }

        let mut tiles = match Hand::parse_closed_hand(closed.get(0).unwrap()) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        let mut chi_tiles_and_shapes = match Hand::parse_chis(&chis) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        let mut pon_tiles_and_shapes = match Hand::parse_pons(&pons) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        let mut kan_tiles_and_shapes = match Hand::parse_kans(&kans) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };

        tiles.append(&mut chi_tiles_and_shapes.0);
        tiles.append(&mut pon_tiles_and_shapes.0);
        tiles.append(&mut kan_tiles_and_shapes.0);

        let mut hand = Hand::new(tiles);

        for shape in chi_tiles_and_shapes.1 {
            hand.add_open_shape(shape);
        }

        for shape in pon_tiles_and_shapes.1 {
            hand.add_open_shape(shape);
        }

        for shape in kan_tiles_and_shapes.1 {
            match shape {
                CompleteShape::Closed(closed_kan) => {
                    hand.add_closed_kan(closed_kan);
                }
                CompleteShape::Open(open_shape) => {
                    hand.add_open_shape(open_shape);
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
    fn parse_chis(chis: &Vec<&str>) -> Result<(Vec<Option<Tile>>, Vec<OpenShape>), RiichiError> {
        let mut tiles: Vec<Option<Tile>> = Vec::new();
        let mut shapes: Vec<OpenShape> = Vec::new();

        for chi in chis.iter() {
            let c = chi.chars().nth(3).unwrap();
            let n = chi.chars().nth(4).unwrap();

            let mut tile_1 = Tile::from_text(&format!("{}{}", chi.chars().nth(0).unwrap(), c)[..]).unwrap();
            if n.eq(&'0') {
                tile_1.called_from = 3;
            }
            let mut tile_2 = Tile::from_text(&format!("{}{}", chi.chars().nth(1).unwrap(), c)[..]).unwrap();
            if n.eq(&'1') {
                tile_2.called_from = 3;
            }
            let mut tile_3 = Tile::from_text(&format!("{}{}", chi.chars().nth(2).unwrap(), c)[..]).unwrap();
            if n.eq(&'2') {
                tile_3.called_from = 3;
            }

            tiles.push(Some(tile_1));
            tiles.push(Some(tile_2));
            tiles.push(Some(tile_3));

            shapes.push(OpenShape::Chi([
                tile_1,
                tile_2,
                tile_3,
            ]))
        }

        Ok((tiles, shapes))
    }

    /// A pon looks like this:
    /// (pNCP) where
    /// p = pon
    /// N = number 0-9
    /// C = color (mpsz)
    /// P = player who was ponned
    ///
    /// Only insides of the brackets are in the pons vector.
    fn parse_pons(pons: &Vec<&str>) -> Result<(Vec<Option<Tile>>, Vec<OpenShape>), RiichiError> {
        let mut tiles: Vec<Option<Tile>> = Vec::new();
        let mut shapes: Vec<OpenShape> = Vec::new();

        for pon in pons.iter() {
            // number
            let n = pon.chars().nth(1).unwrap();
            // color
            let c = pon.chars().nth(2).unwrap();
            // player
            let p = pon.chars().nth(3).unwrap();

            let mut tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();

            // TODO maybe sometimes specify exactly which one was called with id_136?
            tile.called_from = p.to_digit(10).unwrap() as u8;
            shapes.push(OpenShape::Pon([
                tile,
                tile,
                tile,
            ]));

            tiles.push(Some(tile));
            tiles.push(Some(tile));
            tiles.push(Some(tile));
        }

        Ok((tiles, shapes))
    }

    /// A kan looks like this:
    /// (kNCP) where
    /// k = kan
    /// N = number 0-9
    /// C = color (mpsz)
    /// P = player who was kanned (or ponned originally, if the kan is upgraded from pon). Optional - closed kans don't have this.
    ///
    /// Only insides of the brackets are in the kans vector.
    fn parse_kans(kans: &Vec<&str>) -> Result<(Vec<Option<Tile>>, Vec<CompleteShape>), RiichiError> {
        let mut tiles: Vec<Option<Tile>> = Vec::new();
        let mut shapes: Vec<CompleteShape> = Vec::new();

        for kan in kans.iter() {
            // number
            let n = kan.chars().nth(1).unwrap();
            // color
            let c = kan.chars().nth(2).unwrap();
            // player
            let p = match kan.chars().nth(3) {
                None => 0,
                Some(value) => value.to_digit(10).unwrap() as u8
            } as u8;

            let mut tile = Tile::from_text(&format!("{}{}", n, c)[..]).unwrap();

            tiles.push(Some(tile));
            tiles.push(Some(tile));
            tiles.push(Some(tile));

            if p > 0 {
                tile.called_from = p;
                shapes.push(CompleteShape::Open(OpenShape::Kan([
                        tile,
                        tile,
                        tile,
                        tile,
                    ]))
                );
            } else {
                shapes.push(CompleteShape::Closed(ClosedShape::Kantsu([
                        tile,
                        tile,
                        tile,
                        tile,
                    ]))
                );
            }
            tiles.push(Some(tile));
        }

        Ok((tiles, shapes))
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
                    if t.to_id() == tile.to_id() {
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

    /// Do a closed kan with these tiles, if it has them
    pub fn ankan_tiles(&mut self, mut tile: Tile) {
        let array_34 = self.get_34_array(true);
        if array_34[(tile.to_id() - 1) as usize] != 4 {
            panic!("Trying to kan, but don't have 4 tiles!");
        }

        // remove the kanned tiles
        self.tiles.retain(|x| match x {
            None => true,
            Some(t) => {
                if t.to_id() == tile.to_id() {
                    return false;
                }

                true
            }
        });

        // add them as kanned
        tile.is_kan = true;

        self.add_tile(tile);
        self.add_tile(tile);
        self.add_tile(tile);
        self.add_tile(tile);
    }

    /// Goes through the tiles and dedicates them to an open shape
    pub fn add_open_shape(&mut self, shape: OpenShape) {
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
            OpenShape::Kan(tiles) => {
                for tile in tiles.iter() {
                    let mut found = false;
                    for (i, t) in self.tiles.iter().enumerate() {
                        match t {
                            None => {}
                            Some(mut hand_tile) => {
                                if hand_tile.eq(tile) && !hand_tile.is_open && !hand_tile.is_kan {
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
        }

        self.shapes.push(CompleteShape::Open(shape));
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
            },
            _ => panic!("This is not a kan")
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

    fn get_tile_count_by_id(&self, tile_id: u8) -> u8 {
        self.get_34_array(false)[(tile_id - 1) as usize]
    }

    pub fn get_kans(&self) -> u8 {
        let mut array_34 = [0u8; 34];
        let mut cnt = 0;
        for t_o in self.tiles.iter() {
            match t_o {
                None => {}
                Some(tile) => {
                    if tile.is_kan {
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
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        let mut color = 'x';
        let mut last_tile: Option<&Tile> = Option::None;

        let mut tiles = self.tiles.clone();

        for complete_shape in self.shapes.iter() {
            match complete_shape {
                CompleteShape::Closed(closed_shape) => {
                    match closed_shape {
                        ClosedShape::Kantsu(closed_kan) => {
                            self.remove_meld_from_tiles(&closed_kan.to_vec(), &mut tiles);
                        }
                        _ => {}
                    }
                }
                CompleteShape::Open(open_shape) => {
                    match open_shape {
                        OpenShape::Chi(tls) | OpenShape::Pon(tls) => {
                            self.remove_meld_from_tiles(&tls.to_vec(), &mut tiles);
                        },
                        OpenShape::Kan(tls) => {
                            self.remove_meld_from_tiles(&tls.to_vec(), &mut tiles);
                        }
                    }
                }
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
            },
            Option::None => out.push_str(&color.to_string()[..]),
        }

        for complete_shape in self.shapes.iter() {
            match complete_shape {
                CompleteShape::Closed(closed_shape) => {
                    match closed_shape {
                        ClosedShape::Kantsu(closed_kan) => out.push_str(&Shape::new(ShapeType::Complete(CompleteShape::Closed(ClosedShape::Kantsu(*closed_kan))), 4, true).to_string()[..]),
                        _ => {}
                    }
                }
                CompleteShape::Open(open_shape) => {
                    match open_shape {
                        OpenShape::Chi(tls) => out.push_str(&Shape::new(ShapeType::Complete(CompleteShape::Open(OpenShape::Chi(*tls))), 3, true).to_string()[..]),
                        OpenShape::Pon(tls) => out.push_str(&Shape::new(ShapeType::Complete(CompleteShape::Open(OpenShape::Pon(*tls))), 3, true).to_string()[..]),
                        OpenShape::Kan(tls) => out.push_str(&Shape::new(ShapeType::Complete(CompleteShape::Open(OpenShape::Kan(*tls))), 4, true).to_string()[..]),
                    }
                }
            }
        }

        out
    }

    fn remove_meld_from_tiles(&self, meld_tiles: &Vec<Tile>, tiles: &mut Vec<Option<Tile>>) {
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

    pub fn to_vec_of_strings(&self) -> Vec<String> {
        let mut tile_vec = vec![];
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
    pub fn shanten(&mut self) -> i8 {
        if self.shanten == 99 {
            match ShantenFinder::new().shanten(self) {
                Ok(shanten) => {
                    self.shanten = shanten;
                }
                Err(_error) => (),
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
    /// You can set visible_tiles that you can see on the table and it will remove them from the final list / ukeire count
    pub fn find_shanten_improving_tiles(
        &mut self,
        visible_tiles: Option<&[u8; 34]>,
    ) -> Vec<(Option<Tile>, Vec<(Tile, u8)>, u8)> {
        let mut imp_tiles = vec![];
        let count_total_ukeire =
            |ukeires: &Vec<(Tile, u8)>| ukeires.iter().map(|u| u.1).sum::<u8>();

        let current_shanten = self.shanten();

        // for 13 tile hands, the Option for the discard tile is None
        let hand_count = self.count_tiles();

        if hand_count == 13 {
            let mut result = self.get_shanten_improving_tiles_13(current_shanten, &visible_tiles);

            result.sort();
            imp_tiles.push((None, result.clone(), count_total_ukeire(&result)));
        } else if hand_count == 14 {
            // finished hand has no improving tiles
            if current_shanten < 0 {
                return imp_tiles;
            }

            // first we choose a tile to discard, then we look at our tiles
            let original_shanten = self.shanten();
            let hand_tiles = self.tiles.to_vec();

            let mut tried = vec![];
            for o_tile in hand_tiles.iter() {
                match o_tile {
                    Some(t) => {
                        if tried.contains(&t.to_id()) {
                            continue;
                        }

                        tried.push(t.to_id());
                        self.remove_tile(t);
                        self.reset_shanten();
                        let new_shanten = self.shanten();

                        if new_shanten <= original_shanten {
                            // only cares about tiles that don't raise our shanten
                            let mut result = self
                                .get_shanten_improving_tiles_13(current_shanten, &visible_tiles);
                            result.sort();
                            imp_tiles.push((
                                Some(t.clone()),
                                result.clone(),
                                count_total_ukeire(&result),
                            ));
                        }

                        self.add_tile(*t);
                    }
                    None => (),
                }
            }
        }

        self.reset_shanten();

        imp_tiles.sort_by(|a, b| b.2.cmp(&a.2));
        imp_tiles
    }

    fn get_shanten_improving_tiles_13(
        &mut self,
        current_shanten: i8,
        visible_tiles: &Option<&[u8; 34]>,
    ) -> Vec<(Tile, u8)> {
        let mut try_tiles: Vec<u8> = vec![];
        let mut tiles_and_counts = vec![];

        // we don't need to try all tiles:
        // - the same tile
        // - next tile
        // - next + 1
        // - previous tile
        // - previous - 1
        // - all terminals and honors because kokushi

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
                }
                None => (),
            }
        }

        // terminals and honors check
        for tile_id in [1, 9, 10, 18, 19, 27, 28, 29, 30, 31, 32, 33, 34].iter() {
            if !try_tiles.contains(&tile_id) {
                try_tiles.push(*tile_id);
            }
        }

        let array_34 = self.get_34_array(true);

        // we draw a tile and count shanten - if it improves, we add it to the tiles
        for i in try_tiles.iter() {
            if self.get_tile_count_by_id(*i) == 4 {
                continue;
            }
            let drawn_tile = Tile::from_id(*i).unwrap();
            // let tile_str = drawn_tile.to_string();
            self.add_tile(drawn_tile);

            self.reset_shanten();
            let new_shanten = self.shanten();
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

            self.remove_tile(&Tile::from_id(*i).unwrap());
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
    fn from_text_hand_add_chi() {
        let rep = "123m123p12345s22z";
        let mut hand = Hand::from_text(rep, false).unwrap();

        let mut called_tile = Tile::from_text("2m").unwrap();
        called_tile.called_from = 3; // always called from 3
        hand.add_open_shape(OpenShape::Chi([
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
        hand.add_open_shape(OpenShape::Pon([
            tile,
            tile,
            tile,
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
        assert_eq!(rep2, "123p12345s22z(p4m1)");

        assert_eq!(open_tiles_count, 3);

        assert_eq!(hand.get_open_shapes().len(), 1);
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

        let tiles = hand.find_shanten_improving_tiles(None);

        assert_eq!(tiles.get(0).unwrap().1.len(), 6);
    }

    #[test]
    fn find_improving_tiles_2_shanten_14() {
        let mut hand = Hand::from_text("237m13478s45699p1z", false).unwrap();

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
    fn find_improving_tiles_13_tenpai() {
        let mut hand = Hand::from_text("888p333s12345m77z", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        println!("{:#?}", map);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn find_improving_tiles_14_tenpai() {
        let mut hand = Hand::from_text("123456789p12345m", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        assert_eq!(map.len(), 4);
    }

    #[test]
    fn find_improving_tiles_14_complete() {
        let mut hand = Hand::from_text("123456789p12344m", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn find_improving_tiles_14_kokushi() {
        let mut hand = Hand::from_text("129m19s19p1234566z", false).unwrap();
        let map = hand.find_shanten_improving_tiles(None);

        println!("{:#?}", map);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn find_improving_tiles_13_3() {
        let mut hand = Hand::from_text("1234s123p999m456z", false).unwrap();
        let result = hand.find_shanten_improving_tiles(None);

        println!("{:#?}", result);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn find_improving_tiles_14_repeating() {
        let mut hand = Hand::from_text("12356m12333s4499p", false).unwrap();
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

        hand.ankan_tiles(tile);

        assert_eq!(hand.count_tiles(), 13);
    }

    #[test]
    fn shanten_with_kan_method() {
        let mut hand = Hand::from_text("111123m456s678p22z", false).unwrap();
        let tile = Tile::from_id(1).unwrap();

        hand.ankan_tiles(tile);

        assert_eq!(hand.shanten(), 0);
    }

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
}
