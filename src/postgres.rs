use crate::{error::Result, metadata::TableMetadata};
use tokio_postgres::types::Json;

const INIT_SQL: &str = include_str!("./postgres_schema.sql");

pub async fn init_schema(client: &tokio_postgres::Client) -> Result<()> {
    // Execute the SQL commands to create the schema
    client.batch_execute(INIT_SQL).await?;
    Ok(())
}

pub async fn upsert(
    client: &tokio_postgres::Client,
    table: &str,
    metadata: &TableMetadata,
) -> Result<()> {
    // Prepare the SQL statement for upsert
    let stmt = client
        .prepare(
            r#"
            INSERT INTO "datasets"
                ("table_name", "metadata")
            VALUES ($1, $2)
            ON CONFLICT ("table_name")
                DO UPDATE
                SET "metadata" = EXCLUDED."metadata"
            "#,
        )
        .await?;

    // Execute the upsert
    client.execute(&stmt, &[&table, &Json(metadata)]).await?;
    Ok(())
}

pub async fn get(
    client: &tokio_postgres::Client,
    table: &[&str],
) -> Result<Vec<(String, TableMetadata)>> {
    // Prepare the SQL statement for getting metadata
    let stmt = client
        .prepare(
            r#"
            SELECT
                "table_name",
                "metadata"
            FROM "datasets"
            WHERE "table_name" = ANY($1)
            "#,
        )
        .await?;

    // Execute the query
    let rows = client.query(&stmt, &[&table]).await?;

    // Map the rows to TableMetadata and return the Vec
    let mut result = Vec::new();
    for row in rows {
        let table_name: String = row.get(0);
        let metadata: Json<TableMetadata> = row.get(1);
        result.push((table_name, metadata.0));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio_postgres::NoTls;

    async fn drop_table(client: &tokio_postgres::Client) -> Result<()> {
        client
            .execute(r#"DROP TABLE IF EXISTS "datasets""#, &[])
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_upsert_and_get() -> Result<()> {
        let connect_str = env::var("POSTGRES_CONN_STR_TEST").unwrap();
        let (client, connection) = tokio_postgres::connect(&connect_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        init_schema(&client).await?;

        let metadata = TableMetadata {
            name: "example_table".to_string(),
            desc: Some("An example table".to_string()),
            source: Some("example_source".to_string()),
            source_url: None,
            license: Some("MIT".to_string()),
            license_url: None,
            primary_key: Some("id".to_string()),
            columns: vec![],
        };

        upsert(&client, "example_table", &metadata).await?;

        let result = get(&client, &["example_table"]).await?;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "example_table");
        assert_eq!(result[0].1.name, "example_table");

        drop_table(&client).await?;

        Ok(())
    }
}
