use crate::riichi::riichi_error::RiichiError;
use serde::Serializer;
use std::cmp::Ordering;
use std::fmt;
use wasm_bindgen::__rt::core::fmt::{Display, Formatter};

// '0m', '1m', '2m', '3m', '4m', '5m', '6m', '7m', '8m', '9m',
// '0p', '1p', '2p', '3p', '4p', '5p', '6p', '7p', '8p', '9p',
// '0s', '1s', '2s', '3s', '4s', '5s', '6s', '7s', '8s', '9s',
//  E    S    W     N
// '1z', '2z', '3z', '4z',
//  W    G     R
// '5z', '6z', '7z'

#[derive(Debug, Clone, Copy, Hash)]
pub enum TileType {
    Number(u8, TileColor),
    Wind(u8),
    Dragon(u8),
}

impl TileType {
    pub fn to_char(&self) -> char {
        match &self {
            TileType::Number(_number, color) => color.to_char(),
            TileType::Wind(_number) => 'z',
            TileType::Dragon(_number) => 'z',
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum TileColor {
    Manzu,
    Pinzu,
    Souzu,
}

impl TileColor {
    pub fn from_char(rep: &char) -> Result<TileColor, RiichiError> {
        match rep {
            'm' => Ok(TileColor::Manzu),
            'p' => Ok(TileColor::Pinzu),
            's' => Ok(TileColor::Souzu),
            _ => Err(RiichiError::new(106, "Wrong representation of tile color!")),
        }
    }

    pub fn to_char(&self) -> char {
        match &self {
            TileColor::Manzu => 'm',
            TileColor::Pinzu => 'p',
            TileColor::Souzu => 's',
        }
    }
}

impl fmt::Display for TileColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_red: bool,
    pub is_open: bool,
    pub is_draw: bool,
    pub is_chi: bool,
    pub is_pon: bool,
    pub called_from: u8,
    pub is_kan: bool,
    pub is_riichi: bool,
    pub is_tsumogiri: bool,
    pub id_136: Option<u8>,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        match &tile_type {
            TileType::Number(number, _color) => {
                if *number > 9 {
                    panic!("Numbers can be only up to 9");
                }
            }
            TileType::Wind(number) => {
                if *number > 4 {
                    panic!("Winds can be only up to 4");
                }
            }
            TileType::Dragon(number) => {
                if *number < 5 || *number > 7 {
                    panic!("Dragons can be only 5-7");
                }
            }
        }

        Tile {
            tile_type,
            ..Default::default()
        }
    }

    pub fn from_text(representation: &str) -> Result<Tile, RiichiError> {
        if representation.len() != 2 {
            return Err(RiichiError::new(105, "Tile length must be 2"));
        }

        let mut r_chars = representation.chars();
        let first_char = &r_chars.next().unwrap();
        let second_char = &r_chars.next().unwrap();
        let mut number = first_char.to_string().parse().unwrap();

        if ['m', 'p', 's'].contains(second_char) {
            let color: TileColor;
            if *second_char == 'm' {
                color = TileColor::Manzu;
            } else if *second_char == 'p' {
                color = TileColor::Pinzu;
            } else if *second_char == 's' {
                color = TileColor::Souzu;
            } else {
                return Err(RiichiError::new(102, "Wrong color, only m, p an s allowed"));
            }

            // red fives are represented by a 0
            let mut is_red = false;
            if number == 0 {
                number = 5;
                is_red = true;
            }
            let mut new_tile = Tile::new(TileType::Number(number, color));
            new_tile.is_red = is_red;

            Ok(new_tile)
        } else if *second_char == 'z' {
            if number > 0 && number <= 4 {
                // winds
                Ok(Tile::new(TileType::Wind(number)))
            } else if number > 4 && number <= 7 {
                // dragons
                Ok(Tile::new(TileType::Dragon(number)))
            } else {
                Err(RiichiError::new(103, "Wrong number for honors!"))
            }
        } else {
            Err(RiichiError::new(104, "Invalid tile definition"))
        }
    }

    /// id is an integer value > 0 of a tile.
    /// The order is Manzu - Pinzu - Souzu - Winds - Dragons
    pub fn from_id(id: u8) -> Result<Tile, RiichiError> {
        if !(1..=34).contains(&id) {
            return Err(RiichiError::new(107, &format!("Wrong tile ID {}", id)[..]));
        }

        if id <= 9 {
            return Ok(Tile::new(TileType::Number(id, TileColor::Manzu)));
        }

        if id <= 18 {
            return Ok(Tile::new(TileType::Number(id - 9, TileColor::Pinzu)));
        }

        if id <= 27 {
            return Ok(Tile::new(TileType::Number(id - 18, TileColor::Souzu)));
        }

        if id <= 31 {
            return Ok(Tile::new(TileType::Wind(id - 27)));
        }

        Ok(Tile::new(TileType::Dragon(id - 27)))
    }

    /// Gets the id of this tile based on its type
    pub fn to_id(&self) -> u8 {
        match &self.tile_type {
            TileType::Number(number, color) => match color {
                TileColor::Manzu => *number,
                TileColor::Pinzu => number + 9,
                TileColor::Souzu => number + 18,
            },
            TileType::Wind(number) => number + 27,
            TileType::Dragon(number) => number + 27,
        }
    }

    /// Some algorithms use ids starting from 0
    pub fn to_id_minus_1(&self) -> u8 {
        self.to_id() - 1
    }

    /// Returns an ID of the next tile in order.
    pub fn next_id(&self, dora: bool, depth: u8) -> u8 {
        let id = self.to_id();

        // manzu
        if id < 9 - (depth - 1) {
            return id + depth;
        }

        if id == 9 && !dora {
            return 0;
        }

        if id == 9 && dora {
            return depth;
        }

        // pinzu
        if id < 18 - (depth - 1) {
            return id + depth;
        }

        if id == 18 && !dora {
            return 0;
        }

        if id == 18 && dora {
            return 10 + (depth - 1);
        }

        // souzu
        if id < 27 - (depth - 1) {
            return id + depth;
        }

        if id == 27 && !dora {
            return 0;
        }

        if id == 27 && dora {
            return 19 + (depth - 1);
        }

        // honors
        if dora {
            if id < 31 - (depth - 1) {
                // winds
                return id + depth;
            } else if id == 31 {
                return 28 + (depth - 1);
            } else if id < 34 - (depth - 1) {
                // dragons
                return id + depth;
            } else if id == 34 {
                return 31 + (depth - 1);
            }
        }

        0
    }

    /// Returns an ID of the previous tile in order.
    pub fn prev_id(&self, dora: bool, depth: u8) -> u8 {
        let id = self.to_id();

        // manzu
        if id > (1 + depth - 1) && id <= 9 {
            return id - depth;
        }

        if id == 1 && !dora {
            return 0;
        }

        if id == 1 && dora {
            return 9 - (depth - 1);
        }

        // pinzu
        if id > (10 + depth - 1) && id <= 18 {
            return id - depth;
        }

        if id == 10 && !dora {
            return 0;
        }

        if id == 10 && dora {
            return 18 - (depth - 1);
        }

        // souzu
        if id > (19 + depth - 1) && id < 27 {
            return id - depth;
        }

        if id == 19 && !dora {
            return 0;
        }

        if id == 19 && dora {
            return 27 - (depth - 1);
        }

        // honors
        if dora {
            if id > (28 + depth - 1) && id < 32 {
                return id - depth;
            } else if id == 28 {
                return 31 - (depth - 1);
            } else if id > (32 + depth - 1) && id <= 34 {
                return id - depth;
            } else if id == 32 {
                return 34 - (depth - 1);
            }
        }

        0
    }

    /// 1-8 returns the next number
    /// 9 returns None for dora = false, 1 for dora = true
    /// honors return None for dora = false, honor order for dora = true
    pub fn next(&self, dora: bool) -> Option<Tile> {
        let new_color;

        match &self.tile_type {
            TileType::Number(number, color) => {
                new_color = *color;
                if *number < 9 {
                    Some(Tile::new(TileType::Number(number + 1, new_color)))
                } else if dora {
                    Some(Tile::new(TileType::Number(1, new_color)))
                } else {
                    None
                }
            }
            TileType::Wind(number) => {
                if !dora {
                    return None;
                }

                if *number < 4 {
                    Some(Tile::new(TileType::Wind(number + 1)))
                } else {
                    Some(Tile::new(TileType::Wind(1)))
                }
            }
            TileType::Dragon(number) => {
                if !dora {
                    return None;
                }

                if *number < 7 {
                    Some(Tile::new(TileType::Dragon(number + 1)))
                } else {
                    Some(Tile::new(TileType::Dragon(5)))
                }
            }
        }
    }

    pub fn prev(&self) -> Option<Tile> {
        let new_color;

        match &self.tile_type {
            TileType::Number(number, color) => {
                new_color = *color;
                if *number > 1 {
                    Some(Tile::new(TileType::Number(number - 1, new_color)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        match &self.tile_type {
            TileType::Number(number, _color) => *number == 1 || *number == 9,
            TileType::Wind(_) | TileType::Dragon(_) => false,
        }
    }

    pub fn is_honor(&self) -> bool {
        match &self.tile_type {
            TileType::Number(_, _) => false,
            TileType::Wind(_) | TileType::Dragon(_) => true,
        }
    }

    pub fn is_terminal_or_honor(&self) -> bool {
        self.is_terminal() || self.is_honor()
    }

    pub fn get_type_char(&self) -> char {
        match &self.tile_type {
            TileType::Number(_number, color) => color.to_char(),
            TileType::Wind(_number) => 'z',
            TileType::Dragon(_number) => 'z',
        }
    }

    pub fn get_value(&self) -> u8 {
        match &self.tile_type {
            TileType::Number(number, _color) => {
                if self.is_red {
                    return 0;
                }

                *number
            }
            TileType::Wind(number) => *number,
            TileType::Dragon(number) => *number,
        }
    }

    /// Returns an array of 3 values: type, color and number for this tile
    /// TODO red 5s
    fn get_ordering_values(&self) -> [u8; 3] {
        let self_type;
        let mut self_color = 0;
        let self_number: u8;

        match &self.tile_type {
            TileType::Number(number, color) => {
                self_type = 1;
                self_number = *number;
                self_color = match color {
                    TileColor::Manzu => 1,
                    TileColor::Pinzu => 2,
                    TileColor::Souzu => 3,
                };
            }
            TileType::Wind(number) => {
                self_type = 2;
                self_number = *number;
            }
            TileType::Dragon(number) => {
                self_type = 3;
                self_number = *number;
            }
        };

        [self_type, self_color, self_number]
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.tile_type {
                TileType::Number(mut number, color) => {
                    if self.is_red {
                        number = 0;
                    }
                    format!("{}{}", number, color)
                }
                TileType::Wind(number) => format!("{}z", number),
                TileType::Dragon(number) => format!("{}z", number),
            }
        )
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            tile_type: TileType::Dragon(1),
            is_red: false,
            is_draw: false,
            is_open: false,
            is_chi: false,
            is_pon: false,
            called_from: 0,
            is_kan: false,
            is_riichi: false,
            is_tsumogiri: false,
            id_136: None,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        let self_ord_values = self.get_ordering_values();
        let other_ord_values = other.get_ordering_values();

        self_ord_values[0] == other_ord_values[0]
            && self_ord_values[1] == other_ord_values[1]
            && self_ord_values[2] == other_ord_values[2]
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Tile) -> Option<Ordering> {
        let self_ord_values = self.get_ordering_values();
        let other_ord_values = other.get_ordering_values();

        if self_ord_values[0] < other_ord_values[0] {
            Some(Ordering::Less)
        } else if self_ord_values[0] > other_ord_values[0] {
            Some(Ordering::Greater)
        } else if self_ord_values[1] < other_ord_values[1] {
            Some(Ordering::Less)
        } else if self_ord_values[1] > other_ord_values[1] {
            Some(Ordering::Greater)
        } else if self_ord_values[2] < other_ord_values[2] {
            Some(Ordering::Less)
        } else if self_ord_values[2] > other_ord_values[2] {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for Tile {}

impl Ord for Tile {
    fn cmp(&self, other: &Tile) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl serde::Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string()[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_number_less_than_9() {
        let tile = Tile::new(TileType::Number(4, TileColor::Manzu));
        let next = tile.next(false);
        assert_eq!(next, Some(Tile::new(TileType::Number(5, TileColor::Manzu))));
    }

    #[test]
    fn next_number_9() {
        let tile = Tile::new(TileType::Number(9, TileColor::Manzu));
        let next = tile.next(false);
        assert!(next == None);
    }

    #[test]
    fn next_number_9_dora() {
        let tile = Tile::new(TileType::Number(9, TileColor::Manzu));
        let next = tile.next(true);
        assert!(next == Some(Tile::new(TileType::Number(1, TileColor::Manzu))));
    }

    #[test]
    fn next_wind_east() {
        let tile = Tile::new(TileType::Wind(1));
        let next = tile.next(false);
        assert!(next == None);
    }

    #[test]
    fn next_id_wind_dora() {
        let tile = Tile::new(TileType::Wind(4));
        let next = tile.next_id(true, 1);

        assert_eq!(next, 28);
    }

    #[test]
    fn next_id_dragon_dora() {
        let tile = Tile::new(TileType::Dragon(7));
        let next = tile.next_id(true, 1);

        assert_eq!(next, 31);
    }

    #[test]
    fn prev_id_wind_dora() {
        let tile = Tile::new(TileType::Wind(1));
        let prev = tile.prev_id(true, 1);

        assert_eq!(prev, 31);
    }

    #[test]
    fn prev_id_dragon_dora() {
        let tile = Tile::new(TileType::Dragon(5));
        let prev = tile.prev_id(true, 1);

        assert_eq!(prev, 34);
    }

    #[test]
    fn next_id_depth_2() {
        let tile = Tile::new(TileType::Number(7, TileColor::Manzu));
        let prev = tile.next_id(true, 2);

        assert_eq!(prev, 9);
    }

    #[test]
    fn prev_id_depth_2() {
        let tile = Tile::new(TileType::Number(9, TileColor::Manzu));
        let prev = tile.prev_id(true, 2);

        assert_eq!(prev, 7);
    }
}
