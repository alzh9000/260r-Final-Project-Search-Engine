#!/usr/bin/python
import numpy as np
import pandas as pd
import sys
from blocktools import *
from block import Block, BlockHeader
import pandas as pd

# removes the max column limit for displaying on the screen
pd.set_option("display.max_columns", None)

import argparse

parser = argparse.ArgumentParser(description="Check the status of an asynchronous job.")
parser.add_argument(
    "-d",
    "--dat_file",
    type=str,
    required=False,
    default="actual_blk00000.dat",
    help="The blk*.dat file that you want to check the transaction data from",
)
parser.add_argument(
    "-b",
    "--num_blocks",
    type=int,
    required=False,
    default=10,
    help="The number of blocks to parse",
)
parser.add_argument(
    "-n",
    "--num_transactions",
    type=int,
    required=False,
    default=10,
    help="The number of transactions to parse",
)

args = parser.parse_args()

dat = args.dat_file
num_t = args.num_transactions


def parse(blockchain, blkNo):
    print("Parsing Block Chain block head, transaction etc.")
    continueParsing = True
    counter = 0
    blockchain.seek(0, 2)
    fSize = blockchain.tell() - 80  # Minus last Block header size for partial file
    blockchain.seek(0, 0)
    while continueParsing:
        block = Block(blockchain)
        continueParsing = block.continueParsing
        if continueParsing:
            block.toString()
            block_df = block.toDataFrame()
            print(block_df)
            filename = block.getFileName()
            block_df.to_csv("csv/" + filename + ".csv")
            block_df.to_pickle("pandas/" + filename + ".pkl")
        counter += 1
        print("#" * 20 + "Block counter No. %s" % counter + "#" * 20)
        # TODO: figure out why there's this limit of 0xFF for blkNo
        if counter >= blkNo and blkNo != 0xFF:
            continueParsing = False

    print("")
    print("Reached End of Field")
    print("Parsed %d blocks" % counter)


def main():
    # What's the difference between the number of blocks vs the number of transactions? I thought each block only had 1 transaction? But it seems like potentially there can be more than 1 transaction per block. In that case, do we want each row to still be just 1 transaction or 1 block?
    # I think we should do 1 block per CSV, then 1 row for each transaction in the block
    # TODO: figure out why there's this limit of 0xFF for blkNo
    blkNo = num_t if num_t < 0xFF else 0xFF

    with open(dat, "rb") as blockchain:
        parse(blockchain, blkNo)


if __name__ == "__main__":
    main()
