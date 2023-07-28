I 'm going to try the following:

1. Bootstrap a local fuel network and pick the current block (call it block_a)
2. Send a transaction to the local node, making the blockchain jump 1 block ahead
3. Pick the new block at the tip (call it block_b)
4. Serialize both into some sort of file (TBD)

Then, in another flow, instance or program:

5. Instantiate a database starting with block_a
6. Instantiate an executor linked to this database
7. Extract the signed transactions from block_b and run
8. Check that the hashes are the same
