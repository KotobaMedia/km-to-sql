# km-to-sql

Shared helpers for KotobaMedia `to-sql` projects.

## Includes:

* Metadata types (`metadata::TableMetadata`)
    * Conforms to `serde::{Serialize, Deserialize}`
* Metadata (`datasets`) table schema and creation helpers
    * PostgreSQL: `postgres::{init_schema, upsert, get}`
