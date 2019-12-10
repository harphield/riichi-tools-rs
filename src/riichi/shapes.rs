use super::tile::Tile;
use crate::riichi::tile::TileColor;
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::tile::TileType::{Number, Wind, Dragon};
use crate::riichi::shapes::ShapeType::Complete;

/// A hand consists of shapes.
/// A tenpai hand has (usually) only 1 incomplete shape.
/// Exceptions are for example 23456 wait, where you can either have 234 (complete) & 56 (incomplete), or 23 (incomplete) and 456 (complete)
/// Or, 13-sided kokushi, or 9-sided nine gates
pub struct Shape {
    shape_type: ShapeType,
    tile_count: u8,
}

pub enum ShapeType {
    Complete(CompleteShape),
    Incomplete(IncompleteShape)
}

pub enum CompleteShape {
    // meld
    Shuntsu([Tile; 3]),
    // triplet
    Koutsu([Tile; 3]),
    // pair
    Toitsu([Tile; 2]),
    Single(Tile)
}

pub enum IncompleteShape {
    TwoDifferent([Tile; 2]),
    Shanpon([Tile; 2]),
    Tanki(Tile)
}

impl Shape {
    pub fn new(shape_type: ShapeType, tile_count: u8) -> Shape {
        Shape {
            shape_type,
            tile_count
        }
    }

    pub fn from_tiles(tiles: Vec<Tile>) -> Result<Shape, RiichiError> {
        let shape_type: ShapeType;
        let tile_count: u8 = tiles.count() as u8;

        if tile_count < 1 || tile_count > 4 { // 4 = kan?
            return Err(RiichiError::new(120, "Not a valid shape - wrong tile count"));
        }

        // is this a valid shape?
        for i in 0..tile_count {
            if i < tile_count - 1 {
                if !Shape::are_in_shape(tiles.get(i).unwrap().to_id(), tiles.get(i + 1).unwrap().to_id()) {
                    return Err(RiichiError::new(121, "Not a valid shape - tiles are not relevant to each other"));
                }
            } else {
                break;
            }
        }

        // what type is this shape?
        // TODO kans
        if tile_count == 3 {
            let tile_1 = tiles.get(0).unwrap();
            match tile_1.tile_type {
                Number(value, color) => {
                    let tile_2 = tiles.get(1).unwrap();
                    let tile_3 = tiles.get(2).unwrap();
                    if tile_2.eq(tile_1.next(false).unwrap().as_ref()) &&
                        tile_3.eq(tile_2.next(false).unwrap().as_ref()) {
                        shape_type = ShapeType::Complete(CompleteShape::Shuntsu([
                            *tile_1, tile_2, tile_3
                        ]));
                    }
                }
                Wind(value) => {
                    shape_type = ShapeType::Complete(CompleteShape::Koutsu([
                        *tile_1, tiles.get(1).unwrap(), tiles.get(2).unwrap()
                    ]));
                }
                Dragon(value) => {
                    shape_type = ShapeType::Complete(CompleteShape::Koutsu([
                        *tile_1, tiles.get(1).unwrap(), tiles.get(2).unwrap()
                    ]));
                }
            }
        }

        Result::Ok(Shape::new(shape_type, tile_count))
    }

    /// Are these two tiles in a shape together?
    pub fn are_in_shape(first_tile_id : u8, second_tile_id : u8) -> bool {
        if first_tile_id < 1 || first_tile_id > 34 || second_tile_id < 1 || second_tile_id > 34 {
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

        let diff = (first_tile_id - second_tile_id) as i8;

        // too far away
        if diff.abs() > 2 {
            return false;
        }

        true
    }
}
