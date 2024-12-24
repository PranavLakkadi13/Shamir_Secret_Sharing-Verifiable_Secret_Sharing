mod shamir_sharing;
use num_bigint::BigInt;


mod mytry;

fn main() {
    // Create a secret using BigInt
    let secret = BigInt::from(22773311);
    let sharer = shamir_sharing::SecretSharer::new(36, 4466);

    // Split the secret
    let shares = sharer.split_secret(&secret);
    std::println!("Generated shares: {:#?}", shares);

    // Reconstruct from shares with error handling
    match sharer.reconstruct_secret(&shares[0..39]) {
        Some(reconstructed) => {
            std::println!("Reconstructed secret: {:#}", reconstructed);
            assert_eq!(reconstructed, secret);
        }
        None => std::println!("Failed to reconstruct secret"),
    }
}
