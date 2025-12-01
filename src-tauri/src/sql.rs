use anyhow::Result;
use sqlx::Executor;
use tauri::{plugin::TauriPlugin, AppHandle, Manager, Runtime};
use tauri_plugin_sql::{DbInstances, Migration, MigrationKind, PluginConfig};

const DB_PATH: &str = "sqlite:data.db";

pub fn init_sql<T: Runtime>() -> TauriPlugin<T, Option<PluginConfig>> {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: include_str!("sql/init_20240418.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create history relative tables",
            sql: include_str!("sql/init_20251121.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "create cache table",
            sql: include_str!("sql/init_20251124.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 4,
            description: "add order for prompt_config",
            sql: include_str!("sql/init_20251201.sql"),
            kind: MigrationKind::Up,
        },
    ];

    tauri_plugin_sql::Builder::default()
        .add_migrations(DB_PATH, migrations)
        .build()
}

pub async fn query_prompt_configs_count(app: &AppHandle) -> Result<usize> {
    let db_instances = app.state::<DbInstances>();
    let db_pool_map = db_instances.inner().0.read().await;

    let pool = db_pool_map.get(&DB_PATH.to_string());

    if pool.is_none() {
        return Ok(0);
    }

    let db = pool.unwrap();

    let size = match db {
        tauri_plugin_sql::DbPool::Sqlite(pool) => {
            let query = sqlx::query("select id from prompt_config");

            let r = pool.fetch_all(query).await?;
            r.len()
        }
    };

    Ok(size)
}
