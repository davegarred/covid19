CREATE TABLE records (
    state text NOT NULL,
    country text NOT NULL,
    record_type varchar(9) NOT NULL,
    date varchar(8) NOT NULL,
    value int NOT NULL,
    PRIMARY KEY (state, country, date)
);