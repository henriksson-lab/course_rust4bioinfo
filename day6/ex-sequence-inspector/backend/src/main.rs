use axum::{extract::Path, routing::get, Json, Router};
use shared::{SampleRecord, SampleSummary};

// A small built-in library of sample DNA records. In a real tool this
// would be a database query or a file scan; for the course it lives in
// code so students do not need to install or download anything extra.
fn samples() -> Vec<SampleRecord> {
    vec![
        SampleRecord {
            id: "lambda-fragment".into(),
            name: "Phage lambda — 100 bp fragment".into(),
            sequence: "GCTCAGCGAAGCTGGGAACGAATTGCAACAGTAATCATTTGCTACGGCCGGCCTGGGAAGACATCAATTCCAGGCATGGCCATATCATGGTGGCCATGGCG".into(),
        },
        SampleRecord {
            id: "mt-fragment".into(),
            name: "Human mitochondrion — 100 bp fragment".into(),
            sequence: "GATCACAGGTCTATCACCCTATTAACCACTCACGGGAGCTCTCCATGCATTTGGTATTTTCGTCTGGGGGGTGTGCACGCGATAGCATTGCGAGACGCTGG".into(),
        },
        SampleRecord {
            id: "synth-at-rich".into(),
            name: "Synthetic AT-rich sequence".into(),
            sequence: "AAAATTTTAAAATTTTAAAATTTTAAAATTTTAAAATTTTAAAATTTTAAAATTTTAAAATTTT".into(),
        },
        SampleRecord {
            id: "synth-gc-rich".into(),
            name: "Synthetic GC-rich sequence".into(),
            sequence: "GGGGCCCCGGGGCCCCGGGGCCCCGGGGCCCCGGGGCCCCGGGGCCCCGGGGCCCCGGGGCCCC".into(),
        },
        SampleRecord {
            id: "synth-with-n".into(),
            name: "Synthetic — with N bases".into(),
            sequence: "ACGTNNNNACGTNNNNACGTNNNNACGTNNNNACGTACGTACGTACGTACGTACGTACGTACGT".into(),
        },
    ]
}

/// `GET /api/samples` — list every record without its sequence.
async fn list_samples() -> Json<Vec<SampleSummary>> {
    let summaries = samples()
        .into_iter()
        .map(|r| SampleSummary {
            id: r.id,
            name: r.name,
        })
        .collect();
    Json(summaries)
}

/// `GET /api/samples/{id}` — return one full record (with sequence) or 404.
async fn get_sample(
    Path(id): Path<String>,
) -> Result<Json<SampleRecord>, axum::http::StatusCode> {
    samples()
        .into_iter()
        .find(|r| r.id == id)
        .map(Json)
        .ok_or(axum::http::StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/api/samples", get(list_samples))
        .route("/api/samples/{id}", get(get_sample));
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("backend listening on http://{}", addr);
    axum::serve(listener, app).await
}
