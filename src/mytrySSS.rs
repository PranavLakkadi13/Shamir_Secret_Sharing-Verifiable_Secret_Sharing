use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

// the shares that will be generated and passed to the users
#[derive(Debug)]
pub struct Shares {
    pub x: BigUint,
    pub y: BigUint,
}

// The struct that has the data about the total generated shares and the threshhold and will hold the prime value,
//  the imp data related to SSS
#[derive(Debug)]
pub struct SecretSharer {
    pub prime: BigUint,
    pub threshold: usize,
    pub total_shares: usize,
}

// The implementation of the SecretSharer struct
impl SecretSharer {
    // The new function that will create a new instance of the SecretSharer struct
    pub fn new(threshhold: usize, total_shares: usize) -> Self {
        let prime_num = BigUint::from(2u64).pow(31) - BigUint::from(1u64);
        SecretSharer {
            prime: prime_num,
            threshold: threshhold,
            total_shares: total_shares,
        }
    }

    // trying on my own to generate the random points as the coefficients of the x values
    pub fn generate_random_points(&self) -> Vec<BigUint> {
        let mut rng = thread_rng();
        let mut coefficients: Vec<BigUint> = Vec::new();
        for _ in 0..self.threshold {
            let gen_num = rng.gen_biguint_range(&BigUint::from(0u64), &self.prime);
            coefficients.push(gen_num);
        }
        coefficients
    }

    pub fn mod_pow(&self, base: &BigUint, exp: &BigUint) -> BigUint {
        let mut result = BigUint::from(1u64);
        let mut base = base.clone();
        let mut exp = exp.clone();

        while exp > BigUint::from(0u64) {
            if &exp & BigUint::from(1u64) == BigUint::from(1u64) {
                result = (&result * &base) % &self.prime;
            }
            base = (&base * &base) % &self.prime;
            exp >>= 1;
        }
        result
    }

    pub fn mod_inverse(&self, a: &BigUint) -> Option<BigUint> {
        if a == &BigUint::from(0u64) {
            return None;
        }
        let exp = &self.prime - BigUint::from(2u64);
        Some(self.mod_pow(a, &exp))
    }

    pub fn split_secret(&self, secret: &BigUint) -> Vec<Shares> {
        // Generate random coefficients with proper modulo operations
        let coefficients = self.generate_random_points();

        let mut shares = Vec::with_capacity(self.total_shares);
        for i in 1..=self.total_shares {
             
        }
        shares
    }
}
