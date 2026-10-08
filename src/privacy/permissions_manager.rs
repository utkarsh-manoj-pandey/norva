// ============================================================================
// NORVA: Native Open-source Rust-based Virtual Access
// File: src/privacy/permissions_manager.rs
// Description: Hardware (Internal & External Camera / Microphone) and Geolocation arbiter.
// ============================================================================

use crate::db::permissions_db::{PermissionPolicy, PermissionsDatabase};
use std::sync::Arc;
use url::Url;
use wry::{PermissionKind, PermissionResponse};

/// Hardware & Location Permissions Arbiter.
///
/// I have written this arbiter because camera, microphone, and location sensors represent
/// the most intrusive physical hardware attack surface on any operating system.
/// Whether a user has an integrated laptop webcam, an external USB podcast microphone,
/// or an external DSLR connected via capture card, NORVA strictly verifies whether the
/// calling origin has been explicitly permitted before activating hardware devices.
#[derive(Clone)]
pub struct PermissionsManager {
    permissions_db: Arc<PermissionsDatabase>,
}

impl PermissionsManager {
    /// Creates a new permissions manager connected to the permissions database.
    pub fn new(permissions_db: Arc<PermissionsDatabase>) -> Self {
        Self { permissions_db }
    }

    /// Evaluates an incoming WebKit permission request for Camera, Mic, or Geolocation.
    ///
    /// I have upgraded this evaluation routine to query the isolated `permissions.db`
    /// with zero-trust defaults:
    /// - If a site rule exists (Allow or Block), it is obeyed immediately.
    /// - If no rule exists, it defaults safely to Deny (zero-trust security) to prevent
    ///   stealth background activations.
    /// - Every request is logged to the forensic audit log for transparent user inspection.
    pub fn handle_permission_request(
        &self,
        kind: PermissionKind,
        current_url: Option<&str>,
    ) -> PermissionResponse {
        let domain = current_url
            .and_then(|u| Url::parse(u).ok())
            .and_then(|u| u.host_str().map(|h| h.to_lowercase()))
            .unwrap_or_else(|| "unknown".to_string());

        let perm_type_str = match kind {
            PermissionKind::Camera => "camera",
            PermissionKind::Microphone => "microphone",
            PermissionKind::Geolocation => "location",
            _ => "other",
        };

        // Internal NORVA pages and local developer test endpoints have full native clearance
        if domain == "norva" || domain == "localhost" {
            let _ = self.permissions_db.log_audit(&domain, perm_type_str, "allowed_internal");
            return PermissionResponse::Allow;
        }

        let site_policy = self.permissions_db.get_domain_policy(&domain).ok().flatten();

        let decision = match kind {
            PermissionKind::Camera => {
                // I have written this camera policy branch to handle internal webcams and external video capture
                match site_policy.as_ref().map(|p| p.camera).unwrap_or(PermissionPolicy::Ask) {
                    PermissionPolicy::Allow => PermissionResponse::Allow,
                    PermissionPolicy::Block => PermissionResponse::Deny,
                    PermissionPolicy::Ask => {
                        // In non-interactive callbacks, default to safe Deny to prevent rogue background activations
                        PermissionResponse::Deny
                    }
                }
            }
            PermissionKind::Microphone => {
                // I have written this microphone branch to manage internal mics and external USB/XLR inputs
                match site_policy.as_ref().map(|p| p.microphone).unwrap_or(PermissionPolicy::Ask) {
                    PermissionPolicy::Allow => PermissionResponse::Allow,
                    PermissionPolicy::Block => PermissionResponse::Deny,
                    PermissionPolicy::Ask => PermissionResponse::Deny,
                }
            }
            PermissionKind::Geolocation => {
                // I have written this location branch to prevent stealth geo-triangulation
                match site_policy.as_ref().map(|p| p.location).unwrap_or(PermissionPolicy::Ask) {
                    PermissionPolicy::Allow => PermissionResponse::Allow,
                    PermissionPolicy::Block => PermissionResponse::Deny,
                    PermissionPolicy::Ask => PermissionResponse::Deny,
                }
            }
            _ => PermissionResponse::Deny,
        };

        let decision_str = match decision {
            PermissionResponse::Allow => "allowed",
            PermissionResponse::Deny => "blocked",
            _ => "default",
        };

        // Persist audit trail of device invocation
        let _ = self.permissions_db.log_audit(&domain, perm_type_str, decision_str);

        decision
    }
}
