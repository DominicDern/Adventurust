extern crate rand;
use rand::Rng;

pub struct IndividualRoll {
    number_of_rolls: Option<u8>,
    die_size: u8,
    modifier: Option<i16>,
}

impl IndividualRoll {
    pub fn new(number_of_rolls: Option<u8>, die_size: u8, modifier: Option<i16>) -> IndividualRoll {
        IndividualRoll {
            number_of_rolls,
            die_size,
            modifier,
        }
    }
}

impl IndividualRoll {
    pub fn roll(self) -> u32 {
        let mut total: u32 = 0;
        match self.number_of_rolls {
            Some(number_of_rolls) => {
                for _ in 0..number_of_rolls {
                    let mut rng = rand::thread_rng();
                    total += rng.gen_range(0..(self.die_size + 1)) as u32;
                }
                if let Some(modifier) = self.modifier {
                    total += modifier as u32
                }
            }
            None => {
                let mut rng = rand::thread_rng();
                total += rng.gen_range(0..(self.die_size + 1)) as u32;
            }
        }
        total
    }
}

pub struct Roll {
    roll_vec: Vec<IndividualRoll>,
}

impl Roll {
    fn parse_roll(roll: &str) -> Roll {
        let mut roll_vec: Vec<IndividualRoll> = Vec::new();

        let roll: Vec<&str> = roll.split(',').collect();
        for roll in roll.clone() {
            let number_of_rolls: Option<u8>;
            let die_size: u8;

            let mut roll: Vec<&str> = roll.split('+').collect();
            let modifier = if roll.len() == 2 {
                Some(roll[1].trim().parse().unwrap())
            } else {
                None
            };
            if roll.len() > 1 {
                roll.pop();
            }
            let roll = String::from(roll[0]);

            let mut roll: Vec<&str> = roll.split('d').collect();
            roll[0] = roll[0].trim();
            if roll[0].is_empty() {
                number_of_rolls = None;
                die_size = roll[1].trim().parse().unwrap();
            } else {
                number_of_rolls = Some(roll[0].trim().parse().unwrap());
                die_size = roll[1].trim().parse().unwrap();
            }
            let roll = IndividualRoll::new(number_of_rolls, die_size, modifier);
            roll_vec.push(roll);
        }
        Roll::new(roll_vec)
    }

    fn new(roll_vec: Vec<IndividualRoll>) -> Roll {
        Roll { roll_vec }
    }

    pub fn roll(roll: &str) -> u32 {
        let roll = self::Roll::parse_roll(roll);
        let mut total: u32 = 0;
        for roll in roll.roll_vec {
            total += roll.roll();
        }
        total
    }
}
