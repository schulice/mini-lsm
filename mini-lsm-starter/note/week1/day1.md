# Problem

1. Why doesn't the memtable provide a delete API?

	The memtable do not need to actually delete the kv, but only add new log that `k=<empty>`.

2. Is it possible to use other data structures as the memtable in LSM? What are the pros/cons of using the skiplist?

	Yes. The skiplist is ordered and easy to achieve lock-free.
	pors. simple lock-free design.
	cons. bad performance on unbalance workload

3. Why do we need a combination of state and state_lock? Can we only use state.read() and state.write()?

	Yes. state_lock only use when you need to change the state but not just access the async-safe member in it.

4. Why does the order to store and to probe the memtables matter? If a key appears in multiple memtables, which version should you return to the user?

	If you store and probe the tables in wrong order, you will get the oldest version of the key.
	The latest one, which is correspond to table or the head version of the immutable tables.

5. Is the memory layout of the memtable efficient / does it have good data locality? (Think of how Byte is implemented and stored in the skiplist...) What are the possible optimizations to make the memtable more efficient?

	Link table need to jump large distance to find the key.
	Some cache or memory block fit bucket to store the jump index.

6. So we are using parking_lot locks in this tutorial. Is its read-write lock a fair lock? What might happen to the readers trying to acquire the lock if there is one writer waiting for existing readers to stop?

	It has some advantages than std, like adaptive lock for small variable and so on. Yes, it is fair, and can avoid reader and writer stavation. 
	Due to the time writer waiting for. The fairness of the RWLock in parking_lot is achieve via eventual fairness on average every 0.5ms. So if writer waiting more than that time, the scheduler will let writer get the lock the refuse new-reader, else new-reader will get the shared lock and writer is still waiting.

7. After freezing the memtable, is it possible that some threads still hold the old LSM state and wrote into these immutable memtables? How does your solution prevent it from happening?

	No. After getting the state_lock, the write lock of memtable will reject other key writer.

8. There are several places that you might first acquire a read lock on state, then drop it and acquire a write lock (these two operations might be in different functions but they happened sequentially due to one function calls the other). How does it differ from directly upgrading the read lock to a write lock? Is it necessary to upgrade instead of acquiring and dropping and what is the cost of doing the upgrade?

	The upgrading will not drop the read lock.
	May not. In our implement, the thread with read lock still can insert the kv to skip list, and upgrading means longer time to hold the current read lock to get the write lock, which will not perform well on large amount of threads. 