# Rust Implementation of Blake2s Hashing with Password Padding

## Overview
This Rust program demonstrates how to perform Blake2s hashing on a password padded to a fixed length of 32 bytes. The goal is to mimic the behavior of a similar Python code that hashes a password using the Blake2s algorithm. 

## Prerequisites
- Rust (latest stable version recommended)
- Cargo (comes with Rust installation)

## Installation
To use the Blake2s hashing function, add the following dependency to your `Cargo.toml` file:
```toml
[dependencies]
blake2 = "1.0"
```
## Code Explanation
The program takes the following steps:
- Constructs a password from the string "password".
- Pads the password to a length of 32 bytes by appending zero bytes.
- Hashes the padded password using the Blake2s256 hashing algorithm.
- Prints the resulting hash as a list of bytes.
## Code
```rust
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
```
## Cloning the Repository
To get a local copy of the project, run the following command:

```bash
git clone https://github.com/cypriansakwa/Rust_Implementation_of_Blake2s_Hashing_with_Password_Padding.git
cd Rust_Implementation_of_Blake2s_Hashing_with_Password_Padding
```
## Usage
To compile and run the code:
```bash
cargo run
```
## Expected Output
The program will print the hash of the padded password as a list of bytes, similar to:
```csharp
[49, 45, 17, 49, 53, 209, 86, 115, 98, 55, 209, 224, 27, 143, 90, 155, 33, 237, 112, 68, 0, 174, 155, 3, 111, 186, 31, 202, 6, 87, 216, 98]
```
## Explanation of the Password
The password used in this program is the string "password" followed by 24 zero bytes, resulting in a total length of 32 bytes.
## When is it Useful?
This hashing technique is useful when:
1. You need to securely hash passwords or other sensitive data.
2. You want to perform hashing in a memory-efficient way, with fixed output size.
3. You need compatibility with systems that utilize the Blake2s hashing algorithm, particularly in cryptographic applications.
## Troubleshooting
If you encounter errors related to the hashing function, ensure that:

- You have added the correct version of the `blake2` crate in `Cargo.toml`.
- You are using the correct hash function, `Blake2s256`, which specifies a 256-bit (32-byte) output size.

## References
- [Blake2 RFC](https://www.rfc-editor.org/rfc/rfc7693)
- [blake2 crate documentation](https://docs.rs/blake2/latest/blake2/)
