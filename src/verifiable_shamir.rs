use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;
use std::ops::{Mul, Sub};

#[derive(Clone, Debug)]
pub struct Share {
    pub id: BigUint,
    pub value: BigUint,
}

#[derive(Clone, Debug)]
pub struct Commitment(Vec<BigUint>);

pub struct FeldmanVSS {
    pub p: BigUint,     // Prime field modulus
    pub q: BigUint,     // Prime order of generator
    pub g: BigUint,     // Generator
    pub t: usize,       // Threshold
    pub n: usize,       // Total number of shares
}

impl FeldmanVSS {
    /// Create new instance of Feldman's VSS
    pub fn new(p: BigUint, q: BigUint, g: BigUint, threshold: usize, total_shares: usize) -> Self {
        if threshold > total_shares {
            panic!("Threshold must be less than or equal to total shares");
        }
        FeldmanVSS {
            p,
            q,
            g,
            t: threshold,
            n: total_shares,
        }
    }

    /// Generate random polynomial coefficients
    pub fn generate_coefficients(&self, secret: &BigUint) -> Vec<BigUint> {
        let mut rng = thread_rng();
        let mut coefficients = vec![secret.clone()];
        
        // Generate t-1 random coefficients
        for _ in 1..self.t {
            let coeff = rng.gen_biguint_range(&0u32.into(), &self.q);
            coefficients.push(coeff);
        }
        
        coefficients
    }

    /// Generate commitments to polynomial coefficients
    pub fn generate_commitments(&self, coefficients: &[BigUint]) -> Commitment {
        let commitments: Vec<BigUint> = coefficients
            .iter()
            .map(|coeff| self.g.modpow(coeff, &self.p))
            .collect();
        
        Commitment(commitments)
    }

    /// Evaluate polynomial at point x
    pub fn evaluate_polynomial(&self, coefficients: &[BigUint], x: &BigUint) -> BigUint {
        let mut result = BigUint::from(0u32);
        let mut x_power = BigUint::from(1u32);
        
        for coeff in coefficients {
            let term = coeff.mul(&x_power) % &self.q;
            result = (result + term) % &self.q;
            x_power = (x_power * x) % &self.q;
        }
        
        result
    }

    /// Calculate modular multiplicative inverse using extended Euclidean algorithm
    pub fn mod_inverse(&self, a: &BigUint, m: &BigUint) -> Option<BigUint> {
        if a == &BigUint::from(0u32) {
            return None;
        }

        let mut t = BigUint::from(0u32);
        let mut newt = BigUint::from(1u32);
        let mut r = m.clone();
        let mut newr = a.clone();

        while newr != BigUint::from(0u32) {
            let quotient = &r / &newr;
            let temp_t = t.clone();
            t = newt.clone();
            newt = temp_t - &quotient * &newt;
            let temp_r = r.clone();
            r = newr.clone();
            newr = temp_r - &quotient * &newr;
        }

        if r > BigUint::from(1u32) {
            return None;
        }
        if &t < m {
            t = t + m;
        }
        Some(t % m)
    }

    /// Split secret into shares and generate commitments
    pub fn split_secret(&self, secret: &BigUint) -> (Vec<Share>, Commitment) {
        if secret >= &self.q {
            panic!("Secret must be less than q");
        }

        // Generate polynomial coefficients
        let coefficients = self.generate_coefficients(secret);
        
        // Generate commitments
        let commitments = self.generate_commitments(&coefficients);
        
        // Generate shares
        let mut shares = Vec::with_capacity(self.n);
        for i in 1..=self.n {
            let id = BigUint::from(i as u32);
            let value = self.evaluate_polynomial(&coefficients, &id);
            shares.push(Share { id, value });
        }
        
        (shares, commitments)
    }

    /// Verify a share against commitments
    pub fn verify_share(&self, share: &Share, commitments: &Commitment) -> bool {
        let mut lhs = BigUint::from(1u32);
        let mut x_power = BigUint::from(1u32);
        
        for commitment in &commitments.0 {
            let term = commitment.modpow(&x_power, &self.p);
            lhs = (lhs * term) % &self.p;
            x_power = (x_power * &share.id) % &self.q;
        }
        
        let rhs = self.g.modpow(&share.value, &self.p);
        lhs == rhs
    }

    /// Reconstruct secret from shares using Lagrange interpolation
    pub fn reconstruct_secret(&self, shares: &[Share]) -> Option<BigUint> {
        if shares.len() < self.t {
            return None;
        }

        let shares = &shares[0..self.t];
        let mut secret = BigUint::from(0u32);

        for (i, share_i) in shares.iter().enumerate() {
            let mut numerator = BigUint::from(1u32);
            let mut denominator = BigUint::from(1u32);

            for (j, share_j) in shares.iter().enumerate() {
                if i != j {
                    numerator = (numerator * &share_j.id) % &self.q;
                    let diff = if share_j.id > share_i.id {
                        (share_j.clone().id.sub(&share_i.id)) % &self.q
                    } else {
                        (&self.q + &share_j.id - &share_i.id) % &self.q
                    };
                    denominator = (denominator * diff) % &self.q;
                }
            }

            // Calculate modular multiplicative inverse using the extended Euclidean algorithm
            let denominator_inv = self.mod_inverse(&denominator, &self.q)
                .expect("Failed to compute modular multiplicative inverse");
            
            let lagrange_coeff = (numerator * denominator_inv) % &self.q;
            let term = (share_i.clone().value.mul(&lagrange_coeff)) % &self.q;
            secret = (secret + term) % &self.q;
        }

        Some(secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_feldman_vss() {
        // Test parameters (using small numbers for testing)
        let p = 23u32.to_biguint().unwrap();  // Prime modulus
        let q = 11u32.to_biguint().unwrap();  // Prime order
        let g = 2u32.to_biguint().unwrap();   // Generator
        let threshold = 3;
        let total_shares = 5;
        
        // Create VSS instance
        let vss = FeldmanVSS::new(p.clone(), q.clone(), g.clone(), threshold, total_shares);
        
        // Secret to share
        let secret = 7u32.to_biguint().unwrap();
        
        // Split secret and generate commitments
        let (shares, commitments) = vss.split_secret(&secret);
        
        // Verify all shares
        for share in &shares {
            assert!(vss.verify_share(share, &commitments));
        }
        
        // Reconstruct secret from threshold number of shares
        let reconstructed = vss.reconstruct_secret(&shares[0..threshold]);
        assert_eq!(reconstructed, Some(secret));
        
        // Try reconstruction with insufficient shares
        let reconstructed = vss.reconstruct_secret(&shares[0..threshold-1]);
        assert_eq!(reconstructed, None);
    }

    #[test]
    fn test_mod_inverse() {
        let p = 23u32.to_biguint().unwrap();
        let q = 11u32.to_biguint().unwrap();
        let g = 2u32.to_biguint().unwrap();
        let vss = FeldmanVSS::new(p, q.clone(), g, 3, 5);

        // Test some known modular multiplicative inverses
        let a = 3u32.to_biguint().unwrap();
        let inv = vss.mod_inverse(&a, &q).unwrap();
        assert_eq!((a * inv) % &q, BigUint::from(1u32));
    }
}