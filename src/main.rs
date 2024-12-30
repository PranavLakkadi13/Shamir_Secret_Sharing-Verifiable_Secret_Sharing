mod shamir_sharing;
use num_bigint::{BigInt, BigUint};
mod mytrySSS;
mod verifiable_shamir;

use num_bigint::ToBigUint;
use std::error::Error;
use verifiable_shamir::FeldmanVSS;

fn main() {
    // Create a secret using BigInt
    let secret = BigUint::from(22773311u64);
    // let sharer = shamir_sharing::SecretSharer::new(9, 12);

    // // Split the secret
    // let shares = sharer.split_secret(&secret);
    // std::println!("Generated shares: {:#?}", shares);

    // // Reconstruct from shares with error handling
    // match sharer.reconstruct_secret(&shares[0..9]) {
    //     Some(reconstructed) => {
    //         std::println!("Reconstructed secret: {:#}", reconstructed);
    //         assert_eq!(reconstructed, secret);
    //     }
    //     None => std::println!("Failed to reconstruct secret"),
    // }

    #[allow(unused_doc_comments)]
    ////////////////////////////////////////////
    // / Verifiable Shamir Secret Sharing Test //
    ////////////////////////////////////////////
    // Set up system parameters
    // In a real system, these should be carefully chosen cryptographic parameters
    // For demonstration, we're using smaller numbers
    let p = "115792089237316195423570985008687907853269984665640564039457584007908834671663"
        .parse::<BigUint>()
        .unwrap(); // Prime modulus (this is the SECP256k1 prime)
    
    let q = "115792089237316195423570985008687907852837564279074904382605163141518161494337"
        .parse::<BigUint>()
        .unwrap(); // Prime order (SECP256k1 order)
    
    let g = "2".parse::<BigUint>().unwrap(); // Generator

    // Set up the VSS parameters
    let threshold = 3;  // Number of shares needed to reconstruct
    let total_shares = 5;  // Total number of shares to create

    println!("Initializing Feldman VSS...");
    let vss = FeldmanVSS::new(p.clone(), q.clone(), g.clone(), threshold, total_shares);

    // Create a secret to share
    // In practice, this could be a private key or other sensitive data
    let secret = "123456789".parse::<BigUint>().unwrap();
    println!("Original secret: {}", secret);

    // Split the secret and generate commitments
    println!("\nGenerating shares and commitments...");
    let (shares, commitments) = vss.split_secret(&secret);

    // Print all shares (in practice, these would be distributed securely to different parties)
    println!("\nGenerated shares:");
    for (i, share) in shares.iter().enumerate() {
        println!("Share {}: ID = {}, Value = {}", i + 1, share.id, share.value);
    }

    // Verify each share
    println!("\nVerifying shares...");
    for (i, share) in shares.iter().enumerate() {
        let is_valid = vss.verify_share(share, &commitments);
        println!("Share {} verification: {}", i + 1, if is_valid { "Valid" } else { "Invalid" });
    }

    // Demonstrate reconstruction with threshold number of shares
    println!("\nReconstructing secret with {} shares...", threshold);
    let reconstructed = vss.reconstruct_secret(&shares[0..threshold]);
    match reconstructed {
        Some(value) => println!("Reconstructed secret: {}", value),
        None => println!("Failed to reconstruct secret"),
    }

    // Demonstrate failed reconstruction with fewer than threshold shares
    println!("\nAttempting reconstruction with {} shares (should fail)...", threshold - 1);
    let failed_reconstruction = vss.reconstruct_secret(&shares[0..threshold - 1]);
    match failed_reconstruction {
        Some(value) => println!("Reconstructed secret: {}", value),
        None => println!("Failed to reconstruct secret (as expected with fewer than threshold shares)"),
    }

    // Demonstrate verification of subset of shares
    println!("\nVerifying a subset of shares individually...");
    for share in shares.iter().take(threshold) {
        if vss.verify_share(share, &commitments) {
            println!("Share with ID {} is valid", share.id);
        } else {
            println!("Share with ID {} is invalid", share.id);
        }
    }


    #[allow(unused_doc_comments)]
    ////////////////////////////////////////////
    // / My Shamir Secret Sharing Test //
    ////////////////////////////////////////////
    let secret = BigUint::from(22773311u32);
    let x = mytrySSS::SecretSharer::new(5, 10);
    println!("{:?}", x);

    let new_x = mytrySSS::SecretSharer::generate_random_points(&x);
    std::println!("{:?}", new_x);

    let shares = mytrySSS::SecretSharer::split_secret(&x, &secret);
    std::println!("{:#?}", shares);
}


fn simulate_share_distribution(shares: Vec<Share>) -> Vec<Share> {
    // In a real implementation, this would handle the secure distribution of shares
    // For demonstration, we just return the shares
    shares
}

// Helper function to simulate share collection (in practice, this would involve secure communication)
fn simulate_share_collection(shares: &[Share], threshold: usize) -> Vec<Share> {
    // In a real implementation, this would handle the secure collection of shares
    // For demonstration, we just take the first 'threshold' shares
    return shares[0..threshold].to_vec();
}

pub struct Share {
    pub id: BigUint,
    pub value: BigUint,
}
