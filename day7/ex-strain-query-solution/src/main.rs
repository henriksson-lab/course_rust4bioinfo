use rusqlite::{Connection, Result};
use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct AssayRow {
    pub strain_name: String,
    pub species: String,
    pub medium: String,
    pub od600_24h: Option<f64>,
    pub date_measured: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpeciesSummary {
    pub species: String,
    pub n_strains: i64,
    pub n_assays: i64,
    pub mean_od: Option<f64>,
}

pub fn assays_for_species(conn: &Connection, species: &str) -> Result<Vec<AssayRow>> {
    let mut stmt = conn.prepare(
        "SELECT s.strain_name, s.species, a.medium, a.od600_24h, a.date_measured
         FROM strain AS s
         JOIN assay  AS a ON a.strain_id = s.strain_id
         WHERE s.species = ?1
         ORDER BY s.strain_name, a.date_measured",
    )?;
    let rows = stmt.query_map([species], |row| {
        Ok(AssayRow {
            strain_name:   row.get(0)?,
            species:       row.get(1)?,
            medium:        row.get(2)?,
            od600_24h:     row.get(3)?,
            date_measured: row.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn species_summary(conn: &Connection) -> Result<Vec<SpeciesSummary>> {
    let mut stmt = conn.prepare(
        "SELECT s.species,
                COUNT(DISTINCT s.strain_id) AS n_strains,
                COUNT(a.assay_id)           AS n_assays,
                AVG(a.od600_24h)            AS mean_od
         FROM strain AS s
         LEFT JOIN assay AS a ON a.strain_id = s.strain_id
         GROUP BY s.species
         ORDER BY s.species",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(SpeciesSummary {
            species:   row.get(0)?,
            n_strains: row.get(1)?,
            n_assays:  row.get(2)?,
            mean_od:   row.get(3)?,
        })
    })?;
    rows.collect()
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: strain-query <PATH-TO-DB> [species]");
        std::process::exit(1);
    }
    let conn = Connection::open(&args[1])?;

    if let Some(species) = args.get(2) {
        let rows = assays_for_species(&conn, species)?;
        println!("{} assay(s) for {}", rows.len(), species);
        for r in &rows {
            println!(
                "  {:<14} {:<11} OD={:>5}  on {}",
                r.strain_name,
                r.medium,
                r.od600_24h.map(|v| format!("{:.2}", v)).unwrap_or_else(|| "?".into()),
                r.date_measured.as_deref().unwrap_or("?"),
            );
        }
    } else {
        let summary = species_summary(&conn)?;
        println!("{:<28} {:>9} {:>8} {:>8}", "species", "n_strains", "n_assays", "mean_od");
        for s in &summary {
            println!(
                "{:<28} {:>9} {:>8} {:>8}",
                s.species,
                s.n_strains,
                s.n_assays,
                s.mean_od.map(|v| format!("{:.2}", v)).unwrap_or_else(|| "-".into()),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../data/seed.sql")).unwrap();
        conn
    }

    #[test]
    fn assays_for_ecoli() {
        let conn = seeded();
        let rows = assays_for_species(&conn, "Escherichia coli").unwrap();
        assert_eq!(rows.len(), 11);
        for r in &rows {
            assert_eq!(r.species, "Escherichia coli");
        }
    }

    #[test]
    fn assays_for_unknown_species_is_empty() {
        let conn = seeded();
        let rows = assays_for_species(&conn, "Vibrio cholerae").unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn summary_has_every_species() {
        let conn = seeded();
        let s = species_summary(&conn).unwrap();
        let names: Vec<&str> = s.iter().map(|r| r.species.as_str()).collect();
        assert!(names.contains(&"Escherichia coli"));
        assert!(names.contains(&"Staphylococcus aureus"));
        assert!(names.contains(&"Bacillus subtilis"));
        assert!(names.contains(&"Pseudomonas aeruginosa"));
    }

    #[test]
    fn summary_counts_for_ecoli() {
        let conn = seeded();
        let s = species_summary(&conn).unwrap();
        let ec = s.iter().find(|r| r.species == "Escherichia coli").unwrap();
        assert_eq!(ec.n_strains, 4);
        assert_eq!(ec.n_assays, 11);
        let mean = ec.mean_od.unwrap();
        assert!((mean - 1.4155).abs() < 0.01, "mean was {mean}");
    }

    #[test]
    fn summary_handles_species_with_no_assays() {
        let conn = seeded();
        conn.execute(
            "INSERT INTO strain (strain_name, species) VALUES ('NoData1', 'Vibrio cholerae')",
            [],
        )
        .unwrap();
        let s = species_summary(&conn).unwrap();
        let vc = s
            .iter()
            .find(|r| r.species == "Vibrio cholerae")
            .expect("species with zero assays should still appear");
        assert_eq!(vc.n_strains, 1);
        assert_eq!(vc.n_assays, 0);
        assert!(vc.mean_od.is_none());
    }
}
