use rand::{rngs::ThreadRng, Rng};

pub trait RandomItem<T> {
    fn random_item(&self, rng: &mut ThreadRng) -> &T;
}

impl<T> RandomItem<T> for Vec<T> {
    fn random_item(&self, rng: &mut ThreadRng) -> &T {
        &self[rng.gen_range(0..self.len())]
    }
}

impl<T> RandomItem<T> for [T] {
    fn random_item(&self, rng: &mut ThreadRng) -> &T {
        &self[rng.gen_range(0..self.len())]
    }
}
