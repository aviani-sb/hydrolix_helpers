# Hydrolix Helpers

[![Crates.io](https://img.shields.io/crates/v/hydrolix_helpers.svg)](https://crates.io/crates/hydrolix_helpers)
[![Documentation](https://docs.rs/hydrolix_helpers/badge.svg)](https://docs.rs/hydrolix_helpers)

**Hydrolix Helpers** is an unofficial Rust library designed to simplify interactions with [Hydrolix](https://www.hydrolix.io), a high-performance analytics database. This crate provides utility functions and abstractions for common tasks such as querying, ingesting data, and managing configurations.

---

## Features

- Easy-to-use APIs for interacting with Hydrolix
- Helper functions for constructing and executing queries
- Utilities for working with Hydrolix-specific data formats

---

## Example


Add this crate to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
hydrolix_helpers = "0.1.2"

```rust
use hydrolix_helpers::auth::HydrolixAuth;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let base_url = env::var("TEST_URL").unwrap();
    let username = env::var("TEST_LOGIN").unwrap();
    let password = env::var("TEST_PASSWORD").unwrap();

    let auth = HydrolixAuth::new(&base_url, &username, &password);

    // Verify that the token is being cached
    for i in 0..100 {
        let t = match auth.clone().get_token().await {
            Ok(v) => v,
            Err(e) => panic!("Failed to authenticate: {e}"),
        };
        assert!(t.hits == i);
        println!("t={:?}", t);
    }
}
