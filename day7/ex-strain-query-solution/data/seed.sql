CREATE TABLE strain (
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
);

BEGIN;

INSERT INTO strain (strain_id, strain_name, species, isolation_source, year_isolated) VALUES
    (1,  'K-12 MG1655', 'Escherichia coli',       'human gut',        1922),
    (2,  'DH5alpha',    'Escherichia coli',       'lab derivative',   1980),
    (3,  'BL21(DE3)',   'Escherichia coli',       'lab derivative',   1986),
    (4,  'Newman',      'Staphylococcus aureus',  'clinical isolate', 1952),
    (5,  'USA300',      'Staphylococcus aureus',  'community MRSA',   2000),
    (6,  '168',         'Bacillus subtilis',      'soil',             1947),
    (7,  'UA159',       'Streptococcus mutans',   'oral isolate',     1980),
    (8,  'PAO1',        'Pseudomonas aeruginosa', 'wound',            1955),
    (9,  'SL1344',      'Salmonella enterica',    'calf',             1980),
    (10, 'W3110',       'Escherichia coli',       'lab derivative',   1966);

INSERT INTO assay (assay_id, strain_id, medium, od600_24h, date_measured) VALUES
    (1,  1,  'LB',         1.82, '2024-03-04'),
    (2,  1,  'M9-glucose', 0.94, '2024-03-04'),
    (3,  1,  'LB',         1.79, '2024-03-11'),
    (4,  2,  'LB',         1.65, '2024-03-04'),
    (5,  2,  'M9-glucose', 0.71, '2024-03-04'),
    (6,  3,  'LB',         1.93, '2024-03-04'),
    (7,  3,  'LB',         1.88, '2024-03-11'),
    (8,  3,  'M9-glucose', 0.62, '2024-03-04'),
    (9,  4,  'TSB',        1.41, '2024-03-05'),
    (10, 4,  'TSB',        1.39, '2024-03-12'),
    (11, 5,  'TSB',        1.55, '2024-03-05'),
    (12, 5,  'TSB',        1.61, '2024-03-12'),
    (13, 5,  'LB',         1.22, '2024-03-05'),
    (14, 6,  'LB',         1.74, '2024-03-06'),
    (15, 6,  'M9-glucose', 1.02, '2024-03-06'),
    (16, 7,  'BHI',        1.18, '2024-03-07'),
    (17, 7,  'BHI',        1.21, '2024-03-14'),
    (18, 8,  'LB',         1.97, '2024-03-08'),
    (19, 8,  'M9-glucose', 0.85, '2024-03-08'),
    (20, 9,  'LB',         1.71, '2024-03-09'),
    (21, 9,  'M9-glucose', 0.78, '2024-03-09'),
    (22, 10, 'LB',         1.69, '2024-03-04'),
    (23, 10, 'M9-glucose', 0.81, '2024-03-04'),
    (24, 10, 'LB',         1.73, '2024-03-11');

COMMIT;
