use rusqlite::{params, Connection, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct StrainInput {
    pub strain_name: String,
    pub species: String,
    pub isolation_source: Option<String>,
    pub year_isolated: Option<i64>,
}

pub fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE strain (
            strain_id        INTEGER PRIMARY KEY,
            strain_name      TEXT    NOT NULL,
            species          TEXT    NOT NULL,
            isolation_source TEXT,
            year_isolated    INTEGER
        );
        CREATE TABLE assay (
            assay_id       INTEGER PRIMARY KEY,
            strain_id      INTEGER NOT NULL REFERENCES strain(strain_id),
            medium         TEXT    NOT NULL,
            od600_24h      REAL,
            date_measured  TEXT
        );",
    )
}

pub fn parse_tsv<R: BufRead>(reader: R) -> std::io::Result<Vec<StrainInput>> {
    let mut out = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 || line.is_empty() {
            continue;
        }
        let mut parts = line.split('\t');
        let strain_name = parts.next().unwrap_or("").trim().to_string();
        let species = parts.next().unwrap_or("").trim().to_string();
        let source_raw = parts.next().unwrap_or("").trim();
        let year_raw = parts.next().unwrap_or("").trim();

        let isolation_source = if source_raw.is_empty() {
            None
        } else {
            Some(source_raw.to_string())
        };
        let year_isolated = if year_raw.is_empty() {
            None
        } else {
            Some(year_raw.parse::<i64>().map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            })?)
        };

        out.push(StrainInput { strain_name, species, isolation_source, year_isolated });
    }
    Ok(out)
}

pub fn insert_strains(conn: &mut Connection, strains: &[StrainInput]) -> Result<usize> {
    let tx = conn.transaction()?;
    let mut n = 0;
    {
        let mut stmt = tx.prepare(
            "INSERT INTO strain (strain_name, species, isolation_source, year_isolated)
             VALUES (?1, ?2, ?3, ?4)",
        )?;
        for s in strains {
            stmt.execute(params![
                s.strain_name,
                s.species,
                s.isolation_source,
                s.year_isolated,
            ])?;
            n += 1;
        }
    }
    tx.commit()?;
    Ok(n)
}

pub fn build_database(db_path: &Path, tsv_path: &Path) -> Result<usize> {
    if db_path.exists() {
        std::fs::remove_file(db_path).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    }
    let mut conn = Connection::open(db_path)?;
    create_schema(&conn)?;
    let file = File::open(tsv_path).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let strains = parse_tsv(BufReader::new(file))
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    insert_strains(&mut conn, &strains)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: strain-write <PATH-TO-TSV> <PATH-TO-DB>");
        std::process::exit(1);
    }
    let n = build_database(Path::new(&args[2]), Path::new(&args[1]))?;
    println!("inserted {} strains into {}", n, &args[2]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE_TSV: &str = "\
strain_name\tspecies\tisolation_source\tyear_isolated
K-12 MG1655\tEscherichia coli\thuman gut\t1922
USA300\tStaphylococcus aureus\tcommunity MRSA\t2000
ToyA\tToy species\t\t
";

    #[test]
    fn parse_tsv_basic() {
        let parsed = parse_tsv(Cursor::new(SAMPLE_TSV)).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0].strain_name, "K-12 MG1655");
        assert_eq!(parsed[0].species, "Escherichia coli");
        assert_eq!(parsed[0].isolation_source.as_deref(), Some("human gut"));
        assert_eq!(parsed[0].year_isolated, Some(1922));
    }

    #[test]
    fn parse_tsv_handles_blank_optional_fields() {
        let parsed = parse_tsv(Cursor::new(SAMPLE_TSV)).unwrap();
        assert_eq!(parsed[2].strain_name, "ToyA");
        assert_eq!(parsed[2].isolation_source, None);
        assert_eq!(parsed[2].year_isolated, None);
    }

    #[test]
    fn schema_creates_strain_table() {
        let conn = Connection::open_in_memory().unwrap();
        create_schema(&conn).unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='strain'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn insert_round_trips() {
        let mut conn = Connection::open_in_memory().unwrap();
        create_schema(&conn).unwrap();
        let strains = parse_tsv(Cursor::new(SAMPLE_TSV)).unwrap();
        let n = insert_strains(&mut conn, &strains).unwrap();
        assert_eq!(n, 3);

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM strain", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 3);

        let (name, year): (String, Option<i64>) = conn
            .query_row(
                "SELECT strain_name, year_isolated FROM strain WHERE species = ?1",
                ["Staphylococcus aureus"],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(name, "USA300");
        assert_eq!(year, Some(2000));
    }

    #[test]
    fn insert_handles_null_year() {
        let mut conn = Connection::open_in_memory().unwrap();
        create_schema(&conn).unwrap();
        let strains = parse_tsv(Cursor::new(SAMPLE_TSV)).unwrap();
        insert_strains(&mut conn, &strains).unwrap();

        let year: Option<i64> = conn
            .query_row(
                "SELECT year_isolated FROM strain WHERE strain_name = 'ToyA'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(year, None);
    }
}
