# Struct Auto From

Simple Rust library for auto generating conversion methods between structs.

When specifying conversion, all fields in the receiving struct type must either be defined in the sender, or have their default values defined on the receiver.

Default value attribute lets you override data from sender.

## Instalation
```toml
[dependencies]
struct_auto_from = "0.1.0"
```

## Usage
```rust
use std::collections::HashMap;
use struct_auto_from::auto_from;

#[auto_from(UserType)]
pub struct UserModel {
    id: i32,
    name: String,
}

#[auto_from(UserModel)]
pub struct UserType {
    #[auto_from_attr(default_value = 0)]
    id: i32,
    name: String,
    #[auto_from_attr(default_value = Default::default())]
    metadata: HashMap<String, String>,
}

fn main() {
    let user_model = UserModel {
        id: 1234,
        name: "GvR".into(),
    };

    let user_type: UserType = user_model.into();

    // use user_type without the need for manual conversion
    // user_type.id == 0
    // user_type.metadata.is_empty() == true
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
