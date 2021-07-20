use core::num;
use fastrand::i32;
use crate::types::Weights;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trainer {
    pub variants : Vec<Variant>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Variant {
    pub genome : Weights,
    pub elo : i64,
}

impl Variant {

    /// create a certain number of children based on the max variance and also my own genome.
    pub fn create_children (&self, num_children: u8, max_variance: u8) -> Vec<Variant> {
        let out = vec![];
        for _ in 0..num_children {
            out.push(Variant::new(self.genome.randomize(max_variance as i32)));
        }
        out
    }

    /// create a new version of myself.
    pub fn new (genome: Weights ) -> Self{
        Variant{genome, elo: 0}
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
        Weights(self.0 + i32(-max_variance..max_variance), self.1 + i32(-max_variance..max_variance), self.2 + i32(-max_variance..max_variance), self.3 + i32(-max_variance..max_variance))
    }
}

impl Trainer {
    fn get_matchups(&self) -> Vec<(&mut Variant, &mut Variant)> {
        todo!();
    }
    /// Run a game between two variants, and based on the winner raise and decrement elo as necessary.
    fn run_game(&mut self, g1: &mut Variant, g2: &mut Variant ) {
        todo!();
    }
    /// Runs a training cycle given the previous best, and then returns the next best.
    fn run_cycle(&mut self, best: Variant, games: u8) -> Variant {
        for x in &mut self.get_matchups() {
            for _ in 0..games {
                self.run_game(&mut x.0, &mut x.1);
            }
        }
        self.variants.sort_by(|x,y| x.elo.cmp(&y.elo));
        self.variants[0]
    }

    /// takes in the Variant to tune, and the number of training cycles to complete.
    pub fn tune(&mut self, to_tune: Variant, cycles: u8) -> Variant{
        let mut best = to_tune;
        for _ in 0..cycles {
            best = self.run_cycle(best, 30);
        }
        best
    }
}