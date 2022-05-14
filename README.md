# 260r-Final-Project-Search-Engine

[Link to the presentation](https://docs.google.com/presentation/d/1prOULuMPMDCbr_X2Q-mrp0hG1oda7GuKV8LsiTpDwB4/edit?usp=sharing)

To run one of the binary targets, use the command `cargo run --release --bin COMMAND [-- ARGS]`.

List of targets:

- `parser` --- Used to parse the raw Bitcoin block data into various formats and configurations.
- `sqlite-baseline` --- A sqlite interface for querying data on a single machine.
- `search-worker` --- The worker in our distributed search engine.
- `search-master` --- The master in our distributed search engine.

To set-up the cluster:
- Spin up the number of workers + one master node
- Run the `search-worker` in each of the worker nodes until the terminal says it's listening
- Run the query in your master node to reach each of your worker nodes. To specify the worker clients and ports, list them sequentially `cargo run --release --bin search-master -- --client [IPADDR1] --port [PORT1] --client [IPADDR2] --port [PORT2]`.
