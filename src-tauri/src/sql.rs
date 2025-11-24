use tauri::{plugin::TauriPlugin, Runtime};
use tauri_plugin_sql::{Migration, MigrationKind, PluginConfig};

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
    ];

    tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:data.db", migrations)
        .build()
}
