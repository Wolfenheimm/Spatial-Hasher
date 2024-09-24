#![deny(missing_docs)]
//! # Spatial Hasher
//!
//! A Rust library for deterministic encryption and decryption using 3D spatial parameters with secure authenticated encryption.
//!
//! ## Overview
//!
//! `spatial_hasher` provides a way to encrypt and decrypt data using a deterministic algorithm based on 3D spatial parameters. It utilizes a combination of a 3D point, a rotation axis, the number of iterations, and a strength parameter to derive a cryptographic key using SHA-256 hashing. This key is then used with the **ChaCha20-Poly1305 authenticated encryption algorithm** to ensure secure encryption and decryption.
//!
//! ## Features
//!
//! - **Secure Authenticated Encryption**: Uses the ChaCha20-Poly1305 algorithm for strong encryption and integrity protection.
//! - **Deterministic Key Derivation**: Generates a consistent key from spatial parameters, allowing for reproducible encryption and decryption.
//! - **Customizable Parameters**: Adjust the starting point, rotation axis, iterations, and strength to modify the encryption.
//! - **Simple API**: Easy to integrate into other Rust projects.
//! - **Serialization Support**: Structures can be serialized and deserialized using `serde`.
//! - **Unit Tests Included**: Verify functionality with built-in tests.
//!
//! ## Nomenclature
//!
//! - **Point3D**: Represents a point in 3D space.
//! - **RotationAxis**: Represents a rotation axis in 3D space.
//! - **Spha256**: The core struct that provides encryption and decryption methods.
//!
//! ## Architecture
//!
//! The `Spha256` struct is the core of this library. It uses the spatial parameters to generate a cryptographic key and perform encryption and decryption using the ChaCha20-Poly1305 algorithm.
//! ![Diagram](https://raw.githubusercontent.com/Wolfenheimm/spatial-hasher/main/assets/architecture_diagram.png)
//!
//! ### **Key Derivation**
//!
//! The key is derived by hashing the spatial parameters using SHA-256:
//!
//! - Coordinates of the `Point3D` and `RotationAxis`.
//! - The `iterations` and `strength` parameters.
//!
//! ### **Encryption Process**
//!
//! 1. **Key Generation**: Derive the key from spatial parameters.
//! 2. **Nonce Generation**: Generate a unique nonce for each encryption.
//! 3. **Data Encryption**: Encrypt data using ChaCha20-Poly1305 with the key and nonce.
//!
//! ### **Decryption Process**
//!
//! - Use the same key derived from the parameters.
//! - Extract the nonce from the encrypted data.
//! - Decrypt the data using ChaCha20-Poly1305.
//!
//! ## Usage
//!
//! ### **Creating a `Spha256` Instance**
//!
//! ```rust
//! use spatial_hasher::{Point3D, RotationAxis, Spha256};
//!
//! let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
//! let rotation_axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
//! let iterations = 10;
//! let strength = 0.1;
//!
//! let hasher = Spha256::new(point, rotation_axis, iterations, strength);
//! ```
//!
//! ### **Encrypting Data**
//!
//! ```rust
//! let data = b"Secret Message";
//! let encrypted_data = hasher.encrypt(data);
//! ```
//!
//! ### **Decrypting Data**
//!
//! ```rust
//! let decrypted_data = hasher.decrypt(&encrypted_data).expect("Decryption failed");
//! assert_eq!(data, &decrypted_data[..]);
//! ```
//!
//! ## Modules
//!
//! - [`spatial_hasher`]: Contains the `Spha256` struct and related functionality.
//!
//! ## Structs
//!
//! - [`Point3D`]: Represents a point in 3D space.
//! - [`RotationAxis`]: Represents a rotation axis in 3D space.
//! - [`Spha256`]: Provides methods for encryption and decryption.
//!
//! ## Security Considerations
//!
//! The security of the encryption relies on the secrecy of the parameters used to derive the key. Ensure that the `Point3D`, `RotationAxis`, `iterations`, and `strength` parameters are kept confidential.
//!
//! ## Dependencies
//!
//! - `chacha20poly1305` for encryption
//! - `serde` for serialization
//! - `sha2` for SHA-256 hashing
//! - `rand` for random number generation
//!
//! ## License
//!
//! This project is licensed under the MIT License.
//!
//! ---
//!
//! [spatial_hasher]: crate::spatial_hasher
//! [Point3D]: crate::spatial_hasher::Point3D
//! [RotationAxis]: crate::spatial_hasher::RotationAxis
//! [Spha256]: crate::spatial_hasher::Spha256

pub mod point3d;
pub mod rotation_axis;
pub mod spatial_hasher;

pub use point3d::Point3D;
pub use rotation_axis::RotationAxis;
pub use spatial_hasher::Spha256;
