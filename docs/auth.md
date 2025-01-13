# Auth Module

The auth module in the Hydrolix Helpers crate provides functionality for managing authentication with Hydrolix clusters. It handles token-based authentication, caches tokens for performance, and supports retrieving organizational details.

## Features

* Token Caching: Reuse tokens efficiently to minimize redundant API calls.
* Cluster Authentication: Authenticate with multiple Hydrolix clusters using credentials.
* Organizational Support: Retrieve and manage organizational details associated with the authentication token.
* Thread-Safe Token Management: Uses once_cell and tokio::sync::Mutex to ensure thread safety.

## Example Usage

Authenticate and Retrieve a Token

```rust 
use hydrolix_helpers::auth::HydrolixAuth;

#[tokio::main]
async fn main() {
    let base_url = "https://example-cluster.example.com";
    let username = "user@example.com";
    let password = "password";

    let auth = HydrolixAuth::new(base_url, username, password);

    match auth.get_token().await {
        Ok(token) => {
            println!("Authenticated successfully!");
            println!("Token Value: {}", token.value);
            println!("First Organization: {}", token.first_org());
        }
        Err(e) => eprintln!("Authentication failed: {}", e),
    }
}
```

## API Overview

### HydrolixAuth

* The main struct for handling authentication.

### HydrolixToken

* Holds token and organizational details.

* Thread-Safe Token Management


## Test Suite

The auth module includes tests to validate token caching and authentication logic. Here’s a sample test:

```rust
#[tokio::test]
async fn test_get_token() {
    let base_url = "https://example-cluster.example.com";
    let username = "user@example.com";
    let password = "password";

    let auth = HydrolixAuth::new(base_url, username, password);

    for i in 0..5 {
        match auth.clone().get_token().await {
            Ok(token) => assert!(token.hits == i),
            Err(e) => panic!("Authentication failed: {}", e),
        }
    }
}
```

