# -*- coding: utf-8 -*-
# %% Imports

from pybit.py3.chain_map import DatMap

# %% Map a .dat

path = 'pybit/Blocks/'
f = 'blk00003.dat'
dat = DatMap(path, f,
             verb=6)

# %% Read next block

# Read the block
dat.read_next_block(500)
# dat.read_all()

# %%
# Verify it's correct (this may already have been done on import)
dat.blocks[0].api_verify()

# %% Print example transaction

print(dat.blocks[0].trans[0])
