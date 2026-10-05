//! Version-one schema: small headers and individually addressable records.
pub(super) const SQL: &str = "
CREATE TABLE IF NOT EXISTS blobs (hash TEXT PRIMARY KEY, body TEXT NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS projections (
 session_id TEXT NOT NULL, name TEXT NOT NULL, seq INTEGER NOT NULL, end_seq INTEGER NOT NULL,
 body TEXT NOT NULL, PRIMARY KEY(session_id,name,seq)) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS projection_end ON projections(session_id,end_seq);
CREATE TABLE IF NOT EXISTS views (
 session_id TEXT NOT NULL, name TEXT NOT NULL, header TEXT NOT NULL, PRIMARY KEY(session_id,name)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS sessions (
 id TEXT PRIMARY KEY, revision INTEGER NOT NULL, header TEXT NOT NULL,
 message_count INTEGER NOT NULL, tool_count INTEGER NOT NULL);
CREATE TABLE IF NOT EXISTS constraints (
 session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
 seq INTEGER NOT NULL, excerpt TEXT NOT NULL, PRIMARY KEY(session_id,seq)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS events (
 session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
 revision INTEGER NOT NULL, kind INTEGER NOT NULL, seq INTEGER NOT NULL,
 op TEXT NOT NULL, body TEXT NOT NULL,
 PRIMARY KEY(session_id,revision,kind,seq,op)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS records (
 session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
 kind INTEGER NOT NULL, seq INTEGER NOT NULL, body TEXT NOT NULL,
 PRIMARY KEY(session_id, kind, seq)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS consumers (
 session_id TEXT NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
 name TEXT NOT NULL, seq INTEGER NOT NULL, revision INTEGER NOT NULL,
 PRIMARY KEY(session_id, name)) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS tool_calls (
 session_id TEXT NOT NULL, call_id TEXT NOT NULL, state TEXT NOT NULL,
 seq INTEGER NOT NULL, result_seq INTEGER, PRIMARY KEY(session_id, call_id)) WITHOUT ROWID;
CREATE INDEX IF NOT EXISTS call_sequence ON tool_calls(session_id,seq);
CREATE INDEX IF NOT EXISTS result_sequence ON tool_calls(session_id,result_seq);
CREATE INDEX IF NOT EXISTS session_updated ON sessions(json_extract(header,'$.updated_at') DESC);
CREATE INDEX IF NOT EXISTS session_workspace ON sessions(json_extract(header,'$.metadata.directory'),json_extract(header,'$.updated_at') DESC);
CREATE TABLE IF NOT EXISTS commits (
 session_id TEXT NOT NULL, revision INTEGER NOT NULL, nonce TEXT NOT NULL UNIQUE,
 PRIMARY KEY(session_id, revision)) WITHOUT ROWID;";
