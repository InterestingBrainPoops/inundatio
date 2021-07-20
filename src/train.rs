use core::num;
use alea::i32_in_range;
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
    pub fn create_children (&self, num_children: u8, max_variance: u8) -> Vec<Variant> {
        let out = vec![];
        for _ in 0..num_children {
            out.push(Variant::new(self.genome.randomize(max_variance)));
        }
        out
    }
    pub fn new (genome: Weights ) -> Self{
        Variant{genome, elo: 0}
    }
    pub fn reg_win(&mut self) {
        elo += 5;
    }
    pub fn reg_loss(&mut self) {
        elo -= 3;
    }
}

impl Weights {
    pub fn randomize(&self, max_variance: u8) -> Self {
        Weights(weights.0 + i32_in_range(max_variance as i32 * -1, max_variance as i32) as i32, weights.1 + i32_in_range(max_variance as i32 * -1, max_variance as i32) as i32,weights.2 + i32_in_range(max_variance as i32 * -1, max_variance as i32) as i32,weights.3 + i32_in_range(max_variance as i32 * -1, max_variance as i32) as i32)
    }
}

impl Trainer {

}