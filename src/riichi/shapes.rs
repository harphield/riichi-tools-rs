use super::tile::Tile;
use crate::riichi::tile::TileColor;

/// A hand consists of shapes.
/// A tenpai hand has (usually) only 1 incomplete shape.
/// Exceptions are for example 23456 wait, where you can either have 234 (complete) & 56 (incomplete), or 23 (incomplete) and 456 (complete)
/// Or, 13-sided kokushi, or 9-sided nine gates
pub struct Shape {
    shape_type: ShapeType,
    color: TileColor,
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

impl ShapeType {
    // pub fn get_shape(tile_ids : )

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
