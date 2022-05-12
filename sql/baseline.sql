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
WHERE input_output_pairs.value = 3342000000
LIMIT 5;