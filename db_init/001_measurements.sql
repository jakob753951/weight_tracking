DROP TABLE IF EXISTS measurements;

CREATE TABLE measurements (
    id SERIAL NOT NULL,
    weight FLOAT NOT NULL CHECK(weight > 0),
    measured_at TIMESTAMP NOT NULL DEFAULT NOW()
);
