#[derive(Debug)]
pub struct Score {
    pub han: u8,
    pub fu: u8,
    oya: bool,
    tsumo: bool,
}

impl Score {
    pub fn new(han: u8, fu: u8, oya: bool, tsumo: bool) -> Score {
        let mut new_fu = fu;
        if fu != 25 {
            new_fu = ((fu as f32 / 10f32).ceil() * 10f32) as u8;
        }

        Score {
            han,
            fu: new_fu,
            oya,
            tsumo,
        }
    }

    /// Finds the first han + fu combination that reaches at least the value of points given.
    pub fn from_points(points: u32, oya: bool, tsumo: bool, fu_limit: u8) -> Option<Vec<Score>> {
        let base_points: f32 = if oya {
            points as f32 / 6f32
        } else {
            points as f32 / 4f32
        };

        //        println!("base points {}", base_points);

        // 8000 is base for yakuman - if it's more, this score can't be reached by a hand
        // TODO double yakumans?
        if base_points > 8000f32 {
            return None;
        }

        let mut scores = vec![];

        if base_points > 6000f32 {
            scores.push(Score::new(13, 0, oya, tsumo));
            return Some(scores);
        }

        if base_points > 4000f32 {
            scores.push(Score::new(11, 0, oya, tsumo));
            scores.push(Score::new(12, 0, oya, tsumo));
            return Some(scores);
        }

        if base_points > 3000f32 {
            scores.push(Score::new(8, 0, oya, tsumo));
            scores.push(Score::new(9, 0, oya, tsumo));
            scores.push(Score::new(10, 0, oya, tsumo));
            return Some(scores);
        }

        if base_points > 2000f32 {
            scores.push(Score::new(6, 0, oya, tsumo));
            scores.push(Score::new(7, 0, oya, tsumo));
            return Some(scores);
        }

        if base_points > 1920f32 {
            if fu_limit > 60 {
                scores.push(Score::new(3, 70, oya, tsumo));
            }
            scores.push(Score::new(4, 40, oya, tsumo));
            scores.push(Score::new(5, 0, oya, tsumo));
            return Some(scores);
        }

        let mut done = false;
        for mut han in 1..4 {
            let mut fu = 20;

            // 1 han hands and rons can't have 20 fu
            if han == 1 || !tsumo {
                fu = 30;
            }

            while fu <= fu_limit {
                let s = Score::new(han, fu, oya, tsumo);
                if s.total_points() >= points {
                    // found!
                    scores.push(s);
                    done = true;

                    loop {
                        han += 1;
                        fu = fu / 2;
                        if fu >= 20 && (fu == 25 || fu as f32 % 10f32 == 0f32) {
                            // fu 20 can only be achieved with a tsumo
                            if fu == 20 && !tsumo {
                                break;
                            }

                            scores.push(Score::new(han, fu, oya, tsumo));

                            if fu == 25 {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    break;
                }

                fu += 10;
            }

            if done {
                break;
            }
        }

        if !scores.is_empty() {
            return Some(scores);
        }

        None
    }

    /// Returns the base points of this Score. See http://arcturus.su/wiki/Japanese_mahjong_scoring_rules#Scoring_procedure
    fn base_points(&self) -> u32 {
        if self.han == 5 || (self.han == 3 && self.fu >= 70) || (self.han == 4 && self.fu >= 40) {
            // mangan
            return 2000;
        } else if self.han >= 6 && self.han <= 7 {
            // haneman
            return 3000;
        } else if self.han >= 8 && self.han <= 10 {
            // baiman
            return 4000;
        } else if self.han >= 11 && self.han <= 12 {
            // sanbaiman
            return 6000;
        } else if self.han >= 13 {
            // yakuman
            // TODO double yakuman?
            return 8000;
        }

        self.fu as u32 * (2u32.pow(2u32 + self.han as u32))
    }

    /// Returns total points that will be distributed from this Score
    pub fn total_points(&self) -> u32 {
        let base_points = self.base_points();

        if self.han == 5 || (self.han == 3 && self.fu >= 70) || (self.han == 4 && self.fu >= 40) {
            // mangan
            if self.oya {
                12000
            } else {
                8000
            }
        } else if self.han >= 6 && self.han <= 7 {
            // haneman
            if self.oya {
                18000
            } else {
                12000
            }
        } else if self.han >= 8 && self.han <= 10 {
            // baiman
            if self.oya {
                24000
            } else {
                16000
            }
        } else if self.han >= 11 && self.han <= 12 {
            // sanbaiman
            if self.oya {
                36000
            } else {
                24000
            }
        } else if self.han >= 13 {
            // yakuman
            if self.oya {
                48000
            } else {
                32000
            }
        } else {
            if self.oya {
                if self.tsumo {
                    ((((base_points * 2) as f32 / 100f32).ceil() * 100f32) * 3f32) as u32
                } else {
                    (((base_points * 6) as f32 / 100f32).ceil() * 100f32) as u32
                }
            } else if self.tsumo {
                ((((base_points as f32 / 100f32).ceil() * 100f32) * 2f32)
                    + (((base_points * 2) as f32 / 100f32).ceil() * 100f32)) as u32
            } else {
                (((base_points * 4) as f32 / 100f32).ceil() * 100f32) as u32
            }
        }
    }

    /// How many points will oya pay from this Score?
    pub fn points_from_oya(&self) -> u32 {
        let base_points = self.base_points();

        if self.tsumo {
            return (((2 * base_points) as f32 / 100f32).ceil() * 100f32) as u32;
        }

        (((4 * base_points) as f32 / 100f32).ceil() * 100f32) as u32
    }

    /// How many points will non-oya pay from this Score?
    pub fn points_from_ko(&self) -> u32 {
        let base_points = self.base_points();

        if self.tsumo {
            if self.oya {
                return (((2 * base_points) as f32 / 100f32).ceil() * 100f32) as u32;
            }
            return (((base_points) as f32 / 100f32).ceil() * 100f32) as u32;
        }

        if self.oya {
            return (((6 * base_points) as f32 / 100f32).ceil() * 100f32) as u32;
        }

        (((4 * base_points) as f32 / 100f32).ceil() * 100f32) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_1_30() {
        let score = Score::new(1, 30, false, false);
        assert_eq!(score.total_points(), 1000);
    }

    #[test]
    fn score_1_30_oya() {
        let score = Score::new(1, 30, true, false);
        assert_eq!(score.total_points(), 1500);
    }

    #[test]
    fn score_1_30_tsumo() {
        let score = Score::new(1, 30, false, true);
        assert_eq!(score.total_points(), 1100);
    }

    #[test]
    fn score_1_30_oya_tsumo() {
        let score = Score::new(1, 30, true, true);
        assert_eq!(score.total_points(), 1500);
    }

    #[test]
    fn score_1_40() {
        let score = Score::new(1, 40, false, false);
        assert_eq!(score.total_points(), 1300);
    }

    #[test]
    fn score_1_40_oya() {
        let score = Score::new(1, 40, true, false);
        assert_eq!(score.total_points(), 2000);
    }

    #[test]
    fn score_1_40_tsumo() {
        let score = Score::new(1, 40, false, true);
        assert_eq!(score.total_points(), 1500);
    }

    #[test]
    fn score_1_40_oya_tsumo() {
        let score = Score::new(1, 40, true, true);
        assert_eq!(score.total_points(), 2100);
    }

    #[test]
    fn score_4_30() {
        let score = Score::new(4, 30, false, false);
        assert_eq!(score.total_points(), 7700);
    }

    #[test]
    fn score_3_50() {
        let score = Score::new(3, 70, false, false);
        assert_eq!(score.total_points(), 8000);
    }

    #[test]
    fn score_4_25() {
        let score = Score::new(4, 25, false, false);
        assert_eq!(score.total_points(), 6400);
    }

    #[test]
    fn oya_points_1_30() {
        let score = Score::new(1, 30, false, false);
        assert_eq!(score.points_from_oya(), 1000);
    }

    #[test]
    fn oya_points_1_30_tsumo() {
        let score = Score::new(1, 30, false, true);
        assert_eq!(score.points_from_oya(), 500);
    }

    #[test]
    fn ko_points_1_30() {
        let score = Score::new(1, 30, false, false);
        assert_eq!(score.points_from_ko(), 1000);
    }

    #[test]
    fn ko_points_1_30_tsumo() {
        let score = Score::new(1, 30, false, true);
        assert_eq!(score.points_from_ko(), 300);
    }

    #[test]
    fn from_points_2000() {
        let scores = Score::from_points(2000, false, false, 60).unwrap();

        assert_eq!(scores.len(), 2);

        assert_eq!(scores[0].han, 1);
        assert_eq!(scores[0].fu, 60);

        assert_eq!(scores[1].han, 2);
        assert_eq!(scores[1].fu, 30);
    }

    #[test]
    fn from_points_3800() {
        let scores = Score::from_points(3800, false, false, 60).unwrap();

        assert_eq!(scores.len(), 2);

        assert_eq!(scores[0].han, 2);
        assert_eq!(scores[0].fu, 60);

        assert_eq!(scores[1].han, 3);
        assert_eq!(scores[1].fu, 30);
    }

    #[test]
    fn from_points_3800_oya() {
        let scores = Score::from_points(3800, true, false, 60).unwrap();

        //        println!("{:#?}", scores);

        assert_eq!(scores.len(), 1);

        assert_eq!(scores[0].han, 2);
        assert_eq!(scores[0].fu, 40);
    }

    #[test]
    fn from_points_3800_oya_tsumo() {
        let scores = Score::from_points(3800, true, true, 60).unwrap();

        //        println!("{:#?}", scores);

        assert_eq!(scores.len(), 2);

        assert_eq!(scores[0].han, 2);
        assert_eq!(scores[0].fu, 40);

        assert_eq!(scores[1].han, 3);
        assert_eq!(scores[1].fu, 20);
    }

    #[test]
    fn from_points_4000() {
        let scores = Score::from_points(4000, false, false, 60).unwrap();

        //        println!("{:#?}", scores);

        assert_eq!(scores.len(), 1);

        assert_eq!(scores[0].han, 3);
        assert_eq!(scores[0].fu, 40);
    }

    #[test]
    fn from_points_7600() {
        let scores = Score::from_points(7600, false, false, 60).unwrap();

        //        println!("{:#?}", scores);

        assert_eq!(scores.len(), 2);

        assert_eq!(scores[0].han, 3);
        assert_eq!(scores[0].fu, 60);

        assert_eq!(scores[1].han, 4);
        assert_eq!(scores[1].fu, 30);
    }

    #[test]
    fn from_points_8000() {
        let scores = Score::from_points(8000, false, false, 60).unwrap();

        println!("{:#?}", scores);

        assert_eq!(scores.len(), 2);

        assert_eq!(scores[0].han, 4);
        assert_eq!(scores[0].fu, 40);

        assert_eq!(scores[1].han, 5);
        assert_eq!(scores[1].fu, 0);
    }

    #[test]
    fn create_score_1_22() {
        let score = Score::new(1, 22, false, true);
        assert_eq!(score.han, 1);
        assert_eq!(score.fu, 30);
    }
}
