# Struct Auto From

Simple Rust library for auto generating conversion methods

## Instalation
```toml
[dependencies]
struct_auto_from = "0.1.0"
```

## Usage
```rust
use struct_auto_from::auto_from;

#[auto_from(UserType)]
pub struct UserModel {
    id: i32,
    name: String,
}

#[auto_from(UserModel)]
pub struct UserType {
    id: i32,
    name: String,
}

fn main() {
    let user_model = UserModel {
        id: 0,
        name: "GvR".into(),
    };

    let user_type: UserType = user_model.into();

    // use user_type without the need for manual conversion
    // ...
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
