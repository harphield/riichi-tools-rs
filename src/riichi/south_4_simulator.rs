extern crate rand;

use rand::Rng;
use crate::riichi::scores::Score;

pub struct South4Simulator {
    pub my_score: u32,
    pub opponent_score: u32,
    /// 1 = no one is oya
    /// 2 = I am oya
    /// 3 = opponent is oya
    pub oya_state: u8,
    pub riichi_sticks: u8,
    pub tsumibo: u8,
}

impl South4Simulator {
    pub fn new() -> South4Simulator {
        let points = South4Simulator::generate_players_state();
        let mut rng = rand::thread_rng();

        // I want the chance of getting more sticks to be lower
        let sticks_roll = [0,0,0,0,1,1,2,3];
        let tsumibo_roll = [0,0,0,1,1,2,2,3,4,5];

        South4Simulator {
            my_score: points.0,
            opponent_score: points.1,
            oya_state: rng.gen_range(1, 4),
            riichi_sticks: *sticks_roll.get(rng.gen_range(0, sticks_roll.len())).unwrap() as u8,
            tsumibo: *tsumibo_roll.get(rng.gen_range(0, tsumibo_roll.len())).unwrap() as u8,
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
    pub fn evaluate(&self, direct_ron: (u8, u8), other_ron: (u8, u8), tsumo: (u8, u8), hard_mode: bool) -> ((bool, bool, bool), ((u8, u8), (u8, u8), (u8, u8))) {
        let mut point_difference: i32 = (self.opponent_score as i32 - self.my_score as i32 + 100 - (1000 * self.riichi_sticks as i32) - (300 * self.tsumibo as i32)) as i32;

        let mut fu_limit = 60;
        if hard_mode {
            fu_limit = 110;
        }

        if point_difference <= 0 {
            point_difference = 100; // you can just win with whatever
        }

        let direct_ron_points = (point_difference / 2) as u32; // opponent pays all, so I only need 1/2 of our point difference
        let mut tsumo_points = point_difference as u32 - (self.tsumibo * 100) as u32; // the leader pays 100 points per stick

        let mut oya = false;

        if self.oya_state == 1 { // opponent will pay 1/4 of the winnings, so I only need to find 4/5 of the difference to beat them
            tsumo_points = (tsumo_points as f32 * (4f32 / 5f32)).ceil() as u32;
        } else if self.oya_state == 2 { // opponent will pay 1/3 of the winnings
            tsumo_points = (tsumo_points as f32 * (3f32 / 4f32)).ceil() as u32;
            oya = true;
        } else if self.oya_state == 3 { // opponent pays 1/2 of the winnings, so our difference changes accordingly
            tsumo_points = (tsumo_points as f32 * (2f32 / 3f32)).ceil() as u32;
        }

//        println!("pd {}", point_difference);
//        println!("tsumo points: {}", tsumo_points);

        let direct_ron_score = Score::new(direct_ron.0, direct_ron.1, oya, false);
        let direct_ron_correct_scores = Score::from_points(direct_ron_points, oya, false, fu_limit);
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
        let other_ron_correct_scores = Score::from_points(point_difference as u32, oya, false, fu_limit);
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
        let tsumo_correct_scores = Score::from_points(tsumo_points, oya, true, fu_limit);
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
            oya_state: 1,
            riichi_sticks: 0,
            tsumibo: 0,
        };

        let result = simulator.evaluate((2, 70), (5, 0), (4, 25), true);

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
            oya_state: 2,
            riichi_sticks: 0,
            tsumibo: 0,
        };

        let result = simulator.evaluate((2, 50), (3, 50), (3, 30), false);

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
            oya_state: 3,
            riichi_sticks: 0,
            tsumibo: 0,
        };

        let result = simulator.evaluate((3, 40), (5, 0), (3, 50), false);

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
            oya_state: 2,
            riichi_sticks: 0,
            tsumibo: 0,
        };

        let result = simulator.evaluate((2, 90), (6, 0), (6, 0), true);

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_8900_with_riichi_stick() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 38900,
            oya_state: 1,
            riichi_sticks: 1,
            tsumibo: 0,
        };

        let result = simulator.evaluate((2, 70), (5, 0), (4, 25), true);

//        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_9500_with_riichi_stick_2_tsumibo() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 39500,
            oya_state: 1,
            riichi_sticks: 1,
            tsumibo: 2,
        };

        let result = simulator.evaluate((3, 40), (5, 0), (4, 25), false);

//        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_100() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 30100,
            oya_state: 1,
            riichi_sticks: 0,
            tsumibo: 0,
        };

        let result = simulator.evaluate((1, 30), (1, 30), (1, 30), false);

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_negative() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 32000,
            oya_state: 1,
            riichi_sticks: 2,
            tsumibo: 1,
        };

        let result = simulator.evaluate((1, 30), (1, 30), (1, 30), false);

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }

    #[test]
    fn eval_tsumo_with_sticks() {
        let simulator = South4Simulator {
            my_score: 30000,
            opponent_score: 40300,
            oya_state: 1,
            riichi_sticks: 0,
            tsumibo: 1,
        };

        let result = simulator.evaluate((3, 40), (6, 0), (5, 0), false);

        println!("{:#?}", result);

        assert_eq!({result.0}.0, true);
        assert_eq!({result.0}.1, true);
        assert_eq!({result.0}.2, true);
    }
}

