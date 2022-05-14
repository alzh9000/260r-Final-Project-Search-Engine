.timer ON

-- SELECT * FROM transactions WHERE id = '0f1d7406160f976ab69458811a386ebe444fcc8bf9b36a7ac27641b8182f8ee1';

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