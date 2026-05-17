use rusqlite::{Connection, Result};
use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Strain {
    pub strain_id: i64,
    pub strain_name: String,
    pub species: String,
    pub isolation_source: Option<String>,
    pub year_isolated: Option<i64>,
}

/// Return every row in the `strain` table, ordered by `strain_id`.
///
/// TODO:
///   - prepare a SELECT over all five columns
///   - call query_map with an empty params slice ([])
///   - in the closure, build a Strain by reading each column with row.get(i)?
///   - collect the resulting iterator of Result<Strain> into a Result<Vec<Strain>>
pub fn list_strains(conn: &Connection) -> Result<Vec<Strain>> {
    let _ = conn;
    Ok(Vec::new())
}

/// Return only the strains whose species matches `species` exactly.
///
/// TODO:
///   - prepare a SELECT with a ?1 parameter on the species column
///   - bind `species` via [species] (or rusqlite::params![species])
///   - same row-mapping pattern as list_strains
pub fn strains_of_species(conn: &Connection, species: &str) -> Result<Vec<Strain>> {
    let _ = (conn, species);
    Ok(Vec::new())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("usage: strain-read <PATH-TO-DB> [species]");
        std::process::exit(1);
    }
    let conn = Connection::open(&args[1])?;

    let strains = if let Some(species) = args.get(2) {
        strains_of_species(&conn, species)?
    } else {
        list_strains(&conn)?
    };

    println!("{} strain(s)", strains.len());
    for s in &strains {
        println!(
            "  [{:>2}] {:<14} {:<25} {:?} ({:?})",
            s.strain_id, s.strain_name, s.species, s.isolation_source, s.year_isolated
        );
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
    fn list_returns_ten_strains() {
        let conn = seeded();
        let all = list_strains(&conn).unwrap();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn list_is_ordered_by_id() {
        let conn = seeded();
        let all = list_strains(&conn).unwrap();
        let ids: Vec<i64> = all.iter().map(|s| s.strain_id).collect();
        assert_eq!(ids, (1..=10).collect::<Vec<_>>());
    }

    #[test]
    fn first_row_is_k12() {
        let conn = seeded();
        let all = list_strains(&conn).unwrap();
        assert_eq!(all[0].strain_name, "K-12 MG1655");
        assert_eq!(all[0].species, "Escherichia coli");
        assert_eq!(all[0].isolation_source.as_deref(), Some("human gut"));
        assert_eq!(all[0].year_isolated, Some(1922));
    }

    #[test]
    fn filter_by_species_ecoli() {
        let conn = seeded();
        let ec = strains_of_species(&conn, "Escherichia coli").unwrap();
        assert_eq!(ec.len(), 4);
        for s in &ec {
            assert_eq!(s.species, "Escherichia coli");
        }
    }

    #[test]
    fn filter_by_species_no_match() {
        let conn = seeded();
        let none = strains_of_species(&conn, "Vibrio cholerae").unwrap();
        assert!(none.is_empty());
    }
}
