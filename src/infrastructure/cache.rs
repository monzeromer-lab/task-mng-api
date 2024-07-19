use moka::future::Cache;
use crate::constns::cache::CACHE_VALUES;

pub fn create_cache_store(capacity: u64) -> Cache<String, CACHE_VALUES> {
    Cache::new(capacity)
}
