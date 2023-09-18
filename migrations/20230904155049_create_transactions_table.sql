CREATE TABLE traders (
    public_key bytea PRIMARY KEY,
    trader_name text
);

CREATE TABLE transactions (
    transaction_hash bytea PRIMARY KEY,
    wallet_public_key bytea REFERENCES traders(public_key),
    transaction_data bytea, 
    nonce bigint,
    block_hash bytea,
    block_number bigint
);
