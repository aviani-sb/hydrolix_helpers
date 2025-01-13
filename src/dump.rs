use serde_json;

use crate::auth::HydrolixToken;
use crate::http;
use crate::hydrolix_cluster;
use crate::hydrolix_function;
use crate::hydrolix_org;
use crate::hydrolix_project;
use crate::hydrolix_storage;
use crate::hydrolix_table;
use crate::hydrolix_transform;

pub async fn dump(auth_token: &HydrolixToken) -> Result<Box<hydrolix_cluster::Cluster>, String> {
    let mut cluster: Box<hydrolix_cluster::Cluster> = Box::new(hydrolix_cluster::Cluster {
        base_url: auth_token.base_url.to_string(),
        orgs: None,
    });

    for org in &auth_token.org_list {
        let mut root_org: hydrolix_org::Org = hydrolix_org::Org {
            name: org.name.to_string(),
            uuid: org.uuid.to_string(),
            cloud: org.cloud.to_string(),
            kubernetes: org.kubernetes,
            projects: None,
            storages: None,
        };

        {
            // /config/v1/orgs/{org_id}/storages/
            let url = format!(
                "https://{}/config/v1/orgs/{}/storages",
                auth_token.base_url, root_org.uuid
            );

            let json_data = match http::get_data(auth_token, &url).await {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("{}.{} Error: {e}", file!(), line!()));
                }
            };

            let mut storages: Vec<hydrolix_storage::Storage> =
                match serde_json::from_str(&json_data) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!("{}.{} Error: {e}", file!(), line!()));
                    }
                };

            for s in &mut storages {
                if root_org.storages.is_none() {
                    root_org.storages = Some(Vec::new());
                }
                if let Some(storage_vec) = &mut root_org.storages {
                    storage_vec.push(s.clone());
                }
            }
        }

        let url = format!(
            "https://{}/config/v1/orgs/{}/projects",
            auth_token.base_url, root_org.uuid
        );

        let json_data = match http::get_data(auth_token, &url).await {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("{}.{} Error: {e}", file!(), line!()));
            }
        };

        let mut projects: Vec<hydrolix_project::Project> = match serde_json::from_str(&json_data) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("{}.{} Error: {e}", file!(), line!()));
            }
        };

        for p in &mut projects {
            let url = format!(
                "https://{}/config/v1/orgs/{}/projects/{}/functions",
                auth_token.base_url, org.uuid, p.uuid
            );

            let json_data = match http::get_data(auth_token, &url).await {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("{}.{} Error: {e}", file!(), line!()));
                }
            };

            let functions: Vec<hydrolix_function::Function> = match serde_json::from_str(&json_data)
            {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("{}.{} Error: {e}", file!(), line!()));
                }
            };

            for f in &functions {
                if p.functions.is_none() {
                    p.functions = Some(Vec::new());
                }

                if let Some(functions_vec) = &mut p.functions {
                    functions_vec.push(f.clone());
                }
            }
        }

        for p in &mut projects {
            let url = format!(
                "https://{}/config/v1/orgs/{}/projects/{}/tables",
                auth_token.base_url, org.uuid, p.uuid
            );

            let json_data = match http::get_data(auth_token, &url).await {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("{}.{} Error: {e}", file!(), line!()));
                }
            };

            let mut tables: Vec<hydrolix_table::Table> = match serde_json::from_str(&json_data) {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!("{}.{} Error: {e}", file!(), line!()));
                }
            };

            // Because transforms are only per table, add them to the local table,
            // and then copy those to the project
            for t in &mut tables {
                let url = format!(
                    "https://{}/config/v1/orgs/{}/projects/{}/tables/{}/transforms/",
                    auth_token.base_url, org.uuid, p.uuid, t.uuid
                );

                let json_data = match http::get_data(auth_token, &url).await {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(format!("{}.{} Error: {e}", file!(), line!()));
                    }
                };

                let mut transforms: Vec<hydrolix_transform::Transform> =
                    match serde_json::from_str(&json_data) {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(format!("{}.{} Error: {url} {e}", file!(), line!()));
                        }
                    };

                for z in &mut transforms {
                    if t.transforms.is_none() {
                        t.transforms = Some(Vec::new());
                    }

                    if let Some(transforms_vec) = &mut t.transforms {
                        transforms_vec.push(z.clone());
                    }
                }
            }
            // Now we walk through the tables and append them
            // to the project
            for t in &mut tables {
                if p.tables.is_none() {
                    p.tables = Some(Vec::new());
                }

                if let Some(tables_vec) = &mut p.tables {
                    tables_vec.push(t.clone());
                }
            }
            if root_org.projects.is_none() {
                root_org.projects = Some(Vec::new());
            }
            if let Some(project_vec) = &mut root_org.projects {
                project_vec.push(p.clone());
            }
        }

        if cluster.orgs.is_none() {
            cluster.orgs = Some(Vec::new());
        }
        if let Some(temp_vec) = &mut cluster.orgs {
            temp_vec.push(root_org.clone());
        }

        //_ = save_pretty_json(&json_value, "pretty_test.json").is_ok();
        //panic!("We are done!!!");
    }

    Ok(cluster)
}

