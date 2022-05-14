# 260r-Final-Project-Search-Engine

To run the sqlite baseline, use command `cargo run --release --bin sqlite-baseline`.

[Link to the presentation](https://docs.google.com/presentation/d/1prOULuMPMDCbr_X2Q-mrp0hG1oda7GuKV8LsiTpDwB4/edit?usp=sharing)

## Azure Notes
All in the azure folder

`indivtasks.sh` shows a nice step by step of how to create a resource group and the pools needed to run jobs and tasks. However, can't figure out how to run rqlite installation steps on the VM through Azure CLI. Maybe as an individual task? Followed this: https://docs.microsoft.com/en-us/azure/batch/quick-create-cli

`sql.sh` shows how to create a sql server and sql database. Next question is how to load csv data or create a .bacpac (currently supported and have docs) to import into the database? This is for baseline comparison work as we query against the data in sql. Followed this: https://docs.microsoft.com/en-us/azure/azure-sql/database/scripts/create-and-configure-database-cli

Don't forget to run `az group delete --name [resource group name]` to delete all resources.
