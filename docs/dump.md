# Dump Module

The `dump` module is a part of the Hydrolix Helpers library. It provides functionality for retrieving and exporting Hydrolix cluster configurations in a hierarchical format. This module enables you to extract information about organizations, projects, tables, and transforms associated with a Hydrolix cluster.

---

## Features

- **Cluster Configuration Dumping**: Retrieve configurations for organizations, projects, tables, and associated transforms.
- **Hierarchical JSON Output**: Outputs configurations in a structured JSON format for further analysis or debugging.
- **Error Handling**: Provides detailed error messages for API failures.
- **Asynchronous Operations**: Supports async operations using `tokio`.

---

## Example Usage

Here’s an example of how to use the `dump` module:

```rust
use hydrolix_helpers::auth::HydrolixAuth;
use hydrolix_helpers::dump::dump;

#[tokio::main]
async fn main() {
    let auth = HydrolixAuth::new("localhost:3001", "ci@hydrolix.net", "test").await;

    let auth_token = auth.get_token().await
        .expect("Failed to authenticate");

    match dump(&auth_token).await {
        Ok(cluster) => println!("Cluster configuration: {:?}", cluster),
        Err(e) => eprintln!("Failed to dump configuration: {}", e),
    }
}
```

---

## API Overview

### `dump(auth_token: &HydrolixToken) -> Result<Box<hydrolix_cluster::Cluster>, String>`
Fetches the entire configuration for a cluster associated with the given `HydrolixToken`. The function makes a series of API calls to retrieve details about organizations, projects, tables, and transforms.

### Supporting Structures
- **`HydrolixToken`**: Contains authentication details and a list of organizations.
- **`hydrolix_cluster::Cluster`**: Represents the hierarchical structure of a cluster.
- **`hydrolix_org::Org`**: Contains organization-specific details.
- **`hydrolix_project::Project`**: Represents a project within an organization.
- **`hydrolix_table::Table`**: Describes a table within a project.

---

## Error Handling

The `dump` function handles errors gracefully by:
- Returning detailed error messages, including file and line numbers.
- Catching issues such as API failures, JSON parsing errors, and missing data.

---

## Test Suite

This module includes comprehensive tests to ensure reliability. Example:

```rust
#[tokio::test]
async fn test_dump_orgs() {
    let auth = HydrolixAuth::new("https://example-cluster.com", "username", "password");
    let auth_token = auth.get_token().await.expect("Failed to authenticate");

    let result = dump(&auth_token).await;
    assert!(result.is_ok());
}
```

---

## Notes

- **Performance**: The function uses `Box` to allocate the `Cluster` structure on the heap, ensuring it handles large configurations efficiently.
- **Asynchronous**: Requires the `tokio` runtime for async operations.
- **Error Reporting**: Uses detailed error messages to simplify debugging.

---

## Dependencies

- `tokio`: For asynchronous operations.
- `serde_json`: For JSON parsing and serialization.
- `reqwest`: For HTTP requests.

---

## License

This module is part of the Hydrolix Helpers library, which is licensed under the MIT License.

---
