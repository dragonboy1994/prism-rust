use crate::crypto::hash::{Hashable, H256};
use crate::transaction::{CoinId, Input, Transaction};
use log::trace;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time;
use rand::Rng;

/// transactions storage
#[derive(Debug)]
pub struct MemoryPool {
    /// Number of transactions
    num_transactions: u64,
    /// Maximum number that the memory pool can hold
    max_transactions: u64,
    /// Counter for storage index
    counter: u64,
    /// By-hash storage
    by_hash: HashMap<H256, Entry>,
    /// Transactions by previous output, formatted as Input
    by_input: HashMap<Input, H256>,
    /// Storage for order by storage index, it is equivalent to FIFO
    by_storage_index: BTreeMap<u64, H256>,
    /// Max. jitter duration
    jitter_max: u64,
}

#[derive(Debug, Clone)]
pub struct Entry {
    /// Transaction
    pub transaction: Transaction,
    /// counter of the tx
    storage_index: u64,
    effective_from: time::Instant,
}

impl MemoryPool {
    pub fn new(size_limit: u64, jitter_max: u64) -> Self {
        Self {
            num_transactions: 0,
            max_transactions: size_limit,
            counter: 0,
            by_hash: HashMap::new(),
            by_input: HashMap::new(),
            by_storage_index: BTreeMap::new(),
            jitter_max: jitter_max,
        }
    }

    /// Insert a tx into memory pool. The input of it will also be recorded.
    pub fn insert(&mut self, tx: Transaction) {
        let mut rng = rand::thread_rng();
        let jitter_ms: u64 = if self.jitter_max == 0 {
            0
        } else 
        {
            rng.gen_range(0, self.jitter_max)
        };
        let effective = time::Instant::now() + time::Duration::from_millis(jitter_ms);
        if self.num_transactions > self.max_transactions {
            return;
        }
        // assumes no duplicates nor double spends
        let hash = tx.hash();
        let entry = Entry {
            transaction: tx,
            storage_index: self.counter,
            effective_from: effective,
        };
        self.counter += 1;

        // associate all inputs with this transaction
        for input in &entry.transaction.input {
            self.by_input.insert(input.clone(), hash);
        }

        // add to btree
        self.by_storage_index.insert(entry.storage_index, hash);

        // add to hashmap
        self.by_hash.insert(hash, entry);

        self.num_transactions += 1;
    }

    pub fn get(&self, h: &H256) -> Option<&Entry> {
        let entry = self.by_hash.get(h)?;
        Some(entry)
    }

    /// Check whether a tx hash is in memory pool
    /// When adding tx into mempool, should check this.
    pub fn contains(&self, h: &H256) -> bool {
        self.by_hash.contains_key(h)
    }

    /// Check whether the input of a tx is already recorded. If so, this tx is a double spend.
    /// When adding tx into mempool, should check this.
    pub fn is_double_spend(&self, inputs: &[Input]) -> bool {
        inputs.iter().any(|input| self.by_input.contains_key(input))
    }

    fn remove_and_get(&mut self, hash: &H256) -> Option<Entry> {
        let entry = self.by_hash.remove(hash)?;
        for input in &entry.transaction.input {
            self.by_input.remove(&input);
        }
        self.by_storage_index.remove(&entry.storage_index);
        self.num_transactions -= 1;
        Some(entry)
    }

    /// Remove a tx by its hash, also remove its recorded inputs
    pub fn remove_by_hash(&mut self, hash: &H256) {
        self.remove_and_get(hash);
    }

    /// Remove potential tx that use this input.
    /// This function runs recursively, so it may remove more transactions.
    pub fn remove_by_input(&mut self, prevout: &Input) -> u64 {
        //use a deque to recursively remove, in case there are multi level dependency between txs.
        let mut num_removed: u64 = 0;
        let mut queue: VecDeque<Input> = VecDeque::new();
        queue.push_back(prevout.clone());

        while let Some(prevout) = queue.pop_front() {
            if let Some(entry_hash) = self.by_input.get(&prevout) {
                let entry_hash = *entry_hash;
                let entry = self.remove_and_get(&entry_hash).unwrap();
                num_removed += 1;
                for (index, output) in entry.transaction.output.iter().enumerate() {
                    queue.push_back(Input {
                        coin: CoinId {
                            hash: entry_hash,
                            index: index as u32,
                        },
                        value: output.value,
                        owner: output.recipient,
                    });
                }
            }
        }
        return num_removed;
    }

    /// get n transaction by fifo
    pub fn get_transactions(&self, n: u32) -> Vec<Transaction> {
        let mut r: Vec<Transaction> = vec![];
        let now = time::Instant::now();
        let mut remaining = n;
        let mut passed: u64 = 0;
        for (_, hash) in self.by_storage_index.iter() {
            if remaining == 0 {
                break;
            }
            let entry = self.get(hash).unwrap();
            if entry.effective_from > now {
                passed += 1;
                continue;
            } else {
                r.push(entry.transaction.clone());
                remaining -= 1;
            }
        }
        trace!("Excluding {} transactions due to jittering", passed);
        return r;
    }

    /// get size/length
    pub fn len(&self) -> usize {
        self.by_hash.len()
    }
}

#[cfg(test)]
pub mod tests {}
