use hex::ToHex;
use sha2::{Digest, Sha256};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::Vec;

/// Generates a SHA-256 hash for a given number
///
/// # Arguments
///
/// * `num` - The number to hash
///
/// # Returns
///
/// * A string representation of the SHA-256 hash
fn sha256(num: u64) -> String {
    let mut hasher = Sha256::new();

    hasher.update(num.to_string().as_bytes());
    hasher.finalize().encode_hex()
}

/// Worker thread function that computes hashes and checks for desired suffix
///
/// # Arguments
///
/// * `counter` - Atomic counter to keep track of the number being hashed
/// * `results` - Shared vector to store results
/// * `zero_suffix` - Suffix that hashes should end with
/// * `found` - Atomic flag indicating if the required number of hashes is found
/// * `hash_count` - Number of hashes to find
fn worker_thread(
    counter: Arc<AtomicU64>,
    results: Arc<Mutex<Vec<(u64, String)>>>,
    zero_suffix: Arc<String>,
    found: Arc<AtomicBool>,
    hash_count: usize,
) {
    while !found.load(Ordering::SeqCst) {
        let num = counter.fetch_add(1, Ordering::SeqCst);
        let hash = sha256(num);

        if hash.ends_with(zero_suffix.deref()) {
            let mut results = results.lock().unwrap();

            results.push((num, hash.clone()));
            println!("{}, \"{}\"", num, hash);

            if results.len() >= hash_count {
                found.store(true, Ordering::SeqCst);
            }
        }
    }
}

/// Initiates the hash finding process with multiple threads
///
/// # Arguments
///
/// * `zero_count` - The number of trailing zeros the hash should end with
/// * `hash_count` - The number of hashes to find
///
/// # Returns
///
/// * A shared vector containing the found hashes and their corresponding numbers
pub fn found_hash(zero_count: usize, hash_count: usize) -> Arc<Mutex<Vec<(u64, String)>>> {
    let counter = Arc::new(AtomicU64::new(1));
    let results = Arc::new(Mutex::new(Vec::new()));
    let found = Arc::new(AtomicBool::new(false));
    let zero_suffix = Arc::new("0".repeat(zero_count));
    let mut handles = vec![];

    for _ in 0..num_cpus::get() {
        let counter_clone = Arc::clone(&counter);
        let results_clone = Arc::clone(&results);
        let found_clone = Arc::clone(&found);
        let zero_suffix_clone = Arc::clone(&zero_suffix);

        let handle = thread::spawn(move || {
            worker_thread(
                counter_clone,
                results_clone,
                zero_suffix_clone,
                found_clone,
                hash_count,
            )
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results
}


