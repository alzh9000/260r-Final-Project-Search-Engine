# -*- coding: utf-8 -*-
"""
Created on Sat Apr 21 19:17:53 2018

@author: gareth

Class that should behave the same as in Blocks, but without holding any data.
Here indexs to data locations in .dat files are held in ._[name]_i attributes.
Properties replace the ._[name] attributes with a get method.
.[name] properties are inherited from Blocks.


"""

# %% Imports

from pybit.py2.block import Block, Trans, TxIn, TxOut
from pybit.py2.chain_map import DatMap


# %% Lower level classes

class BlockMap(Block):
    """
    Test class to map to location in .dat file, rather than holding data in
    attributes
    """
    @property
    def _magic(self):
        """
        Get bytes at index - same for all ._properties
        """
        return self.read_range(r1=self._magic_i[0],
                               r2=self._magic_i[1])

    @property
    def _blockSize(self):
        return self.read_range(r1=self._blockSize_i[0],
                               r2=self._blockSize_i[1])

    @property
    def _version(self):
        return self.read_range(r1=self._version_i[0],
                               r2=self._version_i[1])

    @property
    def _prevHash(self):
        return self.read_range(r1=self._prevHash_i[0],
                               r2=self._prevHash_i[1])

    @property
    def _merkleRootHash(self):
        return self.read_range(r1=self._merkleRootHash_i[0],
                               r2=self._merkleRootHash_i[1])

    @property
    def _timestamp(self):
        return self.read_range(r1=self._timestamp_i[0],
                               r2=self._timestamp_i[1])

    @property
    def _nBits(self):
        return self.read_range(r1=self._nBits_i[0],
                               r2=self._nBits_i[1])

    @property
    def _nonce(self):
        return self.read_range(r1=self._nonce_i[0],
                               r2=self._nonce_i[1])

    @property
    def _nTransactions(self):
        return self.read_range(r1=self._nTransactions_i[0])

    def read_header(self):
        """
        Read the block header, store data indexs in ._[name]_i attributes
        """
        # Read magic number: 4 bytes
        self._magic_i = self.map_next(4)

        # Read block size: 4 bytes
        self._blockSize_i = self.map_next(4)

        # Read version: 4 bytes
        self._version_i = self.map_next(4)

        # Read the previous hash: 32 bytes
        self._prevHash_i = self.map_next(32)

        # Read the merkle root: 32 bytes
        self._merkleRootHash_i = self.map_next(32)

        # Read the time stamp: 32 bytes
        self._timestamp_i = self.map_next(4)

        # Read target difficulty: 4 bytes
        self._nBits_i = self.map_next(4)

        # Read the nonce: 4 bytes
        self._nonce_i = self.map_next(4)

        # Read the number of transactions: VarInt 1-9 bytes
        self._nTransactions_loc, self._nTransactions_i = self.map_var()

    def read_trans(self):
        """
        Read transaction information in block
        """
        self.trans = {}
        fr = self.cursor
        for t in range(self.nTransactions):

            # Make transaction objects and table
            trans = TransMap(self.mmap, fr,
                             verb=self.verb,
                             f=self.f)
            fr = trans.cursor
            self.trans[t] = trans

        self.cursor = fr


class TransMap(Trans):
    @property
    def _version(self):
        return self.read_range(r1=self._version_i[0],
                               r2=self._version_i[1])

    @property
    def _nInputs(self):
        return self.read_range(r1=self._nInputs_i[0])

    @property
    def _nOutputs(self):
        return self.read_range(r1=self._nOutputs_i[0])

    @property
    def _lockTime(self):
        return self.read_range(r1=self._lockTime_i[0],
                               r2=self._lockTime_i[1])

    def get_transaction(self):

        # Read the version: 4 bytes
        self._version_i = self.map_next(4)

        # Read number of inputs: VarInt 1-9 bytes (or CVarInt?)
        self._nInputs_loc, self._nInputs_i = self.map_var()

        # Read the inputs (variable bytes)
        inputs = []
        for inp in range(self.nInputs):
            txIn = TxInMap(self.mmap, self.cursor,
                           f=self.f)
            inputs.append(txIn)

            # Update cursor position to the end of this input
            self.cursor = txIn.cursor

        self.txIn = inputs

        # Read number of outputs: VarInt 1-9 bytes (or CVarInt?)
        self._nOutputs_loc, self._nOutputs_i = self.map_var()

        # Read the outputs (varible bytes)
        outputs = []
        for oup in range(self.nOutputs):
            txOut = TxOutMap(self.mmap, self.cursor,
                             f=self.f)
            outputs.append(txOut)

            # Update cursor position to the end of this output
            self.cursor = txOut.cursor

        self.txOut = outputs

        # Read the locktime (4 bytes)
        self._lockTime_i = self.map_next(4)

        # Record the end for refernece, remove later?
        self.end = self.cursor


class TxInMap(TxIn):
    @property
    def _prevOutput(self):
        return self.read_range(r1=self._prevOutput_i[0],
                               r2=self._prevOutput_i[1])

    @property
    def _prevIndex(self):
        return self.read_range(r1=self._prevIndex_i[0],
                               r2=self._prevIndex_i[1])

    @property
    def _scriptLength(self):
        return self.read_range(r1=self._scriptLength_i[0],
                               r2=self._scriptLength_i[1])

    @property
    def _scriptSig(self):
        return self.read_range(r1=self._scriptSig_i[0],
                               r2=self._scriptSig_i[1])

    @property
    def _sequence(self):
        return self.read_range(r1=self._sequence_i[0],
                               r2=self._sequence_i[1])

    def read_in(self):
        # TxIn:
        # Read the previous_output (input) hash: 34 bytess
        self._prevOutput_i = self.map_next(34)

        # Read the index of the previous output (input)
        self._prevIndex_i = self.read_next(4)

        # Read the script length: 1 byte
        self._scriptLength_i = self.map_next(1)

        # Read the script sig: Variable
        self._scriptSig_i = self.map_next(self.scriptLength)

        # Read sequence: 4 bytes
        self._sequence_i = self.map_next(4)


class TxOutMap(TxOut):

    @property
    def _value(self):
        return self.read_range(r1=self._value_i[0],
                               r2=self._value_i[1])

    @property
    def _pkScriptLen(self):
        return self.read_range(r1=self._pkScriptLen_i[0],
                               r2=self._pkScriptLen_i[1])

    @property
    def _pkScript(self):
        return self.read_range(r1=self._pkScript_i[0],
                               r2=self._pkScript_i[1])

    def read_out(self):
        # TxOut:
        # Read value in Satoshis: 8 bytes
        self._value_i = self.map_next(8)

        # pk script
        self._pkScriptLen_i = self.map_next(1)

        # Read the script: Variable
        self._pkScript_i = self.map_next(self.pkScriptLen)

        # Record end of transaction for debugging
        self.end = self.cursor


if __name__ == "__main__":
    ""

    # %% Create a map object

    f = 'Blocks/blk00000.dat'
    datm = DatMap(f,
                  verb=4)

    # %% Read next block

    datm.read_next_block()

    # Verify it's correct
    datm.blocks[0].api_verify()


