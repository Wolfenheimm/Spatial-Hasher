use spatial_hasher::{Point3D, RotationAxis, Spha256};

fn main() {
    // Define the starting point and rotation axis
    let point = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let axis = RotationAxis {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    // Create a new SpaceHasher instance
    let hasher = Spha256::new(point, axis, 10, 0.1);

    // Original data to be encrypted
    let original_data = b"Hello, World!";
    println!(
        "Original Data: {:?}",
        String::from_utf8_lossy(original_data)
    );

    // Encrypt the data
    let encrypted = hasher.encrypt(original_data);
    println!("Encrypted Data: {:?}", encrypted);

    // Decrypt the data
    let decrypted = hasher.decrypt(&encrypted).expect("Decryption failed");
    println!("Decrypted Data: {:?}", String::from_utf8_lossy(&decrypted));

    // Verify that the decrypted data matches the original data
    assert_eq!(original_data, &decrypted[..], "Decryption failed");
}
