use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hydrolix::transform::Transform;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub project: String,
    pub name: String,
    pub description: Option<String>,
    pub uuid: String,
    pub created: String, // You can replace this with a proper date type like chrono::DateTime if needed
    pub modified: String,
    pub settings: TableSettings,
    pub url: String,
    pub table_type: Option<String>, // Renamed from "type" to avoid reserved keyword conflict
    pub primary_key: String,
    pub transforms: Option<Vec<Transform>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableSettings {
    pub default_query_options: Value, // Generic JSON for flexible key-value pairs
    pub rate_limit: Option<Value>,    // Nullable field
    pub stream: StreamSettings,
    pub age: AgeSettings,
    pub reaper: ReaperSettings,
    pub merge: MergeSettings,
    pub autoingest: Vec<AutoIngestSettings>,
    pub sort_keys: Vec<Value>,    // Array of generic JSON values
    pub shard_key: Option<Value>, // Nullable field
    pub max_future_days: i64,
    pub max_request_bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamSettings {
    pub token_list: Vec<Value>,
    pub hot_data_max_age_minutes: i64,
    pub hot_data_max_active_partitions: i64,
    pub hot_data_max_rows_per_partition: i64,
    pub hot_data_max_minutes_per_partition: i64,
    pub hot_data_max_open_seconds: i64,
    pub hot_data_max_idle_seconds: i64,
    pub cold_data_max_age_days: i64,
    pub cold_data_max_active_partitions: i64,
    pub cold_data_max_rows_per_partition: i64,
    pub cold_data_max_minutes_per_partition: i64,
    pub cold_data_max_open_seconds: i64,
    pub cold_data_max_idle_seconds: i64,
    pub message_queue_max_rows: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgeSettings {
    pub max_age_days: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReaperSettings {
    pub max_age_days: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MergeSettings {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoIngestSettings {
    pub enabled: bool,
    pub source: String,
    pub source_region: String,
    pub pattern: String,
    pub max_rows_per_partition: i64,
    pub max_minutes_per_partition: i64,
    pub max_active_partitions: i64,
    pub dry_run: bool,
    pub source_credential_id: Option<Value>,
    pub bucket_credential_id: Option<Value>,
}
