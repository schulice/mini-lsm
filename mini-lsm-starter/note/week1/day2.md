# Problem

1. What is the time/space complexity of using your merge iterator?

2. Why do we need a self-referential structure for memtable iterator?

3. If a key is removed (there is a delete tombstone), do you need to return it to the user? Where did you handle this logic?

4. If a key has multiple versions, will the user see all of them? Where did you handle this logic?

5. If we want to get rid of self-referential structure and have a lifetime on the memtable iterator (i.e., MemtableIterator<'a>, where 'a = memtable or LsmStorageInner lifetime), is it still possible to implement the scan functionality?

6. What happens if (1) we create an iterator on the skiplist memtable (2) someone inserts new keys into the memtable (3) will the iterator see the new key?

7. What happens if your key comparator cannot give the binary heap implementation a stable order?

8. Why do we need to ensure the merge iterator returns data in the iterator construction order?

9. Is it possible to implement a Rust-style iterator (i.e., next(&self) -> (Key, Value)) for LSM iterators? What are the pros/cons?

10. The scan interface is like fn scan(&self, lower: Bound<&[u8]>, upper: Bound<&[u8]>). How to make this API compatible with Rust-style range (i.e., key_a..key_b)? If you implement this, try to pass a full range .. to the interface and see what will happen.

11. The starter code provides the merge iterator interface to store Box<I> instead of I. What might be the reason behind that?
