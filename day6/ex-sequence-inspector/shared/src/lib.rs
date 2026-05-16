//! Types shared between the backend and the frontend.
//!
//! Both sides depend on this crate. The frontend imports it and compiles
//! to `wasm32-unknown-unknown`; the backend imports it and compiles
//! natively. Because the types live in one place, the JSON wire format
//! cannot drift between the two sides — if you change a field here, both
//! crates need to be rebuilt before they will exchange data again, and
//! the compiler will tell you exactly where.

use serde::{Deserialize, Serialize};

/// One entry in the sample list returned by `GET /api/samples`.
///
/// Just the id and a human-readable name — no sequence, so the list is
/// cheap to send even if there are many records or long sequences.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SampleSummary {
    pub id: String,
    pub name: String,
}

/// The full record returned by `GET /api/samples/{id}`.
///
/// Includes the DNA sequence as a `String` — the frontend converts it
/// to `&[u8]` on the fly when it needs to compute statistics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SampleRecord {
    pub id: String,
    pub name: String,
    pub sequence: String,
}
