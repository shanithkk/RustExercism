#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    curr_frame: u16,
    curr_roll: u16,
    max_roll: u16,
    pins_left: u16,
    score: u16,
    factors: [u16; 2],
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            curr_frame: 1,
            curr_roll: 1,
            max_roll: 2,
            pins_left: 10,
            score: 0,
            factors: [1; 2],
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.curr_roll > self.max_roll {
            if self.completed() {
                return Err(Error::GameComplete);
            } else {
                self.curr_frame += 1;
                self.curr_roll = 1;
                self.pins_left = 10;
            }
        }

        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.pins_left -= pins;
        let factor = self.factors[0];
        self.factors[0] = self.factors[1];
        self.factors[1] = 1;
        self.score += pins * factor;

        if self.pins_left == 0 {
            if self.curr_frame < 10 {
                match self.curr_roll {
                    1 => {
                        self.factors[0] += 1;
                        self.factors[1] += 1;
                        self.curr_roll = self.max_roll;
                    },
                    2 => {
                        self.factors[0] += 1;
                    },
                    _ => (),
                }
            } else {
                match self.curr_roll {
                    1 | 2 => {
                        self.max_roll = 3;
                        self.pins_left = 10;
                    },
                    _ => (),
                }
            }
        }

        self.curr_roll += 1;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.completed() {
            Some(self.score)
        } else {
            None
        }
    }

    fn completed(&self) -> bool {
        self.curr_frame == 10 && self.curr_roll > self.max_roll
    }
}