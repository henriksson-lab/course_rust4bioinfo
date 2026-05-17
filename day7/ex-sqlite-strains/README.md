# Day 7 — SQLite CLI workspace

The first half of day 7 uses the `sqlite3` interactive shell directly. There is no Rust crate here, just a small bundle of sample data:

```
data/
  strains.csv     # 10 microbial strains
  assays.csv      # 24 growth-rate measurements across them
  schema.sql      # CREATE TABLE statements for both
  seed.sql        # schema + INSERTs in one file (for resetting)
```

Used by:

- [`day7/01-sqlite-cli.qmd`](../01-sqlite-cli.qmd) — create a table, insert rows, query.
- [`day7/02-natural-join.qmd`](../02-natural-join.qmd) — add a second table, join across them.

To wipe the slate and start over with a fresh database loaded from `seed.sql`:

```bash
rm -f strains.db
sqlite3 strains.db < data/seed.sql
```
