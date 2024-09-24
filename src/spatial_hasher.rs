use crate::{Point3D, RotationAxis};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A hasher that uses a 3D point and a rotation axis to encrypt and decrypt data.
///
/// The `Spha256` struct provides methods for encrypting and decrypting data based on a deterministic algorithm. It derives a cryptographic key from its parameters and uses the ChaCha20-Poly1305 authenticated encryption algorithm for secure encryption and decryption.
///
/// # Examples
///
/// ```
/// use spatial_hasher::{Point3D, RotationAxis, Spha256};
/// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
/// let axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
/// let hasher = Spha256::new(point, axis, 10, 0.1);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Spha256 {
    /// The starting point in 3D space.
    point: Point3D,
    /// The rotation axis used in the hashing process.
    rotation_axis: RotationAxis,
    /// The number of iterations for the transformation.
    iterations: u32,
    /// The strength of the transformation.
    strength: f64,
}

impl Spha256 {
    /// Creates a new `Spha256` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `point` - A `Point3D` specifying the starting point in 3D space.
    /// * `rotation_axis` - A `RotationAxis` specifying the axis of rotation.
    /// * `iterations` - The number of iterations to perform in the hashing process.
    /// * `strength` - A floating-point value representing the strength of the transformation.
    ///
    /// # Returns
    ///
    /// A new `Spha256` instance configured with the provided parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_hasher::{Point3D, RotationAxis, Spha256};
    /// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
    /// let hasher = Spha256::new(point, axis, 10, 0.1);
    /// ```
    pub fn new(
        point: Point3D,
        rotation_axis: RotationAxis,
        iterations: u32,
        strength: f64,
    ) -> Self {
        Spha256 {
            point,
            rotation_axis,
            iterations,
            strength,
        }
    }

    /// Generates a 256-bit key by hashing the hasher's parameters.
    ///
    /// This function uses the SHA-256 hash function to create a key based on the bit representations of the `point`, `rotation_axis`, `iterations`, and `strength` fields. The key is used in the [`encrypt`](#method.encrypt) and [`decrypt`](#method.decrypt) methods with the ChaCha20-Poly1305 cipher.
    ///
    /// # Returns
    ///
    /// A 32-byte array representing the encryption key.
    fn generate_key(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.point.x.to_bits().to_ne_bytes());
        hasher.update(&self.point.y.to_bits().to_ne_bytes());
        hasher.update(&self.point.z.to_bits().to_ne_bytes());

        hasher.update(&self.rotation_axis.x.to_bits().to_ne_bytes());
        hasher.update(&self.rotation_axis.y.to_bits().to_ne_bytes());
        hasher.update(&self.rotation_axis.z.to_bits().to_ne_bytes());

        hasher.update(&self.iterations.to_ne_bytes());
        hasher.update(&self.strength.to_bits().to_ne_bytes());

        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result[..32]);
        key
    }

    /// Encrypts the provided data using the ChaCha20-Poly1305 authenticated encryption algorithm.
    ///
    /// This method encrypts the input data using the ChaCha20-Poly1305 cipher, with a key derived from the hasher's parameters via the [`generate_key`](#method.generate_key) method. A random nonce is generated for each encryption operation to ensure uniqueness and security. The nonce is prepended to the ciphertext for use during decryption.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes representing the data to encrypt.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encrypted data, with the nonce prepended.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_hasher::{Spha256, Point3D, RotationAxis};
    /// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
    /// let hasher = Spha256::new(point, axis, 10, 0.1);
    ///
    /// let data = b"Secret Message";
    /// let encrypted = hasher.encrypt(data);
    /// ```
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Derive key from parameters
        let key = self.generate_key();
        let cipher = ChaCha20Poly1305::new(&key.into());

        // Generate a random nonce (12 bytes for ChaCha20-Poly1305)
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the data
        let ciphertext = cipher.encrypt(nonce, data).expect("Encryption failed");

        // Prepend nonce to ciphertext
        let mut encrypted = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        encrypted.extend_from_slice(&nonce_bytes);
        encrypted.extend_from_slice(&ciphertext);

        encrypted
    }

    /// Decrypts the provided data using the ChaCha20-Poly1305 authenticated decryption algorithm.
    ///
    /// This method decrypts the input data using the ChaCha20-Poly1305 cipher, with a key derived from the hasher's parameters via the [`generate_key`](#method.generate_key) method. The nonce used during encryption is expected to be prepended to the encrypted data and is extracted during decryption.
    ///
    /// # Arguments
    ///
    /// * `encrypted` - A slice of bytes representing the encrypted data, with the nonce prepended.
    ///
    /// # Returns
    ///
    /// A `Result<Vec<u8>, &'static str>` containing the decrypted data on success, or an error message on failure.
    ///
    /// # Errors
    ///
    /// Returns an error if the decryption fails, such as when the ciphertext has been tampered with or the parameters do not match those used during encryption.
    ///
    /// # Examples
    ///
    /// ```
    /// use spatial_hasher::{Spha256, Point3D, RotationAxis};
    /// let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
    /// let axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
    /// let hasher = Spha256::new(point, axis, 10, 0.1);
    ///
    /// let encrypted = hasher.encrypt(b"Secret Message");
    /// let decrypted = hasher.decrypt(&encrypted).expect("Decryption failed");
    /// assert_eq!(decrypted, b"Secret Message");
    /// ```
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, &'static str> {
        if encrypted.len() < 12 {
            return Err("Ciphertext too short to contain nonce");
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Derive key from parameters
        let key = self.generate_key();
        let cipher = ChaCha20Poly1305::new(&key.into());

        // Decrypt the data
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed")?;

        Ok(plaintext)
    }
}