#[cfg(test)]
mod tests {
    use serde_json::{to_value, Value};
    use std::fs;
    use std::io;

    use crate::auth::HydrolixAuth;
    use crate::hydrolix_secrets;

    use super::dump;

    fn print_pretty_json(json_data: &Value) -> Result<(), String> {
        // Write the JSON in a pretty-printed format to standard output
        let stdout = io::stdout();
        let handle = stdout.lock();
        match serde_json::to_writer_pretty(handle, json_data) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write JSON to stdout: {}", e)),
        }
    }

    #[tokio::test]
    async fn read_config() {
        let file_path = "/tmp/fleet.secrets.toml";

        // Read the file into a string
        let content = match fs::read_to_string(file_path) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Failed to read file: {e}"),
        };

        // Parse the TOML content into the Config struct
        let config: hydrolix_secrets::Config = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse config: {e}"),
        };

        for m in &config.machines {
            assert!(!m.base_url.is_empty());
        }
    }

    #[tokio::test]
    async fn test_get_token() {
        let file_path = "/tmp/fleet.secrets.toml";

        // Read the file into a string
        let content = match fs::read_to_string(file_path) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Failed to read file: {e}"),
        };

        // Parse the TOML content into the Config struct
        let config: hydrolix_secrets::Config = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse config: {e}"),
        };

        for m in &config.machines {
            let auth = HydrolixAuth::new(&m.base_url, &m.username, &m.password).await;
            assert!(!auth.clone().get_base_url().is_empty());

            let token = match auth.clone().get_token().await {
                Ok(v) => v,
                Err(e) => panic!("Failed to authenticate: {e}"),
            };
            assert!(token.org_list.len() > 0);
        }
    }

    #[tokio::test]
    async fn dump_orgs() {
        let file_path = "/tmp/fleet.secrets.toml";

        // Read the file into a string
        let content = match fs::read_to_string(file_path) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Failed to read file: {e}"),
        };

        // Parse the TOML content into the Config struct
        let config: hydrolix_secrets::Config = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => panic!("Failed to parse config: {e}"),
        };

        for m in &config.machines {
            let auth = HydrolixAuth::new(&m.base_url, &m.username, &m.password).await;
            assert!(auth.clone().get_base_url() == m.base_url);

            let auth_token = match auth.clone().get_token().await {
                Ok(v) => v,
                Err(e) => panic!("Failed to authenticate: {e}"),
            };

            let cluster_data = match dump(&auth_token).await {
                Ok(v) => v,
                Err(e) => panic!("Failed to parse the org!: {e}"),
            };

            let json_value: Value = match to_value(cluster_data) {
                Ok(v) => v,
                Err(e) => panic!("Failed {e}"),
            };

            match print_pretty_json(&json_value) {
                Ok(_) => (),
                Err(e) => panic!("Failed {e}"),
            };
        }
    }
}
// Verify the token value and organization
