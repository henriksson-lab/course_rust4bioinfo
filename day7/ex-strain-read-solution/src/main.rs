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

pub fn list_strains(conn: &Connection) -> Result<Vec<Strain>> {
    let mut stmt = conn.prepare(
        "SELECT strain_id, strain_name, species, isolation_source, year_isolated
         FROM strain
         ORDER BY strain_id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Strain {
            strain_id: row.get(0)?,
            strain_name: row.get(1)?,
            species: row.get(2)?,
            isolation_source: row.get(3)?,
            year_isolated: row.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn strains_of_species(conn: &Connection, species: &str) -> Result<Vec<Strain>> {
    let mut stmt = conn.prepare(
        "SELECT strain_id, strain_name, species, isolation_source, year_isolated
         FROM strain
         WHERE species = ?1
         ORDER BY strain_id",
    )?;
    let rows = stmt.query_map([species], |row| {
        Ok(Strain {
            strain_id: row.get(0)?,
            strain_name: row.get(1)?,
            species: row.get(2)?,
            isolation_source: row.get(3)?,
            year_isolated: row.get(4)?,
        })
    })?;
    rows.collect()
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
