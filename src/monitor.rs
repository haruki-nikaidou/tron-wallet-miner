use std::sync::atomic::{AtomicBool, AtomicUsize};
use tracing::info;

pub struct Monitor {
    count: AtomicUsize,
    fount: AtomicBool
}

impl Monitor {
    fn set_zero(&self) {
        self.count.store(0, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn add_one(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn print_reset(&self) {
        info!("current speed: {}/s", self.count.load(std::sync::atomic::Ordering::Relaxed));
        self.set_zero();
    }
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
            fount: AtomicBool::new(false)
        }
    }
    pub fn found(&self) {
        self.fount.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn is_found(&self) -> bool {
        self.fount.load(std::sync::atomic::Ordering::Relaxed)
    }
}

unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}