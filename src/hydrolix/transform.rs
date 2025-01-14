use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transform {
    pub name: String,
    pub description: Option<String>,
    pub uuid: String,
    pub created: String, // Use `chrono::DateTime` for better date handling if needed
    pub modified: String,
    pub settings: TransformSettings,
    pub url: String,
    #[serde(rename = "type")]
    pub transform_type: String, // Renamed from "type" to avoid reserved keyword
    pub table: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformSettings {
    pub is_default: bool,
    pub rate_limit: Option<Value>, // Nullable field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_transform: Option<String>, // Handle as an optional field
    pub null_values: Option<Vec<String>>, // Nullable field
    pub sample_data: Option<Value>,
    pub output_columns: Vec<Column>,
    pub compression: Option<String>,           // Nullable field
    pub wurfl: Option<Value>,                  // Nullable field
    pub format_details: Option<FormatDetails>, // Nullable field
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataType {
    #[serde(rename = "type")]
    pub data_type: String, // Renamed from "type" to avoid reserved keyword
    pub index: bool,
    pub primary: Option<bool>,
    pub format: Option<String>,
    pub resolution: Option<String>,
    pub default: Option<Value>,
    pub script: Option<Value>,
    pub source: Option<Value>,
    pub suppress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatDetails {
    pub flattening: Option<FlatteningDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlatteningDetails {
    pub depth: Option<i64>, // Nullable field
    pub active: bool,
    pub map_flattening_strategy: Option<MapFlatteningStrategy>,
    pub slice_flattening_strategy: Option<MapFlatteningStrategy>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapFlatteningStrategy {
    pub left: Option<String>,  // Nullable field
    pub right: Option<String>, // Nullable field
}
