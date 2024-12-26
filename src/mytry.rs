use num_bigint::BigUint;
use rand::{self, Rng};

pub struct shares {
    pub x: BigUint,
    pub y: BigUint,
}

pub struct secret_sharer {
    pub prime: BigUint,
    pub threshold: usize,
    pub total_shares: usize,
}

pub fn test_random() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..1000000000);
    println!("Random number: {}", random_number);
}
