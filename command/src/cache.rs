use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

pub struct Cache;

pub type CacheValue = Arc<Mutex<Option<String>>>;
pub type CacheMap = Arc<DashMap<String, (Instant, CacheValue)>>;

impl Cache{
    pub fn new() -> CacheMap{
        let cache: CacheMap = Arc::new(DashMap::new());
        cache
    }
}

#[async_trait]
pub trait Cache_set {
    async fn cache_set(cache: CacheMap, key: String, value: String, ttl: Duration);
}

#[async_trait]
pub trait Cache_get {
    async fn cache_get(cache: CacheMap, key: String) -> Option<String>;
}

#[async_trait]
impl Cache_set for Cache{
    async fn cache_set(cache: CacheMap, key: String, value: String, ttl: Duration) {
        let cache_value = Arc::new(Mutex::new(Some(value)));
        let expiration = Instant::now() + ttl;
        cache.insert(key, (expiration, cache_value));
    }
}

#[async_trait]
impl Cache_get for Cache {        
    async fn cache_get(cache: CacheMap, key: String) -> Option<String> {
        if let Some(entry) = cache.get(&key) {
            let (expiration, cache_value) = entry.value();
            if Instant::now() < *expiration {
                return cache_value.lock().await.clone();
            } else {
                cache.remove(&key);
            }
        }
        None
    }
}

use crate::command::{cd, help, history, ls, pwd, PATH};
// 初始化缓存
pub static COMMAND_CACHE: Lazy<OnceCell<CacheMap>> = Lazy::new(OnceCell::new);

pub async fn initialize_command_cache() -> &'static CacheMap {
    let res = COMMAND_CACHE.get_or_init(|| async {
        let cache = Cache::new();
        <Cache as Cache_set>::cache_set(cache.clone(), "pwd".to_string(), pwd(), Duration::from_secs(2)).await;
        <Cache as Cache_set>::cache_set(cache.clone(), "ls".to_string(), ls().unwrap(), Duration::from_secs(2)).await;
        <Cache as Cache_set>::cache_set(cache.clone(), "cd".to_string(), cd(unsafe { PATH }).unwrap(), Duration::from_secs(2)).await;
        <Cache as Cache_set>::cache_set(cache.clone(), "help".to_string(), help(), Duration::from_secs(2)).await;
        <Cache as Cache_set>::cache_set(cache.clone(), "history".to_string(), history(), Duration::from_secs(2)).await;

        cache
    }).await;
    res
}