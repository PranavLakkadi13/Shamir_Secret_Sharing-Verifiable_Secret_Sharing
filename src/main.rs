mod shamir_sharing;
use num_bigint::BigInt;
mod mytry;
mod verifiable_shamir;

fn main() {
    // Create a secret using BigInt
    let secret = BigInt::from(22773311);
    let sharer = shamir_sharing::SecretSharer::new(9, 12);

    // Split the secret
    let shares = sharer.split_secret(&secret);
    std::println!("Generated shares: {:#?}", shares);

    // Reconstruct from shares with error handling
    match sharer.reconstruct_secret(&shares[0..9]) {
        Some(reconstructed) => {
            std::println!("Reconstructed secret: {:#}", reconstructed);
            assert_eq!(reconstructed, secret);
        }
        None => std::println!("Failed to reconstruct secret"),
    }

    #[allow(unused_doc_comments)]
    ////////////////////////////////////////////
    // / Verifiable Shamir Secret Sharing Test //
    ////////////////////////////////////////////
    let threshold = 36;
    let total_shares = 446;

    // let vss = verifiable_shamir::VerifiableSharer::new(threshold, total_shares);
    // let vshares = vss.split_secret(&secret);
    // println!("Generated verifiable shares: {:#?}", vshares);

    // // Reconstruct with verification
    // match vss.reconstruct_secret(&vshares[0..threshold]) {
    //     Some(reconstructed) => {
    //         println!("VSS Reconstructed secret: {:#}", reconstructed);
    //         assert_eq!(reconstructed, secret);
    //     }
    //     None => println!("VSS Failed to reconstruct secret or verification failed"),
    // }

    // // Verify individual shares
    // println!("Verifying shares...");
    // for (i, share) in vshares.iter().enumerate() {
    //     println!("Share {} verified: {}", i, vss.verify_share(share));
    // }
}
