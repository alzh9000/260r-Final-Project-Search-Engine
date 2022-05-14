# 260r-Final-Project-Search-Engine

[Link to the presentation](https://docs.google.com/presentation/d/1prOULuMPMDCbr_X2Q-mrp0hG1oda7GuKV8LsiTpDwB4/edit?usp=sharing)

To run one of the binary targets, use the command `cargo run --release --bin COMMAND [-- ARGS]`.

List of targets:

- `parser` --- Used to parse the raw Bitcoin block data into various formats and configurations.
- `sqlite-baseline` --- A sqlite interface for querying data on a single machine.
- `search-worker` --- The worker in our distributed search engine.
- `search-master` --- The master in our distributed search engine.
