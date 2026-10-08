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

/// Permission decision state for hardware and sensor access.
///
/// I have written this enum to cleanly classify access policies:
/// - `Allow`: Explicitly trusted to access the physical sensor.
/// - `Block`: Explicitly blocked to protect hardware privacy.
/// - `Ask`: Zero-trust default; must prompt the user before activating the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionPolicy {
    Allow,
    Block,
    Ask,
}

impl PermissionPolicy {
    /// Returns the static lowercase string representation of this policy.
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionPolicy::Allow => "allow",
            PermissionPolicy::Block => "block",
            PermissionPolicy::Ask => "ask",
        }
    }

    /// Parses a string into a PermissionPolicy variant with a safe fallback to Ask.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "allow" => PermissionPolicy::Allow,
            "block" => PermissionPolicy::Block,
            _ => PermissionPolicy::Ask,
        }
    }
}

/// Domain permission configuration capturing hardware access rules per origin.
///
/// I have written this struct because users must be able to audit and control
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
///
/// I have written this audit structure to give users complete forensic transparency:
/// whenever a website calls WebRTC or Geolocation APIs, NORVA logs the exact domain,
/// the requested hardware device, the security decision, and the exact timestamp.
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
/// I have upgraded this dedicated database manager with high-speed WAL mode, memory temp
/// storage, and busy timeouts. I wrote this architecture because hardware device access
/// (internal webcams, external USB capture cards, XLR microphones, GPS sensors) represents
/// the highest physical security risk. Isolating these rules in `permissions.db` guarantees
/// that web scripts cannot bypass or corrupt hardware access arbiter rules.
#[derive(Clone)]
pub struct PermissionsDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl PermissionsDatabase {
    /// Opens or initializes the permissions database.
    ///
    /// I have written this initialization routine to enforce WAL mode and setup tables
    /// for persistent origin policies and forensic invocation audit trails.
    pub fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;

        // High-performance SQLite tuning: WAL mode, memory temp store, 64MB cache
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA temp_store = MEMORY;
             PRAGMA cache_size = -64000;
             PRAGMA busy_timeout = 5000;"
        )?;

        // Persistent per-origin hardware permissions
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

        // Forensic audit log of device activation requests
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

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_time ON permission_audit(timestamp DESC);",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Evaluates or retrieves the permission policy for a specific origin domain.
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

    /// Sets or updates the permission policy for a specific origin domain.
    ///
    /// I have written this upsert method so users can seamlessly update Camera,
    /// Microphone, and Location access with an atomic SQL statement.
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
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(domain) DO UPDATE SET
                camera = excluded.camera,
                microphone = excluded.microphone,
                location = excluded.location,
                last_updated = excluded.last_updated",
            params![domain, camera.as_str(), microphone.as_str(), location.as_str(), now],
        )?;

        Ok(())
    }

    /// Retrieves all registered site permissions for management on the dashboard.
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

        let mut perms = Vec::new();
        for r in rows {
            perms.push(r?);
        }

        Ok(perms)
    }

    /// Deletes a domain's custom permission policy, reverting it to zero-trust defaults.
    pub fn delete_domain(&self, domain: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM site_permissions WHERE domain = ?", [domain])?;
        Ok(())
    }

    /// Logs an access request to the forensic audit log.
    ///
    /// I have written this logging function so users have a verifiable record of when
    /// webcams, audio inputs, or GPS coordinates were requested or blocked.
    pub fn log_audit(&self, domain: &str, perm_type: &str, decision: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO permission_audit (domain, permission_type, decision, timestamp)
             VALUES (?, ?, ?, ?)",
            params![domain, perm_type, decision, now],
        )?;

        Ok(())
    }

    /// Retrieves recent forensic audit records.
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

        let mut audits = Vec::new();
        for r in rows {
            audits.push(r?);
        }

        Ok(audits)
    }

    /// Clears all historical forensic audit logs.
    pub fn clear_audits(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM permission_audit;", [])?;
        Ok(())
    }
}
