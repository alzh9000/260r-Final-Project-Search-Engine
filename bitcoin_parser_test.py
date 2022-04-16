# Using https://github.com/alecalve/python-bitcoin-blockchain-parser

import os
from blockchain_parser.blockchain import Blockchain

from pathlib import Path

parent_path = str(Path(__file__).parent)

os_parent_path = os.getcwd()

# We can get some samples of bitcoin data from https://learnmeabitcoin.com/technical/blkdat
# Instantiate the Blockchain by giving the path to the directory
# containing the .blk files created by bitcoind
block_path = os.path.abspath(parent_path + "/.bitcoin/blocks")
print(block_path)

with open(parent_path + "/.bitcoin/blocks/blk00000.blk", "r") as f:
    print(f.read())

blockchain = Blockchain(block_path)
print(os.listdir(block_path))
print(f"blockchain.get_unordered_blocks() -> {list(blockchain.get_unordered_blocks())}")
for block in blockchain.get_unordered_blocks():
    print(f"block {block}")
    for tx in block.transactions:
        for no, output in enumerate(tx.outputs):
            print(
                "tx=%s outputno=%d type=%s value=%s"
                % (tx.hash, no, output.type, output.value)
            )

