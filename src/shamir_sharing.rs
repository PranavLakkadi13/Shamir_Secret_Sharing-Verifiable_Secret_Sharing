// use num_bigint::{BigInt, RandBigInt};
// use num_traits::{One, Zero};
// use rand::thread_rng;

// #[derive(Debug, Clone)]
// pub struct Share {
//     x: BigInt,
//     y: BigInt,
// }

// pub struct SecretSharer {
//     prime: BigInt,
//     threshold: usize,
//     total_shares: usize,
// }

// impl SecretSharer {
//     pub fn new(threshold: usize, total_shares: usize) -> Self {
//         let prime = BigInt::parse_bytes(
//             b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
//             16,
//         )
//         .unwrap();
//         SecretSharer {
//             prime,
//             threshold,
//             total_shares,
//         }
//     }

//     fn mod_pow(&self, base: &BigInt, exp: &BigInt) -> BigInt {
//         let mut result = BigInt::one();
//         let mut base = base.clone();
//         let mut exp = exp.clone();

//         while exp > BigInt::zero() {
//             if &exp & BigInt::one() == BigInt::one() {
//                 result = (&result * &base) % &self.prime;
//             }
//             base = (&base * &base) % &self.prime;
//             exp >>= 1;
//         }
//         result
//     }

//     fn mod_inverse(&self, a: &BigInt) -> BigInt {
//         let exp = &self.prime - 2;
//         self.mod_pow(a, &exp)
//     }

//     pub fn split_secret(&self, secret: &BigInt) -> Vec<Share> {
//         let mut rng = thread_rng();
//         let mut coefficients = vec![secret.clone() % &self.prime];

//         for _ in 1..self.threshold {
//             coefficients.push(rng.gen_bigint_range(&BigInt::zero(), &self.prime));
//         }

//         let mut shares = Vec::new();
//         for x in 1..=self.total_shares {
//             let x = BigInt::from(x);
//             let mut y = coefficients[0].clone();
//             let mut x_pow = BigInt::one();

//             for coef in coefficients.iter().skip(1) {
//                 x_pow = (&x_pow * &x) % &self.prime;
//                 y = (&y + (coef * &x_pow) % &self.prime) % &self.prime;
//             }

//             shares.push(Share { x, y });
//         }

//         shares
//     }

//     pub fn reconstruct_secret(&self, shares: &[Share]) -> Option<BigInt> {
//         if shares.len() < self.threshold {
//             println!(
//                 "Number of shares provided ({}) is less than the threshold ({})",
//                 shares.len(),
//                 self.threshold
//             );
//             return None;
//         }

//         let mut secret = BigInt::zero();

//         for i in 0..shares.len() {
//             let mut numerator = BigInt::one();
//             let mut denominator = BigInt::one();

//             for j in 0..shares.len() {
//                 if i != j {
//                     numerator = (&numerator * &shares[j].x) % &self.prime;
//                     let diff = (&shares[i].x - &shares[j].x + &self.prime) % &self.prime;
//                     denominator = (&denominator * diff) % &self.prime;
//                 }
//             }

//             if denominator.is_zero() {
//                 println!("Error: Denominator became zero during reconstruction.");
//                 return None;
//             }

//             let inv_denominator = self.mod_inverse(&denominator);
//             let coefficient = (&numerator * inv_denominator) % &self.prime;

//             secret = (&secret + (&shares[i].y * coefficient) % &self.prime) % &self.prime;
//         }

//         Some(secret)
//     }
// }

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
        // let prime = BigInt::parse_bytes(
        //     b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        //     16,
        // )
        // .unwrap();
        // let x: BigInt = BigInt::from(2).pow(127) - BigInt::from(1);
        let prime = BigInt::from(2).pow(127) - BigInt::from(1);
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

    fn mod_inverse(&self, a: &BigInt) -> BigInt {
        let exp = &self.prime - 2;
        self.mod_pow(a, &exp)
    }

    pub fn split_secret(&self, secret: &BigInt) -> Vec<Share> {
        let mut rng = thread_rng();
        let mut coefficients = vec![secret.clone() % &self.prime];

        for _ in 1..self.threshold {
            coefficients.push(rng.gen_bigint_range(&BigInt::zero(), &self.prime));
        }

        let mut shares = Vec::new();
        for x in 1..=self.total_shares {
            let x = BigInt::from(x);
            let mut y = coefficients[0].clone();
            let mut x_pow = BigInt::one();

            for coef in coefficients.iter().skip(1) {
                x_pow = (&x_pow * &x) % &self.prime;
                y = (&y + (coef * &x_pow) % &self.prime) % &self.prime;
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

        let mut secret = BigInt::zero();

        for i in 0..shares.len() {
            let mut numerator = BigInt::one();
            let mut denominator = BigInt::one();

            for j in 0..shares.len() {
                if i != j {
                    numerator = (&numerator * &shares[j].x) % &self.prime;
                    let diff = (&shares[i].x - &shares[j].x + &self.prime) % &self.prime;
                    denominator = (&denominator * diff) % &self.prime;
                }
            }

            if denominator.is_zero() {
                println!("Error: Denominator became zero during reconstruction.");
                return None;
            }

            let inv_denominator = self.mod_inverse(&denominator);
            let coefficient = (&numerator * inv_denominator) % &self.prime;

            secret = (&secret + (&shares[i].y * coefficient) % &self.prime) % &self.prime;
        }

        // Ensure the secret is non-negative
        if secret < BigInt::zero() {
            secret += &self.prime;
        }

        Some(secret)
    }
}
