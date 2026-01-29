//! Database layer for Redice

use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use include_dir::{Dir, include_dir};
use rusqlite_migration::Migrations;

use std::{path::PathBuf, sync::OnceLock};
use tokio_rusqlite::Connection;

pub fn db_path_in_local_data() -> Result<PathBuf> {
    let proj = ProjectDirs::from("", "", "redice").ok_or_else(|| anyhow!("无法获取用户目录"))?;

    Ok(proj.data_local_dir().join("redice.db"))
}

static DB: OnceLock<Connection> = OnceLock::new();

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub async fn init_db() -> Result<()> {
    if DB.get().is_some() {
        return Ok(());
    }

    let path = db_path_in_local_data()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(path).await?;

    conn.call(|c| {
        c.pragma_update(None, "journal_mode", "WAL")?;
        c.pragma_update(None, "synchronous", "NORMAL")?;
        c.pragma_update(None, "foreign_keys", "ON")?;

        let migrations = Migrations::from_directory(&MIGRATIONS_DIR)?;
        migrations.to_latest(c)?;

        Ok::<_, rusqlite_migration::Error>(())
    })
    .await
    .map_err(|e| anyhow!(e))?;

    let _ = DB.set(conn);
    Ok(())
}

pub fn get_db() -> Option<&'static Connection> {
    DB.get()
}
