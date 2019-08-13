use std::fmt;
use std::cmp::Ordering;

// 'm0', 'm1', 'm2', 'm3', 'm4', 'm5', 'm6', 'm7', 'm8', 'm9',
// 'p0', 'p1', 'p2', 'p3', 'p4', 'p5', 'p6', 'p7', 'p8', 'p9',
// 's0', 's1', 's2', 's3', 's4', 's5', 's6', 's7', 's8', 's9',
//  E    S    W     N
// 'z1', 'z2', 'z3', 'z4',
//  W    G     R
// 'z5', 'z6', 'z7'

#[derive(Debug)]
pub enum TileType {
    Number(u8, TileColor),
    Wind(u8),
    Dragon(u8)
}

impl TileType {
    pub fn to_char(&self) -> char {
        match &self {
            TileType::Number(number, color) => color.to_char(),
            TileType::Wind(number) => 'z',
            TileType::Dragon(number) => 'z'
        }
    }
}

#[derive(Debug, Clone)]
pub enum TileColor {
    Manzu,
    Pinzu,
    Souzu
}

impl TileColor {
    pub fn from_char(rep : &char) -> TileColor {
        match rep {
            'm' => TileColor::Manzu,
            'p' => TileColor::Pinzu,
            's' => TileColor::Souzu,
            _ => panic!("Wrong representation of tile color!")
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

#[derive(Debug)]
pub struct Tile {
    pub tile_type: TileType,
    is_red: bool,
    is_open: bool,
    is_draw: bool,
    is_chi: bool,
    is_pon: bool,
    is_kan: bool
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        match &tile_type {
            TileType::Number(number, color) => {
                if *number > 9 {
                    panic!("Numbers can be only up to 9");
                }
            },
            TileType::Wind(number) => {
                if *number > 4 {
                    panic!("Winds can be only up to 4");
                }
            },
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

    pub fn from_text(representation: &str) -> Tile {
        if representation.len() != 2 {
            panic!("Tile length must be 2");
        }

        let mut r_chars = representation.chars();
        let first_char = &r_chars.next().unwrap();
        let second_char = &r_chars.next().unwrap();
        let number = second_char.to_string().parse().unwrap();

        if ['m', 'p', 's'].contains(first_char) {
            let color : TileColor;
            if *first_char == 'm' {
                color = TileColor::Manzu;
            } else if *first_char == 'p' {
                color = TileColor::Pinzu;
            } else if *first_char == 's' {
                color = TileColor::Souzu;
            } else {
                panic!("Wrong color, only m, p an s allowed");
            }

            Tile::new(TileType::Number(number, color))
        } else if *first_char == 'z' {
            if number > 0 && number <= 4 {
                // winds
                Tile::new(TileType::Wind(number))
            } else if number > 4 && number <= 7 {
                // dragons
                Tile::new(TileType::Dragon(number))
            } else {
                panic!("Wrong number for honors!");
            }
        } else {
            panic!("Invalid tile definition");
        }
    }

    /// id is an integer value > 0 of a tile.
    /// The order is Manzu - Pinzu - Souzu - Winds - Dragons
    pub fn from_id(id: u8) -> Tile {
        if id < 1 || id > 34 {
            panic!("Wrong tile ID {}", id);
        }

        if id <= 9 {
            return Tile::new(TileType::Number(id, TileColor::Manzu));
        }

        if id <= 18 {
            return Tile::new(TileType::Number(id - 9, TileColor::Pinzu));
        }

        if id <= 27 {
            return Tile::new(TileType::Number(id - 18, TileColor::Souzu));
        }

        if id <= 31 {
            return Tile::new(TileType::Wind(id - 27));
        }

        return Tile::new(TileType::Dragon(id - 27));
    }

    /// Gets the id of this tile based on its type
    pub fn to_id(&self) -> u8 {
        match &self.tile_type {
            TileType::Number(number, color) => {
                match color {
                    TileColor::Manzu => number + 0, // + dereferences?
                    TileColor::Pinzu => number + 9,
                    TileColor::Souzu => number + 18
                }
            },
            TileType::Wind(number) => {
                number + 27
            },
            TileType::Dragon(number) => {
                number + 27
            }
        }
    }

    /// Returns an ID of the next tile in order.
    pub fn next_id(&self, dora: bool) -> u8 {
        let id = self.to_id();

        // manzu
        if id < 9 {
            return id + 1;
        }

        if id == 9 && !dora {
            return 0;
        }

        if id == 9 && dora {
            return 1;
        }

        // pinzu
        if id < 18 {
            return id + 1;
        }

        if id == 18 && !dora {
            return 0;
        }

        if id == 18 && dora {
            return 10;
        }

        // souzu
        if id < 27 {
            return id + 1;
        }

        if id == 27 && !dora {
            return 0;
        }

        if id == 27 && dora {
            return 19;
        }

        // dragons
        if dora {
            if id < 34 {
                return id + 1;
            }

            return 28;
        }

        return 0;
    }

    /// 1-8 returns the next number
    /// 9 returns None for dora = false, 1 for dora = true
    /// honors return None for dora = false, honor order for dora = true
    pub fn next(&self, dora: bool) -> Option<Tile> {
        let new_color;

        match &self.tile_type {
            TileType::Number(number, color) => {
                new_color = color.clone();
                if *number < 9 {
                    return Some(Tile::new(TileType::Number(number + 1, new_color)));
                } else if dora {
                    return Some(Tile::new(TileType::Number(1, new_color)));
                } else {
                    return None;
                }
            },
            TileType::Wind(number) => {
                if !dora {
                    return None;
                }

                if *number < 4 {
                    return Some(Tile::new(TileType::Wind(number + 1)));
                } else {
                    return Some(Tile::new(TileType::Wind(1)));
                }
            },
            TileType::Dragon(number) => {
                if !dora {
                    return None;
                }

                if *number < 7 {
                    return Some(Tile::new(TileType::Dragon(number + 1)));
                } else {
                    return Some(Tile::new(TileType::Dragon(5)));
                }
            }
        }
    }

    // pub fn prev(&self, dora: bool) -> Option(Tile) {

    // }

    pub fn to_string(&self) -> String {
        match &self.tile_type {
            TileType::Number(number, color) => {
                format!("{}{}", number, color)
            },
            TileType::Wind(number) => {
                format!("{}z", number)
            },
            TileType::Dragon(number) => {
                format!("{}z", number)
            }
        }
    }

    pub fn get_type_char(&self) -> char {
        match &self.tile_type {
            TileType::Number(number, color) => color.to_char(),
            TileType::Wind(number) => 'z',
            TileType::Dragon(number) => 'z'
        }
    }

    pub fn get_value(&self) -> u8 {
        match &self.tile_type {
            TileType::Number(number, color) => *number,
            TileType::Wind(number) => *number,
            TileType::Dragon(number) => *number
        }
    }

    fn get_ordering_values(&self) -> [u8; 3] {
        let self_type;
        let mut self_color = 0;
        let self_number : u8;        

        match &self.tile_type {
            TileType::Number(number, color) => {
                self_type = 1;
                self_number = *number;
                self_color = match color {
                    TileColor::Manzu => 1,
                    TileColor::Pinzu => 2,
                    TileColor::Souzu => 3
                };                
            },
            TileType::Wind(number) => {
                self_type = 2;
                self_number = *number;
            },
            TileType::Dragon(number) => {
                self_type = 3;
                self_number = *number;                
            }
        };

        [self_type, self_color, self_number]
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
            is_kan: false
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        let self_ord_values = self.get_ordering_values();
        let other_ord_values = other.get_ordering_values();

        self_ord_values[0] == other_ord_values[0] && self_ord_values[1] == other_ord_values[1] && self_ord_values[2] == other_ord_values[2]
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Tile) -> Option<Ordering> {
        let self_ord_values = self.get_ordering_values();
        let other_ord_values = other.get_ordering_values();

        if self_ord_values[0] < other_ord_values[0] {
            return Some(Ordering::Less);
        } else if self_ord_values[0] > other_ord_values[0] {
            return Some(Ordering::Greater);
        } else if self_ord_values[1] < other_ord_values[1] {
            return Some(Ordering::Less);
        } else if self_ord_values[1] > other_ord_values[1] {
            return Some(Ordering::Greater);
        } else if self_ord_values[2] < other_ord_values[2] {
            return Some(Ordering::Less);
        } else if self_ord_values[2] > other_ord_values[2] {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Equal);
        }
    }
}

impl Eq for Tile {

}

impl Ord for Tile {
    fn cmp(&self, other: &Tile) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_number_less_than_9() {
        let tile = Tile::new(TileType::Number(4, TileColor::Manzu));
        let next = tile.next(false);
        assert!(next == Some(Tile::new(TileType::Number(5, TileColor::Manzu))));
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
}