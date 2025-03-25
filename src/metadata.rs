use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct TableMetadata {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    pub columns: Vec<ColumnMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnMetadata {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_key: Option<ColumnForeignKeyDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<ColumnEnumDetails>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnForeignKeyDetails {
    pub foreign_table: String,
    pub foreign_column: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnEnumDetails {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}
