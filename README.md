# Spatial Hasher

A Rust library for deterministic encryption and decryption using 3D spatial parameters.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Creating a Spatial Hasher Instance](#creating-a-spatial_hasher-instance)
  - [Encrypting Data](#encrypting-data)
  - [Decrypting Data](#decrypting-data)
- [Example](#example)
- [Testing](#testing)
- [Dependencies](#dependencies)
- [Security Considerations](#security-considerations)
- [Disclaimer](#disclaimer)
- [Contributing](#contributing)
- [License](#license)

## Overview

`spatial_hasher` provides a way to encrypt and decrypt data using a deterministic algorithm based on 3D spatial transformations. It utilizes a combination of a 3D point, a rotation axis, the number of iterations, and a strength parameter to seed a pseudo-random number generator (PRNG). This ensures that the same input parameters will always produce the same encrypted output.

![Alt Text](assets/spha256.png)

## Features

- **Deterministic Encryption and Decryption**: Ensures consistent results with the same parameters.
- **Customizable Parameters**: Adjust the starting point, rotation axis, iterations, and strength to modify the encryption.
- **Simple API**: Easy to integrate into other Rust projects.
- **Serialization Support**: Structures can be serialized and deserialized using `serde`.
- **Unit Tests Included**: Verify the functionality with built-in tests.

## Installation

Add `spatial_hasher` to your `Cargo.toml` dependencies:

```toml
[dependencies]
spatial_hasher = "0.1.0"
```

## Usage
First, import the necessary structs:

```rust
use spatial_hasher::{Point3D, RotationAxis, Spha256};
```

## Creating a spatial_hasher Instance
Create a spatial_hasher instance by specifying the starting point, rotation axis, number of iterations, and strength:

```rust
let point = Point3D { x: 1.0, y: 2.0, z: 3.0 };
let rotation_axis = RotationAxis { x: 0.0, y: 1.0, z: 0.0 };
let iterations = 10;
let strength = 0.1;

let hasher = Spha256::new(point, rotation_axis, iterations, strength);
```

## Encrypting Data
Encrypt data by passing a byte slice to the encrypt method:

```rust
let data = b"Secret Message";
let encrypted_data = hasher.encrypt(data);
```

## Decrypting Data
Decrypt data by passing the encrypted byte slice to the decrypt method:

```rust
let decrypted_data = hasher.decrypt(&encrypted_data);
assert_eq!(data, &decrypted_data[..]);
```

## Example
Below is a complete example demonstrating how to use spatial_hasher:

```rust
use spatial_hasher::{Point3D, RotationAxis, Spha256};

fn main() {
    // Define the starting point and rotation axis
    let point = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let rotation_axis = RotationAxis {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let iterations = 10;
    let strength = 0.1;

    // Create a new spatial_hasher instance
    let hasher = Spha256::new(point, rotation_axis, iterations, strength);

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
    let decrypted = hasher.decrypt(&encrypted);
    println!(
        "Decrypted Data: {:?}",
        String::from_utf8_lossy(&decrypted)
    );

    // Verify that the decrypted data matches the original data
    assert_eq!(original_data, &decrypted[..], "Decryption failed");
}
```

Output:
```yaml
Original Data: "Hello, World!"
Encrypted Data: [ ... ]
Decrypted Data: "Hello, World!"
```

## Testing
Run the unit tests included with spatial_hasher using:

```bash
cargo test
```

## Dependencies
`spatial_hasher` relies on the following crates:

- `rand` for random number generation.
- `rand_xoshiro` for the Xoshiro256++ PRNG.
- `serde` for serialization and deserialization.
- `sha2` for SHA-256 hashing.

## Security Considerations

`spatial_hasher` uses XOR encryption with a pseudo-random number generator (PRNG) sequence derived from a seed generated via SHA-256 hashing of the input parameters. The PRNG used is `Xoshiro256++`, which is not cryptographically secure.

As a result, `spatial_hasher` should not be used for encrypting sensitive or confidential data. The encryption provided is suitable for obfuscation or simple data hiding in contexts where strong security is not required.

For applications requiring robust, secure encryption, please use established cryptographic libraries such as the [RustCrypto](https://github.com/RustCrypto) collection of crates.

## Disclaimer
`spatial_hasher` is intended for educational and demonstrational purposes. It should not be used in production systems where data security is a concern. The authors and contributors are not responsible for any misuse of this software.

## Contributing
Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Write tests for any new functionality.
4. Submit a pull request with a detailed description of your changes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.
