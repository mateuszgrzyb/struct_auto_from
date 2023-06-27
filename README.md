# Auto From

Simple Rust library for auto generating conversion methods

## Instalation
```toml
[dependencies]
auto_from = "0.1.0
```

## Usage
```rust
use auto_from::auto_from;

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

# License
This project is licensed under the [MIT license](LICENSE).
