.timer ON

-- For transactions: where hash corresponds with the id, based on Maegan's Google Sheet https://docs.google.com/spreadsheets/d/1HTtOwd4fn4yLmi-K2uEudE5Fjd6uFV2pwZ87Bk638og/edit?usp=sharing  
-- One alone
SELECT * FROM transactions WHERE id='hash1';
-- Two together
SELECT * FROM transactions WHERE id='hash1' or id='hash2';
-- Three together
SELECT * FROM transactions WHERE id='hash1' or id='hash2' or id='hash3';
-- In theory, the N individually should have times that are around N * One alone, since they're just sequential SQL queries.
-- Two individually
SELECT * FROM transactions WHERE id='hash1';
SELECT * FROM transactions WHERE id='hash2';
-- Three individually
SELECT * FROM transactions WHERE id='hash1';
SELECT * FROM transactions WHERE id='hash2';
SELECT * FROM transactions WHERE id='hash3';

SELECT * FROM transactions LIMIT 5;


-- Outdated, ignore:

-- Getting the column names/schemas of the tables
-- SELECT *
-- FROM blocks
-- LIMIT 5;
-- SELECT *
-- FROM transactions
-- LIMIT 5;
-- SELECT *
-- FROM input_output_pairs
-- LIMIT 5;

-- EXPLAIN QUERY PLAN

-- SELECT *
-- FROM input_output_pairs
-- WHERE input_output_pairs.src_tx='��v�y��|�WX�bR_��zh2�b	]T�c�tsC'
-- LIMIT 500;

-- -- To find the parent transactions of X
-- SELECT *
-- FROM input_output_pairs
-- WHERE input_output_pairs.src_tx=X.dest_tx
-- LIMIT 500;