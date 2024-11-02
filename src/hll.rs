use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::sync::Mutex;

const BITSET_CAPACITY: u64 = 64;
const CONSTANT: f64 = 0.79402;

pub struct HyperLogLog<KeyType> {
    n_bits: u16,
    buckets: Vec<u64>,
    cardinality: usize,
    _marker: std::marker::PhantomData<KeyType>,
}

impl<KeyType> HyperLogLog<KeyType>
where
    KeyType: Hash + Eq + Clone,
{
    pub fn new(n_bits: i16) -> Self {
        let num_buckets = 1 << n_bits; // 2^n_bits
        Self {
            n_bits: n_bits.try_into().unwrap(),
            buckets: vec![0; num_buckets as usize],
            cardinality: 0,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get_cardinality(&self) -> usize {
        self.cardinality
    }

    /// Adds a value into the HyperLogLog
    pub fn add_elem(&mut self, val: KeyType) {
        let hash = self.calculate_hash(&val);
        let binary = self.compute_binary(hash);
        let leading_zeroes = self.position_of_leftmost_one(binary);

        let index = (hash >> (BITSET_CAPACITY - self.n_bits as u64)) as usize;
        self.buckets[index] = self.buckets[index].max(leading_zeroes);
    }

    /// Computes the cardinality estimate
    pub fn compute_cardinality(&mut self) {
        let harmonic_mean: f64 = self.buckets
            .iter()
            .map(|&x| 2.0_f64.powi(-(x as i32)))
            .sum::<f64>()
            .recip();

        let m = self.buckets.len() as f64;
        self.cardinality = (CONSTANT * m * m * harmonic_mean) as usize;
    }

    /// Calculates the hash of a given value
    fn calculate_hash(&self, val: &KeyType) -> u64 {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        hasher.finish()
    }

    /// Computes the binary representation of a hash
    fn compute_binary(&self, hash: u64) -> u64 {
        hash
    }

    /// Computes the number of leading zeros
    fn position_of_leftmost_one(&self, bset: u64) -> u64 {
        BITSET_CAPACITY - bset.leading_zeros() as u64
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    #[test]
    fn estimate_cardinality_test() {
        let mut obj = HyperLogLog::new(3);
        assert_eq!(obj.get_cardinality(), 0);

        obj.add_elem(0);
        obj.compute_cardinality();
        let ans = obj.get_cardinality();
        assert_eq!(ans, 7);

        for _ in 0..10 {
            obj.add_elem(10);
            obj.add_elem(122);
            obj.add_elem(200);
            obj.add_elem(911);
            obj.add_elem(999);
            obj.add_elem(1402);
            obj.add_elem(15445);
            obj.add_elem(15645);
            obj.add_elem(123456);
            obj.add_elem(312457);

            if obj.get_cardinality() == 10 {
                obj.compute_cardinality();
                let ans = obj.get_cardinality();
                assert_eq!(ans, 10);
            }
        }

        for _ in 0..10 {
            obj.add_elem(-1);
            obj.add_elem(-2);
            obj.add_elem(-3);
            obj.add_elem(-4);
            obj.add_elem(-5);
            obj.add_elem(-6);
            obj.add_elem(-7);
            obj.add_elem(-8);
            obj.add_elem(-9);
            obj.add_elem(-27);

            if obj.get_cardinality() == 10 {
                obj.compute_cardinality();
                let ans = obj.get_cardinality();
                assert_eq!(ans, 10);
            }
        }
        
        obj.compute_cardinality();
        let ans = obj.get_cardinality();
        assert_eq!(ans, 10);
    }

    #[test]
    fn edge_test_1() {
        let mut obj = HyperLogLog::<i32>::new(-2);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 0);
    }

    #[test]
    fn edge_test_2() {
        let mut obj = HyperLogLog::new(0);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 0);

        obj.add_elem(1);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 1665180);

        obj.add_elem(-1);
        obj.compute_cardinality();
        assert_eq!(obj.get_cardinality(), 1665180);
    }
}
