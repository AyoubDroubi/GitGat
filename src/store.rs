use crate::domain::{CatalogRepository, Workspace};
use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Catalog {
    db_path: PathBuf,
}

impl Catalog {
    pub fn open_default() -> Result<Self> {
        let base = data_home();
        std::fs::create_dir_all(&base)
            .with_context(|| format!("failed to create {}", base.display()))?;
        let catalog = Self {
            db_path: base.join("gitgat.db"),
        };
        catalog.migrate()?;
        Ok(catalog)
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let catalog = Self {
            db_path: path.into(),
        };
        catalog.migrate()?;
        Ok(catalog)
    }

    fn connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path).context("failed to open GitGat catalog")
    }

    fn migrate(&self) -> Result<()> {
        let connection = self.connection()?;
        connection.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA foreign_keys=ON;
             CREATE TABLE IF NOT EXISTS repositories(
               path TEXT PRIMARY KEY NOT NULL,
               name TEXT NOT NULL,
               favorite INTEGER NOT NULL DEFAULT 0,
               last_opened_unix INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS workspaces(
               id INTEGER PRIMARY KEY AUTOINCREMENT,
               name TEXT NOT NULL UNIQUE
             );
             CREATE TABLE IF NOT EXISTS workspace_repositories(
               workspace_id INTEGER NOT NULL,
               repository_path TEXT NOT NULL,
               PRIMARY KEY(workspace_id, repository_path),
               FOREIGN KEY(workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
             );
             CREATE TABLE IF NOT EXISTS operation_journal(
               id INTEGER PRIMARY KEY AUTOINCREMENT,
               repository_path TEXT NOT NULL,
               operation TEXT NOT NULL,
               details TEXT NOT NULL,
               created_unix INTEGER NOT NULL
             );",
        )?;
        Ok(())
    }

    pub fn touch_repository(&self, path: &Path) -> Result<()> {
        let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let name = canonical
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("Repository")
            .to_owned();
        self.connection()?.execute(
            "INSERT INTO repositories(path,name,favorite,last_opened_unix)
             VALUES(?1,?2,0,?3)
             ON CONFLICT(path) DO UPDATE SET name=excluded.name,last_opened_unix=excluded.last_opened_unix",
            params![canonical.to_string_lossy(), name, now_unix()],
        )?;
        Ok(())
    }

    pub fn repositories(&self) -> Result<Vec<CatalogRepository>> {
        let connection = self.connection()?;
        let mut statement =
            connection.prepare("SELECT path,name,favorite,last_opened_unix FROM repositories")?;
        let rows = statement.query_map([], |row| {
            Ok(CatalogRepository {
                path: PathBuf::from(row.get::<_, String>(0)?),
                name: row.get(1)?,
                favorite: row.get::<_, i64>(2)? != 0,
                last_opened_unix: row.get(3)?,
            })
        })?;
        let mut values = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        values.sort_by(|a, b| {
            b.favorite
                .cmp(&a.favorite)
                .then(b.last_opened_unix.cmp(&a.last_opened_unix))
                .then(a.name.cmp(&b.name))
        });
        Ok(values)
    }

    pub fn set_favorite(&self, path: &Path, favorite: bool) -> Result<()> {
        self.connection()?.execute(
            "UPDATE repositories SET favorite=?2 WHERE path=?1",
            params![path.to_string_lossy(), i64::from(favorite)],
        )?;
        Ok(())
    }

    pub fn relocate_repository(&self, old: &Path, new: &Path) -> Result<()> {
        let canonical_new = new
            .canonicalize()
            .with_context(|| format!("failed to resolve {}", new.display()))?;
        let connection = self.connection()?;
        let name = canonical_new
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("Repository")
            .to_owned();
        let changed = connection.execute(
            "UPDATE repositories SET path=?2,name=?3,last_opened_unix=?4 WHERE path=?1",
            params![
                old.to_string_lossy(),
                canonical_new.to_string_lossy(),
                name,
                now_unix()
            ],
        )?;
        if changed == 0 {
            anyhow::bail!("repository catalog entry was not found: {}", old.display());
        }
        connection.execute(
            "UPDATE workspace_repositories SET repository_path=?2 WHERE repository_path=?1",
            params![old.to_string_lossy(), canonical_new.to_string_lossy()],
        )?;
        Ok(())
    }

    pub fn create_workspace(&self, name: &str) -> Result<i64> {
        let connection = self.connection()?;
        connection.execute(
            "INSERT INTO workspaces(name) VALUES(?1) ON CONFLICT(name) DO NOTHING",
            params![name.trim()],
        )?;
        let id = connection.query_row(
            "SELECT id FROM workspaces WHERE name=?1",
            params![name.trim()],
            |row| row.get(0),
        )?;
        Ok(id)
    }

    pub fn add_to_workspace(&self, workspace_id: i64, repository: &Path) -> Result<()> {
        self.connection()?.execute(
            "INSERT OR IGNORE INTO workspace_repositories(workspace_id,repository_path) VALUES(?1,?2)",
            params![workspace_id, repository.to_string_lossy()],
        )?;
        Ok(())
    }

    pub fn workspaces(&self) -> Result<Vec<Workspace>> {
        let connection = self.connection()?;
        let mut statement = connection.prepare("SELECT id,name FROM workspaces ORDER BY name")?;
        let bases = statement
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut result = Vec::with_capacity(bases.len());
        for (id, name) in bases {
            let mut repos_statement = connection.prepare(
                "SELECT repository_path FROM workspace_repositories WHERE workspace_id=?1 ORDER BY repository_path",
            )?;
            let repositories = repos_statement
                .query_map(params![id], |row| {
                    Ok(PathBuf::from(row.get::<_, String>(0)?))
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            result.push(Workspace {
                id,
                name,
                repositories,
            });
        }
        Ok(result)
    }

    pub fn journal(&self, repository: &Path, operation: &str, details: &str) -> Result<()> {
        self.connection()?.execute(
            "INSERT INTO operation_journal(repository_path,operation,details,created_unix)
             VALUES(?1,?2,?3,?4)",
            params![repository.to_string_lossy(), operation, details, now_unix()],
        )?;
        Ok(())
    }

    pub fn recent_journal(&self, repository: &Path, limit: usize) -> Result<Vec<String>> {
        let connection = self.connection()?;
        let mut statement = connection.prepare(
            "SELECT operation,details,created_unix FROM operation_journal
             WHERE repository_path=?1 ORDER BY id DESC LIMIT ?2",
        )?;
        let values = statement
            .query_map(params![repository.to_string_lossy(), limit as i64], |row| {
                Ok(format!(
                    "{} | {} | {}",
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?
                ))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(values)
    }
}

fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn data_home() -> PathBuf {
    if cfg!(windows)
        && let Some(path) = std::env::var_os("LOCALAPPDATA")
    {
        return PathBuf::from(path).join("GitGat");
    }
    if let Some(path) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(path).join("gitgat");
    }
    if let Some(path) = std::env::var_os("HOME") {
        return PathBuf::from(path)
            .join(".local")
            .join("share")
            .join("gitgat");
    }
    std::env::temp_dir().join("gitgat")
}

#[cfg(test)]
mod tests {
    use super::Catalog;
    use tempfile::tempdir;

    #[test]
    fn persists_repositories_and_workspaces() {
        let root = tempdir().unwrap();
        let repo = root.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        let catalog = Catalog::open(root.path().join("catalog.db")).unwrap();
        catalog.touch_repository(&repo).unwrap();
        catalog.set_favorite(&repo, true).unwrap();
        let workspace = catalog.create_workspace("Daily").unwrap();
        catalog.add_to_workspace(workspace, &repo).unwrap();

        let repositories = catalog.repositories().unwrap();
        assert_eq!(repositories.len(), 1);
        assert!(repositories[0].favorite);
        let workspaces = catalog.workspaces().unwrap();
        assert_eq!(workspaces[0].repositories.len(), 1);
    }

    #[test]
    fn relocates_missing_repository_and_workspace_membership() {
        let root = tempdir().unwrap();
        let old = root.path().join("old-repo");
        let new = root.path().join("new-repo");
        std::fs::create_dir_all(&old).unwrap();
        let catalog = Catalog::open(root.path().join("catalog.db")).unwrap();
        catalog.touch_repository(&old).unwrap();
        let workspace = catalog.create_workspace("Moved").unwrap();
        catalog.add_to_workspace(workspace, &old).unwrap();

        std::fs::rename(&old, &new).unwrap();
        catalog.relocate_repository(&old, &new).unwrap();

        let repositories = catalog.repositories().unwrap();
        assert_eq!(repositories.len(), 1);
        assert_eq!(repositories[0].path, new.canonicalize().unwrap());
        let workspaces = catalog.workspaces().unwrap();
        assert_eq!(
            workspaces[0].repositories,
            vec![new.canonicalize().unwrap()]
        );
    }
}
