# rust-hue-sdk
Hue SDK in rust

# Discover Hue Bridges
```rust
extern crate hue_sdk;
use hue_sdk::bridge;

fn main() {
    let bridges = bridge::discover();
    println!("{:?}", bridges);
}
```
