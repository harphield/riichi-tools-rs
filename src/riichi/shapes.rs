use super::tile::Tile;
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::tile::TileType::{Dragon, Number, Wind};
use core::fmt;
use std::fmt::Display;
use wasm_bindgen::__rt::core::fmt::Formatter;

/// A hand consists of shapes.
/// A tenpai hand has (usually) only 1 incomplete shape.
/// Exceptions are for example 23456 wait, where you can either have 234 (complete) & 56 (incomplete), or 23 (incomplete) and 456 (complete)
/// Or, shanpon wait (1155 = 11 pair 55 incomplete, or 11 incomplete 55 pair)
/// Or, 13-sided kokushi, or 9-sided nine gates
#[derive(Debug, Clone, Copy)]
pub struct Shape {
    shape_type: ShapeType,
    tile_count: u8,
    is_open: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Complete(CompleteShape),
    Incomplete(ClosedShape, Tile),
}

#[derive(Debug, Clone, Copy)]
pub enum CompleteShape {
    Closed(ClosedShape),
    Open(OpenShape),
}

#[derive(Debug, Clone, Copy)]
pub enum ClosedShape {
    // meld
    Shuntsu([Tile; 3]),
    // triplet
    Koutsu([Tile; 3]),
    // kan
    Kantsu([Tile; 4]),
    // pair
    Toitsu([Tile; 2]),
    Single(Tile),
}

#[derive(Debug, Clone, Copy)]
pub enum OpenShape {
    Chi([Tile; 3]),
    Pon([Tile; 3]),
    Kan(OpenKan),
}

#[derive(Debug, Clone, Copy)]
pub enum OpenKan {
    // kan opened by a call
    Daiminkan([Tile; 4]),
    // kan added to pon
    Shouminkan([Tile; 4]),
}

impl Shape {
    pub fn new(shape_type: ShapeType, tile_count: u8, is_open: bool) -> Shape {
        Shape {
            shape_type,
            tile_count,
            is_open,
        }
    }

