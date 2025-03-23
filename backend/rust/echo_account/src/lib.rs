pub mod assembly;
pub mod business;
pub mod caches;
pub mod logger;
pub mod queues;
pub mod stores;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}
