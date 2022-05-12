# 260r-Final-Project-Search-Engine

Link to the presentation: https://docs.google.com/presentation/d/1prOULuMPMDCbr_X2Q-mrp0hG1oda7GuKV8LsiTpDwB4/edit?usp=sharing 

Store some `blk*.dat` files in the `./.bitcoin/blocks directory`, so that you can see how to do it. `blk*.dat` files are binary, so you shouldn't be able to open them or read them. Instead, we need to use a parser. 

This kind of works but is maybe weird? To use PyBC, do `cd PyBC_260r`, then `python read_dat.py`. 

This works on at least Intel MacOs and WSL: To use blocktools, do `cd blocktools_py3`, then `python sight.py 1M.dat | less`. 

Since we converted blocktools to Python 3, we plan on using this over PyBC. 

`blocktools_py3/parse_and_store_data.py` is the primary file used to get data into CSV format. 

# I think we should do 1 block per CSV, then 1 row for each transaction in the block

Next steps: goal is to just get transaction data into a CSV or pandas dataframe format, where each transaction corresponds to one row, and there's 1 column for each data piece/attribute of the transaction. Then, once we have the ability to create these CSVs, we can just use them for the rest of the project and never have to touch the actual raw data blk.dat files. We can just put 10 or 100 or 1000 transactions in each CSV and store the CSV in some folder. The naming convention for these CSVs will be based on the structure that Savvy's downloaded data has. Or, we can do 1 CSV file per blk.dat file that Savvy has. So, we should figure out how the blk.dat files are stored for the data that Savvy downloaded, so we know if we need to traverse through a folder or multiple files to get all the blk.dat files, and also if any "ordering" or "naming" of these blk.dat files should affect the resulting CSVs. 

## Azure Notes
All in the azure folder

`indivtasks.sh` shows a nice step by step of how to create a resource group and the pools needed to run jobs and tasks. However, can't figure out how to run rqlite installation steps on the VM through Azure CLI. Maybe as an individual task? Followed this: https://docs.microsoft.com/en-us/azure/batch/quick-create-cli

`sql.sh` shows how to create a sql server and sql database. Next question is how to load csv data or create a .bacpac (currently supported and have docs) to import into the database? This is for baseline comparison work as we query against the data in sql. Followed this: https://docs.microsoft.com/en-us/azure/azure-sql/database/scripts/create-and-configure-database-cli

Don't forget to run `az group delete --name [resource group name]` to delete all resources. 