I 'm going to try the following:

- [x] Bootstrap a local fuel network and pick the current block (call it block_a)
- [x] Send a transaction to the local node, making the blockchain jump 1 block ahead
- [x] Pick the new block at the tip (call it block_b)
- [ ] Serialize both into some sort of file (TBD) (WIP)

Then, in another flow, instance or program:

5. Instantiate a database starting with block_a
6. Instantiate an executor linked to this database
7. Extract the signed transactions from block_b and run
8. Check that the hashes are the same
