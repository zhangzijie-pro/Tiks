use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tokio::sync::OnceCell;

pub struct Cache;

pub type CacheValue = Arc<Mutex<Option<String>>>;
pub type CacheMap = Arc<DashMap<String, CacheValue>>;

impl Cache{
    pub fn new() -> CacheMap{
        let cache: CacheMap = Arc::new(DashMap::new());
        cache
    }

}

#[allow(non_camel_case_types)]
#[async_trait]
pub trait Cache_set {
    async fn cache_set(cache: CacheMap, key: String, value: String);
}

#[allow(non_camel_case_types)]
#[async_trait]
pub trait Cache_get {
    async fn cache_get(cache: CacheMap, key: String) -> Option<String>;
}

#[async_trait]
impl Cache_set for Cache{
    async fn cache_set(cache: CacheMap, key: String, value: String) {
        let cache_value = Arc::new(Mutex::new(Some(value)));
        cache.insert(key, cache_value);
    }
}

#[async_trait]
impl Cache_get for Cache {        
    async fn cache_get(cache: CacheMap, key: String) -> Option<String> {
        if let Some(entry) = cache.get(&key) {
            /*let (expiration, cache_value) = entry.value();
            if Instant::now() < *expiration {
                return cache_value.lock().await.clone();
            } else {
                cache.remove(&key);
            }*/
            return entry.value().lock().await.clone();
        }
        None
    }
}

use crate::command::{ help, history, ls, pwd};
// 初始化缓存
pub static COMMAND_CACHE: Lazy<OnceCell<CacheMap>> = Lazy::new(OnceCell::new);

pub async fn initialize_command_cache() -> &'static CacheMap {
    let res = COMMAND_CACHE.get_or_init(|| async {
        let cache = Cache::new();
        let cache_clone = cache.clone();
        let cache_set_ft = async move{
            <Cache as Cache_set>::cache_set(cache_clone.clone(), "pwd".to_string(), pwd()).await;
            <Cache as Cache_set>::cache_set(cache_clone.clone(), "ls".to_string(), ls().unwrap()).await;
            <Cache as Cache_set>::cache_set(cache_clone.clone(), "help".to_string(), help()).await;
            <Cache as Cache_set>::cache_set(cache_clone.clone(), "history".to_string(), history()).await;
        };
        tokio::task::spawn_blocking(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(cache_set_ft)
        });

        cache
    }).await;
    res
}