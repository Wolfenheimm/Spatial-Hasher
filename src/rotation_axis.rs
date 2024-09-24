use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
/// Represents a rotation axis in 3D space.
///
/// The `RotationAxis` struct defines an axis of rotation using `x`, `y`, and `z` components.
///
/// # Examples
///
/// ```
/// use spatial_hasher::RotationAxis;
/// let axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
/// ```
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RotationAxis {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Hash for RotationAxis {
    /// Hashes the `RotationAxis` by writing the bit representations of its components into the provided hasher.
    ///
    /// This method uses the `to_bits()` function to obtain the raw memory representation of the floating-point numbers, ensuring consistent hashing.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to a type implementing the `Hasher` trait.
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.x.to_bits());
        state.write_u64(self.y.to_bits());
        state.write_u64(self.z.to_bits());
    }
}
