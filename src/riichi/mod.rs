#[cfg(feature = "fast_shanten")]
/// Fast shanten calculator
mod fast_hand_calculator;
/// Hand representation module
pub mod hand;
/// Defines the error struct
pub mod riichi_error;
/// Rules module
pub mod rules;
/// Score handling module
pub mod scores;
#[cfg(not(feature = "fast_shanten"))]
/// Slow shanten calculator
mod shanten;
/// Shapes detection module
mod shape_finder;
/// Shape representations module
pub mod shapes;
/// Table representation module
pub mod table;
/// Tile handling module
pub mod tile;
/// Yaku detection module
pub mod yaku;
