use crate::consts::cache::CACHE_VALUES;
use moka::future::Cache;

pub fn create_cache_store(capacity: u64) -> Cache<String, CACHE_VALUES> {
    Cache::new(capacity)
}
