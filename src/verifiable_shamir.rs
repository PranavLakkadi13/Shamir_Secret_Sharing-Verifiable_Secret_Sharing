use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct VerifiableShare {
    pub x: BigInt,
    pub y: BigInt,
    pub commitment: Vec<BigInt>,
}

pub struct VerifiableSharer {
    prime: BigInt,
    generator: BigInt,
    threshold: usize,
    total_shares: usize,
}

impl VerifiableSharer {
    pub fn new(threshold: usize, total_shares: usize) -> Self {
        let prime = BigInt::from(2).pow(127) - BigInt::from(1);
        let generator = BigInt::from(2); // Simple generator for demonstration

        VerifiableSharer {
            prime,
            generator,
            threshold,
            total_shares,
        }
    }

    fn create_commitment(&self, coefficients: &[BigInt]) -> Vec<BigInt> {
        coefficients
            .iter()
            .map(|coeff| self.mod_pow(&self.generator, coeff))
            .collect()
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

    pub fn split_secret(&self, secret: &BigInt) -> Vec<VerifiableShare> {
        let mut rng = thread_rng();
        let mut coefficients = vec![secret.clone() % &self.prime];

        // Generate random coefficients
        for _ in 1..self.threshold {
            coefficients.push(rng.gen_bigint_range(&BigInt::zero(), &self.prime));
        }

        // Create commitment
        let commitment = self.create_commitment(&coefficients);

        // Generate shares
        let mut shares = Vec::new();
        for x in 1..=self.total_shares {
            let x = BigInt::from(x);
            let mut y = coefficients[0].clone();
            let mut x_pow = BigInt::one();

            for coef in coefficients.iter().skip(1) {
                x_pow = (&x_pow * &x) % &self.prime;
                y = (&y + (coef * &x_pow) % &self.prime) % &self.prime;
            }

            shares.push(VerifiableShare {
                x,
                y,
                commitment: commitment.clone(),
            });
        }
        shares
    }

    pub fn verify_share(&self, share: &VerifiableShare) -> bool {
        let mut expected = BigInt::one();
        let mut x_pow = BigInt::one();

        for commitment in share.commitment.iter() {
            x_pow = (&x_pow * &share.x) % &self.prime;
            expected = (expected * self.mod_pow(commitment, &x_pow)) % &self.prime;
        }

        self.mod_pow(&self.generator, &share.y) == expected
    }

    pub fn reconstruct_secret(&self, shares: &[VerifiableShare]) -> Option<BigInt> {
        if shares.len() < self.threshold {
            return None;
        }

        // Verify all shares first
        for share in shares {
            if !self.verify_share(share) {
                return None;
            }
        }

        let mut secret = BigInt::zero();

        for i in 0..self.threshold {
            let mut numerator = BigInt::one();
            let mut denominator = BigInt::one();

            for j in 0..self.threshold {
                if i != j {
                    numerator = (&numerator * &shares[j].x) % &self.prime;
                    let temp = (&self.prime + &shares[i].x - &shares[j].x) % &self.prime;
                    denominator = (&denominator * &temp) % &self.prime;
                }
            }

            let inv_denominator =
                self.mod_pow(&denominator, &(self.prime.clone() - BigInt::from(2)));
            let coefficient = (&numerator * inv_denominator) % &self.prime;
            secret = (&secret + (&shares[i].y * &coefficient) % &self.prime) % &self.prime;
        }

        Some(secret)
    }
}
