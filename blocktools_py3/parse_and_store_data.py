#!/usr/bin/python
import sys
from blocktools import *
from block import Block, BlockHeader

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

# Add a parser argument for the .dat file name
parser.add_argument(
    "-t",
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
        counter += 1
        print("#" * 20 + "Block counter No. %s" % counter + "#" * 20)
        if counter >= blkNo and blkNo != 0xFF:
            continueParsing = False

    print("")
    print("Reached End of Field")
    print("Parsed %d blocks" % counter)


def main():
    # What's the difference between the number of blocks vs the number of transactions? I thought each block only had 1 transaction? But it seems like potentially there can be more than 1 transaction per block. In that case, do we want each row to still be just 1 transaction or 1 block? 
    blkNo = 0xFF
    blkNo = num_t
    
    with open(dat, "rb") as blockchain:
        parse(blockchain, blkNo)


if __name__ == "__main__":
    main()
