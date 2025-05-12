use blake2::{Blake2s256, Digest};

fn main() {
    // The password to hash, padded to 32 bytes
    let password = b"password";
    let mut padded_password = Vec::with_capacity(32);
    padded_password.extend_from_slice(password);
    padded_password.extend_from_slice(&vec![0u8; 32 - password.len()]);

    // Create a Blake2s hasher with 256-bit output
    let mut hasher = Blake2s256::new();

    // Hash the padded password
    hasher.update(&padded_password);

    // Get the hash as bytes
    let hash_bytes = hasher.finalize();

    // Print the hash as a list of bytes
    let byte_list: Vec<u8> = hash_bytes.to_vec();
    println!("{:?}", byte_list);
}
