//! Durable delivery rows: the patch is stored before the target is touched.

use super::{DeliveryStatus, plan::Plan};
use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, params};
use std::path::Path;

/// Branch head most recently delivered into `root`, if any.
pub(super) fn last_delivered(db: &Connection, root: &Path, branch: &str) -> Result<Option<String>> {
    Ok(db
        .query_row(
            "SELECT head FROM deliveries WHERE target = ?1 AND branch = ?2
             AND status IN ('applied', 'already_present') ORDER BY id DESC LIMIT 1",
            params![root.to_string_lossy(), branch],
            |row| row.get(0),
        )
        .optional()?)
}

/// Insert a `pending` row holding the full patch and return its identifier.
pub(super) fn begin(db: &Connection, plan: &Plan) -> Result<i64> {
    db.execute(
        "INSERT INTO deliveries
         (created_at, target, branch, head, base, status, files, detail, patch)
         VALUES (?1, ?2, ?3, ?4, ?5, 'pending', ?6, '', ?7)",
        params![
            chrono::Utc::now().to_rfc3339(),
            plan.root.to_string_lossy(),
            plan.branch,
            plan.head,
            plan.base,
            serde_json::to_string(&plan.files)?,
            plan.patch,
        ],
    )?;
    Ok(db.last_insert_rowid())
}

/// Record the final outcome of a delivery started with [`begin`].
pub(super) fn finish(db: &Connection, id: i64, status: DeliveryStatus, detail: &str) -> Result<()> {
    db.execute(
        "UPDATE deliveries SET status = ?1, detail = ?2 WHERE id = ?3",
        params![status.as_str(), detail, id],
    )?;
    Ok(())
}
