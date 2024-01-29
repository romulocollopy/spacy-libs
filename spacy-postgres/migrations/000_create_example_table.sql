BEGIN;

CREATE TABLE sku (
    id UUID NOT NULL,
    quantity BIGINT NOT NULL,
    name VARCHAR(50) NOT NULL,
    data JSON,
    PRIMARY KEY (id)
);

COMMIT;
