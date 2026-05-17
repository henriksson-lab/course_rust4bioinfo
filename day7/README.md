# Day 7 — Exercises

A first day of SQL and SQLite, then a first day of talking to SQLite from Rust. The morning runs entirely inside the `sqlite3` CLI; the afternoon is three small `rusqlite` programs that read, write, and join the same strain database.

Before you start you need the `sqlite3` command on your `PATH` — see [**Installing SQLite**](../intro/install-sqlite.qmd).

Start with [**00 — Concepts**](00-concepts.qmd) for a reference overview of relational tables, the bits of SQL we use today, and the `rusqlite` API (companion to the slides).

| # | Topic | Tool | Path |
|---|---|---|---|
| 0 | [**Concepts**](00-concepts.qmd) | tables, INSERT/SELECT, joins, `rusqlite` | — |
| 1 | [Your first SQLite database](01-sqlite-cli.qmd) | `sqlite3` CLI | [`ex-sqlite-strains/`](ex-sqlite-strains/) |
| 2 | [A second table and a join](02-natural-join.qmd) | `sqlite3` CLI | [`ex-sqlite-strains/`](ex-sqlite-strains/) |
| 3 | [Reading from Rust](03-read-from-rust.qmd) | `rusqlite` | [`ex-strain-read/`](ex-strain-read/) |
| 4 | [Writing from Rust](04-write-from-rust.qmd) | `rusqlite` | [`ex-strain-write/`](ex-strain-write/) |
| 5 | [Joining from Rust](05-query-from-rust.qmd) | `rusqlite` | [`ex-strain-query/`](ex-strain-query/) |

## Working pattern

The morning shares one folder, [`ex-sqlite-strains/`](ex-sqlite-strains/), with sample CSVs and a `seed.sql` script that builds the whole database in one shot:

```bash
cd day7/ex-sqlite-strains
rm -f strains.db
sqlite3 strains.db < data/seed.sql
sqlite3 strains.db                 # open the interactive shell
```

The afternoon has one crate per exercise. Each exercise's `data/seed.sql` is identical, so unit tests in `mod tests` build an in-memory database from `include_str!("../data/seed.sql")` and never touch disk:

```bash
cd day7/ex-strain-read         # or ex-strain-write, ex-strain-query
cargo test                     # uses an in-memory DB
cargo run -- data/strains.db   # uses a real file (build it once: sqlite3 ... < data/seed.sql)
```

All `rusqlite` crates declare the `bundled` feature so SQLite is compiled in — no system library required.