    pub fn get_shape_type(&self) -> &ShapeType {
        &self.shape_type
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Creates a shape from the given tiles.
    /// TODO incomplete shapes
    pub fn from_tiles(
        tiles: &[Tile],
        is_open: bool,
        only_complete: bool,
    ) -> Result<Shape, RiichiError> {
        let shape_type: ShapeType;
        let tile_count: u8 = tiles.len() as u8;

        if !(1..=4).contains(&tile_count) {
            // 4 = kan?
            return Err(RiichiError::new(
                120,
                "Not a valid shape - wrong tile count",
            ));
        }

        // is this a valid shape?
        for i in 0..tile_count as usize {
            if i < tile_count as usize - 1 {
                if !Shape::are_in_shape(
                    tiles.get(i).unwrap().to_id(),
                    tiles.get(i + 1).unwrap().to_id(),
                ) {
                    return Err(RiichiError::new(
                        121,
                        "Not a valid shape - tiles are not relevant to each other",
                    ));
                }
            } else {
                break;
            }
        }

        // what type is this shape?
        // TODO kans
        if tile_count == 3 {
            let tile_1 = tiles.get(0).unwrap();
            let tile_2 = tiles.get(1).unwrap();
            let tile_3 = tiles.get(2).unwrap();

            return match tile_1.tile_type {
                Number(_value, _) => match tile_1.next(false) {
                    None => Shape::_koutsu_shape_type(tile_1, tile_2, tile_3, is_open),
                    Some(next_1) => match tile_2.next(false) {
                        None => Shape::_koutsu_shape_type(tile_1, tile_2, tile_3, is_open),
                        Some(next_2) => {
                            if tile_2.eq(&next_1) && tile_3.eq(&next_2) {
                                if is_open {
                                    shape_type =
                                        ShapeType::Complete(CompleteShape::Open(OpenShape::Chi([
                                            *tile_1, *tile_2, *tile_3,
                                        ])));
                                } else {
                                    shape_type = ShapeType::Complete(CompleteShape::Closed(
                                        ClosedShape::Shuntsu([*tile_1, *tile_2, *tile_3]),
                                    ));
                                }

                                Result::Ok(Shape::new(shape_type, tile_count, is_open))
                            } else {
                                Shape::_koutsu_shape_type(tile_1, tile_2, tile_3, is_open)
                            }
                        }
                    },
                },
                Wind(_value) => Shape::_koutsu_shape_type(tile_1, tile_2, tile_3, is_open),
                Dragon(_value) => Shape::_koutsu_shape_type(tile_1, tile_2, tile_3, is_open),
            };
        } else if tile_count == 2 && !is_open {
            let tile_1 = tiles.get(0).unwrap();
            let tile_2 = tiles.get(1).unwrap();

            if tile_1.eq(tile_2) && only_complete {
                shape_type = ShapeType::Complete(CompleteShape::Closed(ClosedShape::Toitsu([
                    *tile_1, *tile_2,
                ])));
                return Result::Ok(Shape::new(shape_type, tile_count, is_open));
            }
        } else if tile_count == 1 && !is_open {
            let tile_1 = tiles.get(0).unwrap();
            if only_complete {
                shape_type =
                    ShapeType::Complete(CompleteShape::Closed(ClosedShape::Single(*tile_1)));
                return Result::Ok(Shape::new(shape_type, tile_count, is_open));
            }
        }

        Err(RiichiError::new(124, "No suitable shape found"))
    }

    fn _koutsu_shape_type(
        tile_1: &Tile,
        tile_2: &Tile,
        tile_3: &Tile,
        is_open: bool,
    ) -> Result<Shape, RiichiError> {
        if tile_1.eq(tile_2) && tile_2.eq(tile_3) {
            let shape_type;
            if is_open {
                shape_type = ShapeType::Complete(CompleteShape::Open(OpenShape::Pon([
                    *tile_1, *tile_2, *tile_3,
                ])));
            } else {
                shape_type = ShapeType::Complete(CompleteShape::Closed(ClosedShape::Koutsu([
                    *tile_1, *tile_2, *tile_3,
                ])));
            }

            return Result::Ok(Shape::new(shape_type, 3, is_open));
        }

        Err(RiichiError::new(122, "Bad shape"))
    }

    /// Are these two tiles in a shape together?
    pub fn are_in_shape(first_tile_id: u8, second_tile_id: u8) -> bool {
        if !(1..=34).contains(&first_tile_id) || second_tile_id < 1 || second_tile_id > 34 {
            panic!("Wrong tile IDs");
        }

        // they can be in a Toitsu or a Koutsu
        if first_tile_id == second_tile_id {
            return true;
        }

        // different honor tiles are never in a shape
        if first_tile_id > 27 || second_tile_id > 27 {
            return false;
        }

        // they are manzu, pinzu or souzu
        let first_color;

        if first_tile_id <= 9 {
            first_color = 1;
        } else if first_tile_id <= 18 {
            first_color = 2;
        } else {
            first_color = 3;
        }

        let second_color;

        if second_tile_id <= 9 {
            second_color = 1;
        } else if second_tile_id <= 18 {
            second_color = 2;
        } else {
            second_color = 3;
        }

        // shapes need tiles in the same color
        if first_color != second_color {
            return false;
        }

        let diff = first_tile_id as i8 - second_tile_id as i8;

        // too far away
        if diff.abs() > 2 {
            return false;
        }

        true
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let closed_to_string = |closed| match closed {
            ClosedShape::Shuntsu(tiles) | ClosedShape::Koutsu(tiles) => format!(
                "{}{}{}",
                tiles[0].to_string(),
                tiles[1].to_string(),
                tiles[2].to_string()
            ),
            ClosedShape::Kantsu(tiles) => format!("(k{})", tiles[0].to_string(),),
            ClosedShape::Toitsu(tiles) => {
                format!("{}{}", tiles[0].to_string(), tiles[1].to_string())
            }
            ClosedShape::Single(tile) => tile.to_string(),
        };
        return write!(
            f,
            "{}",
            match &self.shape_type {
                ShapeType::Complete(cs) => match cs {
                    CompleteShape::Closed(closed) => closed_to_string(*closed),
                    CompleteShape::Open(open) => match open {
                        OpenShape::Chi(tiles) => {
                            let color = tiles[0].get_type_char();

                            let mut called_tile = 99;
                            for (i, t) in tiles.iter().enumerate() {
                                if t.called_from > 0 {
                                    called_tile = i;
                                    break;
                                }
                            }

                            if called_tile == 99 {
                                panic!("Invalid chi - which tile did we call?");
                            }

                            format!(
                                "({}{}{}{}{})",
                                tiles[0].get_value(),
                                tiles[1].get_value(),
                                tiles[2].get_value(),
                                color,
                                called_tile
                            )
                        }
                        OpenShape::Pon(tiles) => {
                            let mut called_from = 0;
                            for t in tiles.iter() {
                                if t.called_from > 0 {
                                    called_from = t.called_from;
                                    break;
                                }
                            }

                            if called_from == 0 {
                                panic!("Invalid pon - who did we call it from?");
                            }

                            format!("(p{}{})", tiles[0].to_string(), called_from)
                        }
                        OpenShape::Kan(open_kan) => {
                            let mut kan_type = 'k';
                            let tiles = match open_kan {
                                OpenKan::Daiminkan(tls) => tls,
                                OpenKan::Shouminkan(tls) => {
                                    kan_type = 's';
                                    tls
                                }
                            };

                            let mut called_from = 0;
                            for t in tiles.iter() {
                                if t.called_from > 0 {
                                    called_from = t.called_from;
                                    break;
                                }
                            }

                            if called_from == 0 {
                                panic!("Invalid kan - who did we call it from?");
                            }

                            format!("({}{}{})", kan_type, tiles[0].to_string(), called_from,)
                        }
                    },
                },
                ShapeType::Incomplete(closed, missing) =>
                    format!("{}{}", closed_to_string(*closed), missing.to_string()),
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::riichi::shapes::Shape;
    use crate::riichi::tile::Tile;

    #[test]
    fn from_ones() {
        let tile = Tile::from_text("1s").unwrap();
        let shape = Shape::from_tiles(&vec![tile, tile, tile], false, true);

        assert!(match shape {
            Ok(_) => true,
            Err(_) => false,
        });
    }

    #[test]
    fn from_nines() {
        let tile = Tile::from_text("9s").unwrap();
        let shape = Shape::from_tiles(&vec![tile, tile, tile], false, true);

        assert!(match shape {
            Ok(_) => true,
            Err(_) => false,
        });
    }
}
