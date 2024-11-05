//! Morris Counter

use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
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

    pub fn increment(&mut self, v: f64, a: f64) {
        let random_value: f64 = self.rng.gen(); // 0.0 -> 1.0
        let delta = 1.0 /(Self::apprx_count(v+1.0, a)- Self::apprx_count(v, a));

        if random_value < delta {
            self.count += 1;
        }
    }

    pub fn apprx_count(v: f64, a: f64) -> f64 {
        a * (1.0 + 1.0 / a).powf(v - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_counter() {
        let counter = ApprxCounter::new();
        assert_eq!(counter.count, 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = ApprxCounter::new();
        let initial_count = counter.count;
        counter.increment(1.0, 2.0);

        assert!(counter.count >= initial_count);
    }

    #[test]
    fn test_apprx_count() {
        let result = ApprxCounter::apprx_count(1.0, 2.0);
        assert!((result - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiple_increments() {
        let mut counter = ApprxCounter::new();
        for _ in 0..100 {
            counter.increment(1.0, 2.0);
        }
        assert!(counter.count > 0);
    }

    #[test]
    fn test_apprx_count_with_different_values() {
        let result1 = ApprxCounter::apprx_count(2.0, 2.0);
        let result2 = ApprxCounter::apprx_count(3.0, 2.0);
        assert!(result2 > result1);
    }
}
