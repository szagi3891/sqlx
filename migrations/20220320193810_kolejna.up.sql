CREATE TABLE IF NOT EXISTS todos
(
    id          BIGSERIAL PRIMARY KEY,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE,
    age integer,
    age2 integer,
    age3 integer
);