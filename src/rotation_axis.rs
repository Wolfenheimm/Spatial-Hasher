//! The `rotation_axis` module provides the `RotationAxis` struct, representing an axis of rotation in 3D space.

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
/// Represents an axis of rotation in 3D space.
///
/// # Fields
///
/// - `x`: The x-component of the rotation axis.
/// - `y`: The y-component of the rotation axis.
/// - `z`: The z-component of the rotation axis.
///
/// # Examples
///
/// ```
/// use spatial_hasher::RotationAxis;
///
/// let axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
/// ```
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RotationAxis {
    /// The x-component of the rotation axis.
    pub x: f64,
    /// The y-component of the rotation axis.
    pub y: f64,
    /// The z-component of the rotation axis.
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
