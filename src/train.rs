use crate::{
    engine::eval,
    small::{SmallBattleSnake, SmallMove, Status},
    types::{Coordinate, Direction, State, Weights},
};
use fastrand::i32;
use rand::thread_rng;
use rand::{seq::SliceRandom, Rng};
use std::time::Instant;
use tinyvec::array_vec;

macro_rules! newc {
    ($x:expr, $y:expr) => {
        Coordinate::new($x, $y)
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trainer {
    pub variants: Vec<Variant>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Variant {
    pub genome: Weights,
    pub elo: i64,
}

impl Variant {
    /// create a certain number of children based on the max variance and also my own genome.
    pub fn create_children(&self, num_children: u8, max_variance: u8) -> Vec<Variant> {
        let mut out = vec![];
        for _ in 0..num_children {
            out.push(Variant::new(self.genome.randomize(max_variance as i32)));
        }
        out
    }

    /// create a new version of myself.
    pub fn new(genome: Weights) -> Self {
        Variant { genome, elo: 0 }
    }
    /// register a win in terms of elo gain.
    pub fn reg_win(&mut self) {
        self.elo += 5;
    }
    /// register a loss in terms of elo loss.
    pub fn reg_loss(&mut self) {
        self.elo -= 3;
    }
}

impl Weights {
    /// return a randomized version of myself based on the variance.
    pub fn randomize(&self, max_variance: i32) -> Self {
        Weights(
            self.0 + i32(-max_variance..max_variance),
            self.1 + i32(-max_variance..max_variance),
            self.2 + i32(-max_variance..max_variance),
            self.3 + i32(-max_variance..max_variance),
        )
    }
}

impl Trainer {
    /// creates n choose x matchups.
    fn get_matchups(&self) -> Vec<(usize, usize)> {
        let mut out = vec![];
        for (i, _) in self.variants.iter().enumerate() {
            for el2 in self.variants[(i + 1)..].iter().enumerate() {
                out.push((i, el2.0));
            }
        }
        out
    }
    /// Run a game between two variants, and based on the winner raise and decrement elo as necessary.
    fn run_game(&mut self, g1: usize, g2: usize) {
        let init_state = Game::gen_start(2);
        let mut s1 = State {
            weights: self.variants[g1].genome,
            state: init_state.for_id(1),
        };
        let mut s2 = State {
            weights: self.variants[g2].genome,
            state: init_state.for_id(2),
        };
        let time = Instant::now();
        while s2.amnt_dead() == 0 {
            let m1 = s1.get_best(&eval, &time);
            let m2 = s2.get_best(&eval, &time);
            s1.make_move(&array_vec!([(Direction, u8) ; 2] => (m1.0, 1), (m2.0, 2)));
            s2.make_move(&array_vec!([(Direction, u8) ; 2] => (m1.0, 1), (m2.0, 2)));
        }

        if s1.state.you.status == Status::Dead && s2.state.you.status == Status::Dead {
            self.variants[g1].reg_loss();
            self.variants[g2].reg_loss();
        } else if s1.state.you.status == Status::Alive {
            self.variants[g1].reg_win();
            self.variants[g2].reg_loss();
        } else if s2.state.you.status == Status::Alive {
            self.variants[g1].reg_loss();
            self.variants[g2].reg_win();
        }
    }
    /// Runs a training cycle given the previous best, and then returns the next best.
    fn run_cycle(&mut self, best: Variant, games: u8) -> Variant {
        self.variants = best.create_children(30, 10);
        for x in &self.get_matchups() {
            for _ in 0..games {
                self.run_game(x.0, x.1);
            }
        }
        self.variants.sort_by(|x, y| x.elo.cmp(&y.elo));
        self.variants[0]
    }

    /// takes in the Variant to tune, and the number of training cycles to complete.
    pub fn tune(&mut self, to_tune: Variant, cycles: u8) -> Variant {
        let mut best = to_tune;
        for _ in 0..cycles {
            best = self.run_cycle(best, 30);
        }
        best
    }
}

pub struct Game {
    snakes: Vec<SmallBattleSnake>,
    food: Vec<Coordinate>,
    width: u8,
    height: u8,
}

impl Game {
    /// returns a SmallMove that is derived from the game.
    pub fn for_id(&self, id: u8) -> SmallMove {
        let mut out = SmallMove::new();
        let mut you = &self.snakes[0];
        for x in &self.snakes {
            if x.id == id {
                you = x;
            }
        }
        out.board.snakes = self.snakes.clone();
        out.board.food = self.food.clone();
        out.board.width = self.width as i8;
        out.board.height = self.height as i8;
        out.you = you.clone();
        out.turn = 0;
        out
    }
    /// returns a start position based on the number of snakes
    pub fn gen_start(num_snakes: u8) -> Self {
        let mut out = Game {
            snakes: vec![],
            food: vec![],
            width: 11,
            height: 11,
        };

        out.place_snakes(num_snakes);
        out.place_food();
        out
    }
    /// Place Snakes
    fn place_snakes(&mut self, num_snakes: u8) {
        let (mn, md, mx) = (1_i8, ((self.width - 1) / 2) as i8, (self.width - 2) as i8);
        let mut start_points = vec![
            newc!(mn, mn),
            newc!(mn, md),
            newc!(mn, mx),
            newc!(md, mn),
            newc!(md, mx),
            newc!(mx, mn),
            newc!(mx, md),
            newc!(mx, mx),
        ];
        start_points.shuffle(&mut thread_rng());
        for x in 0..num_snakes {
            self.snakes
                .push(SmallBattleSnake::new(x, 100, &vec![newc!(3, 3)]));
        }
        for y in &mut self.snakes {
            for _ in 0..3 {
                y.body.push(start_points[y.id as usize]);
            }
            y.head = y.body[0];
            y.length = 3;
            y.status = Status::Alive;
        }
    }
    /// place all food
    /// 1 in center
    /// 1 piece of food per snake that is 2 moves away.
    fn place_food(&mut self) {
        for x in &self.snakes {
            let possible_food_locations = [
                newc!(x.head.x - 1, x.head.y - 1),
                newc!(x.head.x - 1, x.head.y + 1),
                newc!(x.head.x + 1, x.head.y - 1),
                newc!(x.head.x + 1, x.head.y + 1),
            ];
            self.food
                .push(possible_food_locations[thread_rng().gen_range(0..4)]);
        }
        self.food.push(newc!(6, 6));
    }
}
