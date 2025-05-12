-- Table to store NFT transfer events
CREATE TABLE transfer (
    id TEXT NOT NULL,
    trx_hash TEXT NOT NULL,
    "from" TEXT NOT NULL,
    "to" TEXT NOT NULL,
    token_id BIGINT NOT NULL,
    ordinal BIGINT NOT NULL,
    PRIMARY KEY (id)
);

-- Table to store owner balances for tracked contracts
CREATE TABLE owner_count (
    id TEXT NOT NULL,
    contract TEXT NOT NULL,
    holder TEXT NOT NULL,
    balance BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    PRIMARY KEY (id)
);
