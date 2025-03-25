use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct TableMetadata {
    pub name: String,
    pub desc: Option<String>,
    pub source: Option<String>,
    pub source_url: Option<Url>,
    pub license: Option<String>,
    pub license_url: Option<Url>,

    pub primary_key: Option<String>,
    pub columns: Vec<ColumnMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnMetadata {
    pub name: String,
    pub desc: Option<String>,
    pub data_type: String,
    pub foreign_key: Option<ColumnForeignKeyDetails>,
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
    pub desc: Option<String>,
}
