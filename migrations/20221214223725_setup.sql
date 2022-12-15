-- Add migration script here
CREATE TABLE "measurement" (
    "id"	INTEGER NOT NULL UNIQUE,
    "room"	TEXT NOT NULL,
    "device"	TEXT NOT NULL,
    "date_time"	TEXT NOT NULL,
    "temperature"	REAL NOT NULL,
    "humidity"	REAL NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
);
