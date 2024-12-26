use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Share {
    pub x: BigInt,
    pub y: BigInt,
}

pub struct SecretSharer {
    prime: BigInt,
    threshold: usize,
    total_shares: usize,
}

impl SecretSharer {
    pub fn new(threshold: usize, total_shares: usize) -> Self {
        // Using a larger prime for better security with high thresholds
        let prime = BigInt::from(2).pow(521) - BigInt::from(1);
        SecretSharer {
            prime,
            threshold,
            total_shares,
        }
    }

    fn mod_pow(&self, base: &BigInt, exp: &BigInt) -> BigInt {
        let mut result = BigInt::one();
        let mut base = base.clone();
        let mut exp = exp.clone();

        while exp > BigInt::zero() {
            if &exp & BigInt::one() == BigInt::one() {
                result = (&result * &base) % &self.prime;
            }
            base = (&base * &base) % &self.prime;
            exp >>= 1;
        }
        result
    }

    fn mod_inverse(&self, a: &BigInt) -> Option<BigInt> {
        if a.is_zero() {
            return None;
        }
        let exp = &self.prime - 2;
        Some(self.mod_pow(a, &exp))
    }

    pub fn split_secret(&self, secret: &BigInt) -> Vec<Share> {
        let mut rng = thread_rng();
        let mut coefficients = vec![secret.clone() % &self.prime];

        // Generate random coefficients with proper modulo operations
        for _ in 1..self.threshold {
            coefficients.push(rng.gen_bigint_range(&BigInt::zero(), &self.prime));
        }

        let mut shares = Vec::with_capacity(self.total_shares);
        for x in 1..=self.total_shares {
            let x = BigInt::from(x);
            let mut y = coefficients[0].clone();
            let mut x_pow = BigInt::one();

            // Evaluate polynomial with careful modular arithmetic
            for coef in coefficients.iter().skip(1) {
                x_pow = (&x_pow * &x) % &self.prime;
                let term = (coef * &x_pow) % &self.prime;
                y = (&y + term) % &self.prime;
            }

            shares.push(Share { x, y });
        }

        shares
    }

    pub fn reconstruct_secret(&self, shares: &[Share]) -> Option<BigInt> {
        if shares.len() < self.threshold {
            println!(
                "Number of shares provided ({}) is less than the threshold ({})",
                shares.len(),
                self.threshold
            );
            return None;
        }

        // Use exactly threshold number of shares
        let shares = &shares[..self.threshold];
        let mut secret = BigInt::zero();

        for i in 0..self.threshold {
            let mut numerator = BigInt::one();
            let mut denominator = BigInt::one();

            // Calculate Lagrange basis polynomial with careful modular arithmetic
            for j in 0..self.threshold {
                if i != j {
                    // Calculate numerator with modular arithmetic
                    numerator = (&numerator * &shares[j].x) % &self.prime;

                    // Calculate denominator carefully to avoid negative numbers
                    let diff = if shares[i].x > shares[j].x {
                        (&shares[i].x - &shares[j].x) % &self.prime
                    } else {
                        (&self.prime + &shares[i].x - &shares[j].x) % &self.prime
                    };

                    denominator = (&denominator * &diff) % &self.prime;
                }
            }

            // Calculate modular multiplicative inverse
            let inv_denominator = self.mod_inverse(&denominator)?;

            // Calculate final coefficient
            let coefficient = (&numerator * &inv_denominator) % &self.prime;

            // Add contribution to secret
            let term = (&shares[i].y * &coefficient) % &self.prime;
            secret = (&secret + term) % &self.prime;
        }

        Some(secret)
    }
}
