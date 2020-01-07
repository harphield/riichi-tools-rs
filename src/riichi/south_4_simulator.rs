extern crate rand;

use rand::Rng;
use crate::riichi::scores::Score;

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
    pub fn evaluate(&self, direct_ron: (u8, u8), other_ron: (u8, u8), tsumo: (u8, u8)) -> ((bool, bool, bool), ((u8, u8), (u8, u8), (u8, u8))) {
        let point_difference = self.opponent_score - self.my_score + 100;

        let direct_ron_points = point_difference / 2; // opponent pays all, so I only need 1/2 of our point difference
        let mut tsumo_points = point_difference;

        let mut oya = false;

        if self.oya_state == 1 { // opponent will pay 1/4 of the winnings, so I only need to find 4/5 of the difference to beat them
            tsumo_points = (point_difference as f32 * (4f32 / 5f32)).ceil() as u32;
        } else if self.oya_state == 2 { // opponent will pay 1/3 of the winnings
            tsumo_points = (point_difference as f32 * (3f32 / 4f32)).ceil() as u32;
            oya = true;
        } else if self.oya_state == 3 { // opponent pays 1/2 of the winnings, so our difference changes accordingly
            tsumo_points = (point_difference as f32 * (2f32 / 3f32)).ceil() as u32;
        }

//        println!("pd {}", point_difference);
//        println!("tsumo points: {}", tsumo_points);

        let direct_ron_score = Score::new(direct_ron.0, direct_ron.1, oya, false);
        let direct_ron_correct_scores = Score::from_points(direct_ron_points, oya, false, false);
        let mut direct_ron_correct_points: u32 = 0;
        let mut direct_ron_correct_han: u8 = 0;
        let mut direct_ron_correct_fu: u8 = 0;
        match direct_ron_correct_scores {
            Some(scores) => {
                direct_ron_correct_points = scores[0].total_points();
                direct_ron_correct_han = scores[0].han;
                direct_ron_correct_fu = scores[0].fu;
            },
            None => ()
        }

        let other_ron_score = Score::new(other_ron.0, other_ron.1, oya, false);
        let other_ron_correct_scores = Score::from_points(point_difference, oya, false, false);
        let mut other_ron_correct_points: u32 = 0;
        let mut other_ron_correct_han: u8 = 0;
        let mut other_ron_correct_fu: u8 = 0;
        match other_ron_correct_scores {
            Some(scores) => {
                other_ron_correct_points = scores[0].total_points();
                other_ron_correct_han = scores[0].han;
                other_ron_correct_fu = scores[0].fu;
            },
            None => ()
        }

        let tsumo_score = Score::new(tsumo.0, tsumo.1, oya, true);
        let tsumo_correct_scores = Score::from_points(tsumo_points, oya, true, false);
        let mut tsumo_correct_points: u32 = 0;
        let mut tsumo_correct_han: u8 = 0;
        let mut tsumo_correct_fu: u8 = 0;
        match tsumo_correct_scores {
            Some(scores) => {
                tsumo_correct_points = scores[0].total_points();
                tsumo_correct_han = scores[0].han;
                tsumo_correct_fu = scores[0].fu;
            },
            None => ()
        }

//        println!("tsumo correct: {}, tsumo guessed: {}", tsumo_correct_points, tsumo_score.total_points());

        (
            (
                direct_ron_correct_points == direct_ron_score.total_points(),
                other_ron_correct_points == other_ron_score.total_points(),
                tsumo_correct_points == tsumo_score.total_points(),
            ),
            (
                (direct_ron_correct_han, direct_ron_correct_fu),
                (other_ron_correct_han, other_ron_correct_fu),
                (tsumo_correct_han, tsumo_correct_fu),
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let simulator = South4Simulator::new();

        assert!(simulator.my_score < simulator.opponent_score);
    }

    fn eval_7900() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 37900,
            oya_state: 1
        };

        let result = simulator.evaluate((2, 70), (5, 0), (4, 25));

//        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_7900_oya() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 37900,
            oya_state: 2
        };

        let result = simulator.evaluate((1, 90), (2, 90), (2, 60));

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_7900_opponent_oya() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 37900,
            oya_state: 3
        };

        let result = simulator.evaluate((2, 70), (5, 0), (2, 90));

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);

    }

    #[test]
    fn eval_16100_oya() {
        let simulator = South4Simulator {
            my_score: 33200,
            opponent_score: 49200,
            oya_state: 2
        };

        let result = simulator.evaluate((2, 90), (6, 0), (6, 0));

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }
}

