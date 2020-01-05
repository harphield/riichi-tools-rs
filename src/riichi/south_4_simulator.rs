extern crate rand;

use rand::Rng;

/// TODO honba
/// TODO riichi sticks
pub struct South4Simulator {
    pub my_score: u32,
    pub opponent_score: u32,
    /// 1 = no one is oya
    /// 2 = I am oya
    /// 3 = opponent is oya
    pub oya_state: u8,
}

impl South4Simulator {
    pub fn new() -> South4Simulator {
        let points = South4Simulator::generate_players_state();
        let mut rng = rand::thread_rng();

        South4Simulator {
            my_score: points.0,
            opponent_score: points.1,
            oya_state: rng.gen_range(1, 4)
        }
    }

    /// Creates 2 scores, where you are in second place by points that can be reached in 1 hand
    fn generate_players_state() -> (u32, u32) {
        let mut rng = rand::thread_rng();

        let my_score = 30000 + 100 * rng.gen_range(-100i32, 50);
        let first_place_score = my_score + 100 * rng.gen_range(10, 320);

        (my_score as u32, first_place_score as u32)
    }

    /// Checks if the player correctly estimated the hands they need.
    /// Player must always check for point differences with a direct hit, non-direct ron and a tsumo win.
    pub fn evaluate(&self, direct_ron: (u8, u8), other_ron: (u8, u8), tsumo: (u8, u8)) {
        let point_difference = self.opponent_score - self.my_score + 100;

        let direct_ron_points = point_difference / 2; // opponent pays all, so I only need 1/2 of our point difference
        let mut tsumo_points = point_difference;

        if self.oya_state == 1 { // opponent will pay 1/4 of the winnings, so I only need to find 3/4 of the difference to beat them
            tsumo_points -= tsumo_points / 4;
        } else if self.oya_state == 2 { // opponent will pay 1/3 of the winnings
            tsumo_points = point_difference * 3 / 4;
        } else if self.oya_state == 3 { // opponent pays 1/2 of the winnings, so our difference changes accordingly
            tsumo_points = (point_difference as f64 / 1.5).ceil() as u32;
        }

        println!("tsumo points: {}", tsumo_points);

//        w + w/3 = 8000
//        4/3w = 8000
//            4w = 8000 * 3
//                w = 8000 * 3 / 4
//        w = 8000 / 1.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_16000() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 37900,
            oya_state: 2
        };

        simulator.evaluate((2, 70), (5, 0), (4, 25));
    }
}

