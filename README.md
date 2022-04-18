# 260r-Final-Project-Search-Engine

Store some `blk*.dat` files in the `./.bitcoin/blocks directory`, so that you can see how to do it. `blk*.dat` files are binary, so you shouldn't be able to open them or read them. Instead, we need to use a parser. 

This kind of works but is maybe weird? To use PyBC, do `cd PyBC_260r`, then `python read_dat.py`. 

This works (but only in Python 2): To use blocktools, do `cd blocktools`, then `python2 sight.py 1M.dat`. 