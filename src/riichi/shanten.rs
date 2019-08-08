use super::tile::Tile;
use super::tile::TileType;
use super::tile::TileColor;

pub struct ShantenFinder {
    pairs: u8,
    triplets: u8,
    complete_melds: u8,
    incomplete_melds: u8,
    isolated_tiles: u8
}

impl ShantenFinder {
    pub fn new() -> ShantenFinder {
        ShantenFinder {
            ..Default::default()
        }
    }


}

impl Default for ShantenFinder {
    fn default() -> ShantenFinder {
        ShantenFinder {
            pairs: 0,
            triplets: 0,
            complete_melds: 0,
            incomplete_melds: 0,
            isolated_tiles: 0
        }
    }
}