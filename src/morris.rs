//! Morris Counter

use rand::{rngs::ThreadRng, Rng};

pub struct ApprxCounter {
    rng: ThreadRng,
    count: u32,
}

impl ApprxCounter {
    pub fn new() -> ApprxCounter {
        ApprxCounter {
            count: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn increment(&mut self) {
        let random_value: f64 = self.rng.gen(); // 0.0 -> 1.0
        let probability = 1.0 / (2.0_f64.powi(self.count as i32));

        // Increment the count with the probabilistic approach
        if random_value < probability {
            self.count += 1;
        }
    }

    pub fn estimate(&self) -> u32 {
        (2u32.pow(self.count)) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let mut counter = ApprxCounter::new();

        for _ in 1..10_000_000 {
            counter.increment();
        }

        // probabilistc with +-std deviation of ??
        assert_eq!(counter.estimate(), 10_000_000);
    }
}
