use rand::Rng;

pub struct ApprxCounter {
    count: u32,
}

impl ApprxCounter {
    pub fn new() -> ApprxCounter {
        ApprxCounter { count: 0 }
    }

    pub fn increment(&mut self) {
        let mut rng = rand::thread_rng();
        let random_value: f64 = rng.gen(); // 0.0 -> 1.0
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
