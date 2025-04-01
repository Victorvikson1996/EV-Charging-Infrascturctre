CREATE TABLE transactions (
                              id TEXT PRIMARY KEY,
                              charger_id TEXT NOT NULL,
                              energy REAL NOT NULL,
                              cost REAL NOT NULL,
                              timestamp TEXT NOT NULL
);