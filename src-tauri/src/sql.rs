use tauri_plugin_sql::{Builder, Migration, MigrationKind};

pub fn init_sql() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: include_str!("sql/init_20240418.sql"),
            kind: MigrationKind::Up,
        }
    ];

    tauri_plugin_sql::Builder::default()
        .add_migrations("sqlite:data.db", migrations)
        .build()
}