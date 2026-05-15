#![allow(dead_code)]

use super::ChildState;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub fn start_signal_watcher(
    _state: Arc<Mutex<ChildState>>,
    _shutdown: Arc<AtomicBool>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {})
}
