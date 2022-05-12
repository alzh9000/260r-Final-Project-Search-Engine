.timer ON

-- SELECT *
-- FROM blocks
-- LIMIT 5000;
-- SELECT *
-- FROM transactions
-- LIMIT 5;
-- SELECT *
-- FROM input_output_pairs
-- LIMIT 5;

-- EXPLAIN QUERY PLAN

SELECT *
FROM input_output_pairs
WHERE input_output_pairs.src_tx='��v�y��|�WX�bR_��zh2�b	]T�c�tsC'
LIMIT 500;

-- -- To find the parent transactions of X
-- SELECT *
-- FROM input_output_pairs
-- WHERE input_output_pairs.src_tx=X.dest_tx
-- LIMIT 500;