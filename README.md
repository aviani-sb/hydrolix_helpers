# Hydrolix Helpers

Hydrolix Helpers is an unofficial Rust library designed to simplify interactions with Hydrolix, a high-performance analytics database. This crate provides utility functions and abstractions for common tasks such as querying, data ingestion, and managing configurations.

## Modules

* auth.rs
* dump.rs 


## Installation

Add hydrolix_helpers to your Cargo.toml dependencies:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
hydrolix_helpers = "0.1.X"
```

### Configuration

Store your cluster configurations in a Secrets.toml file. Here’s an example:

```toml

[[machines]]
base_url = "https://example-cluster-1.example.com"
username = "user1@example.com"
password = "your-password-here"

[[machines]]
base_url = "https://example-cluster-2.example.com"
username = "user2@example.com"
password = "your-password-here"
```

This file allows the library to load and manage credentials for each cluster.

### Contributing

I welcome contributions! Feel free to submit pull requests or report issues in the GitHub repository.

### License

This project is licensed under the MIT License. See the LICENSE file for details.

