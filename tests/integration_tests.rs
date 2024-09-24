use spatial_hasher::{Point3D, RotationAxis, Spha256};

#[test]
fn test_encryption_decryption() {
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
    let hasher = Spha256::new(point, axis, 10, 0.1);

    let data = b"Test Data";
    let encrypted = hasher.encrypt(data);
    let decrypted = hasher.decrypt(&encrypted);

    assert_eq!(data, &decrypted[..]);
}
