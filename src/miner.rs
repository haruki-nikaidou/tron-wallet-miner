use std::sync::Arc;
use bip39::Mnemonic;
use bs58::encode;
use tracing::info;
use crate::address::{end_with, generate_tron_address};
use crate::hasher::{next_hash, repeat};
use crate::monitor::Monitor;

pub fn mnemonic_to_string(mnemonic: &Mnemonic) -> String {
    mnemonic.word_iter().map(|a| a.to_string()).reduce(|a, b| format!("{} {}", a, b)).unwrap().to_owned()
}

pub fn create_miner(
    start: &str,
    worker_id: usize,
    require: &str,
    monitor: Arc<Monitor>
) {
    let start = repeat(start.to_owned(), worker_id);
    let mut hash = start;
    loop {
        if monitor.is_found() {
            break;
        }
        hash = next_hash(hash);
        let mnemonic = Mnemonic::from_entropy(&hash).unwrap();
        let address = generate_tron_address(&mnemonic);
        if end_with(address, require) {
            monitor.found();
            info!("found: {}", mnemonic_to_string(&mnemonic));
            info!("address: {}", encode(address).into_string());
            break;
        }
        monitor.add_one();
    }
}