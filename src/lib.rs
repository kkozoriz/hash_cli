use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::vec::Vec;
use hex::ToHex;

fn sha256(num: u64, buffer: &mut Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    buffer.clear();
    buffer.extend_from_slice(num.to_string().as_bytes());
    hasher.update(buffer);
    hasher.finalize().encode_hex()
}

fn worker_thread(
    counter: Arc<AtomicU64>,
    results: Arc<Mutex<Vec<(u64, String)>>>,
    zero_suffix: String,
    found: Arc<AtomicBool>,
    hash_count: usize,
) {
    let mut buffer = Vec::with_capacity(20);
    while !found.load(Ordering::SeqCst) {
        let num = counter.fetch_add(1, Ordering::SeqCst);
        let hash = sha256(num, &mut buffer);

        if hash.ends_with(&zero_suffix) {
            let mut results = results.lock().unwrap();
            results.push((num, hash.clone()));
            println!("{}, \"{}\"", num, hash);

            if results.len() >= hash_count {
                found.store(true, Ordering::SeqCst);
            }
        }
    }
}

pub fn found_hash(zero_count: usize, hash_count: usize) {
    let counter = Arc::new(AtomicU64::new(1));
    let results = Arc::new(Mutex::new(Vec::new()));
    let found = Arc::new(AtomicBool::new(false));
    let zero_suffix = "0".repeat(zero_count);

    let mut handles = vec![];

    for _ in 0..num_cpus::get() {
        let counter_clone = Arc::clone(&counter);
        let results_clone = Arc::clone(&results);
        let found_clone = Arc::clone(&found);
        let zero_suffix_clone = zero_suffix.clone();

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_sha256_test() {
    }
}
