// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/db/permissions_db.rs
// Description: Dedicated SQLite database for Hardware (Camera/Mic) & Geolocation permissions.
// ============================================================================

use chrono::Utc;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Permission decision state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionPolicy {
    Allow,
    Block,
    Ask,
}

impl PermissionPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionPolicy::Allow => "allow",
            PermissionPolicy::Block => "block",
            PermissionPolicy::Ask => "ask",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "allow" => PermissionPolicy::Allow,
            "block" => PermissionPolicy::Block,
            _ => PermissionPolicy::Ask,
        }
    }
}

/// Domain permission configuration.
///
/// I am writing this struct because users must be able to audit and control
/// exactly which websites have access to their internal or external cameras,
/// audio microphones, and physical location coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitePermission {
    pub id: i64,
    pub domain: String,
    pub camera: PermissionPolicy,
    pub microphone: PermissionPolicy,
    pub location: PermissionPolicy,
    pub last_updated: String,
}

/// Audit log record for hardware & location requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionAuditLog {
    pub id: i64,
    pub domain: String,
    pub permission_type: String,
    pub decision: String,
    pub timestamp: String,
}

/// Dedicated Permissions Database Manager.
///
/// I have written this dedicated SQLite database manager because hardware security
/// (webcam, mic, GPS sensor) is the highest risk vector for user privacy on the modern web.
/// By storing permissions in an isolated, tamper-evident database (`permissions.db`),
/// we guarantee that no malicious site script can spoof or tamper with device access rules.
#[derive(Clone)]
pub struct PermissionsDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl PermissionsDatabase {
    /// Opens or initializes the permissions database.
    ///
    /// I am writing this initialization routine to enforce WAL mode and initialize
    /// both the site rules table and the security audit log table.
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
        )?;

        // I have written this table to maintain persistent rules per origin domain.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS site_permissions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domain TEXT UNIQUE NOT NULL,
                camera TEXT NOT NULL DEFAULT 'ask',
                microphone TEXT NOT NULL DEFAULT 'ask',
                location TEXT NOT NULL DEFAULT 'ask',
                last_updated TEXT NOT NULL
            );",
            [],
        )?;

        // I am creating this audit table so users have full forensic visibility
        // into every single time a website attempted to invoke their mic, cam, or GPS.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS permission_audit (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domain TEXT NOT NULL,
                permission_type TEXT NOT NULL,
                decision TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_permissions_domain ON site_permissions(domain);",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Evaluates or retrieves the permission policy for a specific domain.
    ///
    /// I have written this lookup function because whenever WebKit triggers a media
    /// capture or geolocation request, we must instantaneously determine if the user
    /// previously granted or denied access for this domain.
    pub fn get_domain_policy(&self, domain: &str) -> Result<Option<SitePermission>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, domain, camera, microphone, location, last_updated
             FROM site_permissions
             WHERE domain = ?"
        )?;

        let mut rows = stmt.query([domain])?;
        if let Some(row) = rows.next()? {
            let camera_str: String = row.get(2)?;
            let mic_str: String = row.get(3)?;
            let loc_str: String = row.get(4)?;

            Ok(Some(SitePermission {
                id: row.get(0)?,
                domain: row.get(1)?,
                camera: PermissionPolicy::from_str(&camera_str),
                microphone: PermissionPolicy::from_str(&mic_str),
                location: PermissionPolicy::from_str(&loc_str),
                last_updated: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Sets or updates a permission rule for a domain.
    ///
    /// I am writing this update function using an UPSERT pattern so that
    /// the user can toggle permissions in the NORVA dashboard with atomic certainty.
    pub fn set_policy(
        &self,
        domain: &str,
        camera: PermissionPolicy,
        microphone: PermissionPolicy,
        location: PermissionPolicy,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO site_permissions (domain, camera, microphone, location, last_updated)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(domain) DO UPDATE SET
                camera = excluded.camera,
                microphone = excluded.microphone,
                location = excluded.location,
                last_updated = excluded.last_updated",
            params![domain, camera.as_str(), microphone.as_str(), location.as_str(), now],
        )?;

        Ok(())
    }

    /// Records an audit log entry when a hardware or location check occurs.
    ///
    /// I have written this audit logger because zero-trust architecture requires
    /// an immutable record of device activation attempts.
    pub fn log_audit(&self, domain: &str, permission_type: &str, decision: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO permission_audit (domain, permission_type, decision, timestamp)
             VALUES (?, ?, ?, ?)",
            params![domain, permission_type, decision, now],
        )?;

        Ok(())
    }

    /// Retrieves all configured site permissions for the dashboard.
    pub fn get_all_permissions(&self) -> Result<Vec<SitePermission>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, domain, camera, microphone, location, last_updated
             FROM site_permissions
             ORDER BY domain ASC"
        )?;

        let rows = stmt.query_map([], |row| {
            let camera_str: String = row.get(2)?;
            let mic_str: String = row.get(3)?;
            let loc_str: String = row.get(4)?;

            Ok(SitePermission {
                id: row.get(0)?,
                domain: row.get(1)?,
                camera: PermissionPolicy::from_str(&camera_str),
                microphone: PermissionPolicy::from_str(&mic_str),
                location: PermissionPolicy::from_str(&loc_str),
                last_updated: row.get(5)?,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            if let Ok(p) = item {
                list.push(p);
            }
        }
        Ok(list)
    }

    /// Deletes a domain rule, resetting it to default prompt.
    pub fn delete_domain(&self, domain: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM site_permissions WHERE domain = ?", [domain])?;
        Ok(())
    }

    /// Retrieves recent permission audit records.
    pub fn get_recent_audits(&self, limit: usize) -> Result<Vec<PermissionAuditLog>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, domain, permission_type, decision, timestamp
             FROM permission_audit
             ORDER BY id DESC
             LIMIT ?"
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            Ok(PermissionAuditLog {
                id: row.get(0)?,
                domain: row.get(1)?,
                permission_type: row.get(2)?,
                decision: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            if let Ok(l) = item {
                list.push(l);
            }
        }
        Ok(list)
    }
}
