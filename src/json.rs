use crate::{
    error::{Error, Result},
    metadata::TableMetadata,
};

pub fn parse_metadata(json: &str) -> Result<TableMetadata> {
    serde_json::from_str(json).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata() {
        let json_str = include_str!("../test/metadata.json");

        let metadata: TableMetadata = parse_metadata(json_str).unwrap();
        assert_eq!(metadata.name, "example_table");
        assert_eq!(metadata.columns.len(), 2);
    }
}
