use crate::{Point3D, RotationAxis};
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
/// A hasher that uses a 3D point and a rotation axis to encrypt and decrypt data.
///
/// The `Spha256` struct provides methods for encrypting and decrypting data based on a deterministic algorithm. It utilizes a pseudo-random number generator (PRNG) seeded with a hash of its parameters.
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

    /// Generates a 256-bit seed for the pseudo-random number generator (PRNG) by hashing the hasher's parameters.
    ///
    /// This function uses the SHA-256 hash function to create a seed based on the bit representations of the `point`, `rotation_axis`, `iterations`, and `strength` fields. The seed is used to initialize the PRNG in the [`encrypt`](#method.encrypt) and [`decrypt`](#method.decrypt) methods.
    ///
    /// # Returns
    ///
    /// A 32-byte array representing the seed for the PRNG.
    fn generate_seed(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // Hash the bit representations of the floating-point numbers
        hasher.update(self.point.x.to_bits().to_ne_bytes());
        hasher.update(self.point.y.to_bits().to_ne_bytes());
        hasher.update(self.point.z.to_bits().to_ne_bytes());

        hasher.update(self.rotation_axis.x.to_bits().to_ne_bytes());
        hasher.update(self.rotation_axis.y.to_bits().to_ne_bytes());
        hasher.update(self.rotation_axis.z.to_bits().to_ne_bytes());

        hasher.update(self.iterations.to_ne_bytes());
        hasher.update(self.strength.to_bits().to_ne_bytes());

        let result = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&result[..32]);
        seed
    }

    /// Encrypts the provided data using the hasher's parameters.
    ///
    /// This method encrypts the input data by XORing each byte with a byte generated from a pseudo-random number generator (PRNG). The PRNG is seeded using the [`generate_seed`](#method.generate_seed) method, ensuring that the encryption is deterministic based on the hasher's parameters.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of bytes representing the data to encrypt.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encrypted data.
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
        let seed = self.generate_seed();
        let mut rng = Xoshiro256PlusPlus::from_seed(seed);
        data.iter()
            .map(|&byte| {
                let rand_byte = rng.next_u32() as u8;
                byte ^ rand_byte
            })
            .collect()
    }

    /// Decrypts the provided data using the hasher's parameters.
    ///
    /// The decryption process is identical to the encryption process since the XOR operation is symmetric. The method uses the same PRNG sequence as in [`encrypt`](#method.encrypt), ensuring that the original data is recovered when the same parameters are used.
    ///
    /// # Arguments
    ///
    /// * `encrypted` - A slice of bytes representing the encrypted data.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the decrypted data.
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
    /// let decrypted = hasher.decrypt(&encrypted);
    /// assert_eq!(decrypted, b"Secret Message");
    /// ```
    pub fn decrypt(&self, encrypted: &[u8]) -> Vec<u8> {
        // Decryption is the same as encryption in XOR cipher
        self.encrypt(encrypted)
    }
}
