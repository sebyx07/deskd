// Element caching with TTL
use super::element::Element;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Element cache with time-to-live expiration
#[allow(dead_code)]
pub struct ElementCache {
    cache: RwLock<HashMap<String, CachedElement>>,
    ttl: Duration,
}

#[allow(dead_code)]
struct CachedElement {
    element: Element,
    cached_at: Instant,
}

#[allow(dead_code)]
impl ElementCache {
    /// Create a new element cache with specified TTL
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    /// Get an element from cache if not expired
    pub async fn get(&self, key: &str) -> Option<Element> {
        let cache = self.cache.read().await;

        if let Some(cached) = cache.get(key) {
            if cached.cached_at.elapsed() < self.ttl {
                debug!("Cache hit for: {}", key);
                return Some(cached.element.clone());
            } else {
                debug!("Cache expired for: {}", key);
            }
        }

        None
    }

    /// Store an element in the cache
    pub async fn set(&self, key: String, element: Element) {
        debug!("Caching element: {}", key);

        let mut cache = self.cache.write().await;
        cache.insert(
            key,
            CachedElement {
                element,
                cached_at: Instant::now(),
            },
        );
    }

    /// Invalidate a specific cache entry
    pub async fn invalidate(&self, key: &str) {
        debug!("Invalidating cache for: {}", key);

        let mut cache = self.cache.write().await;
        cache.remove(key);
    }

    /// Clear all expired entries
    pub async fn clear_expired(&self) {
        let mut cache = self.cache.write().await;

        let before_count = cache.len();
        cache.retain(|_, cached| cached.cached_at.elapsed() < self.ttl);
        let after_count = cache.len();

        if before_count != after_count {
            info!(
                "Cleared {} expired cache entries",
                before_count - after_count
            );
        }
    }

    /// Clear all cache entries
    pub async fn clear_all(&self) {
        info!("Clearing all cache entries");

        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let total = cache.len();
        let expired = cache
            .values()
            .filter(|cached| cached.cached_at.elapsed() >= self.ttl)
            .count();

        CacheStats {
            total_entries: total,
            expired_entries: expired,
            active_entries: total - expired,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub active_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atspi::element::ElementBounds;

    #[tokio::test]
    async fn test_cache_basic() {
        let cache = ElementCache::new(60);

        let element = Element {
            name: "Test Button".to_string(),
            role: "PushButton".to_string(),
            description: "A test button".to_string(),
            states: vec!["Enabled".to_string()],
            bounds: Some(ElementBounds {
                x: 100,
                y: 200,
                width: 80,
                height: 30,
            }),
            path: "/org/a11y/atspi/accessible/1234".to_string(),
        };

        // Set element
        cache.set("test_key".to_string(), element.clone()).await;

        // Get element
        let cached = cache.get("test_key").await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().name, "Test Button");

        // Get non-existent element
        let missing = cache.get("missing_key").await;
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = ElementCache::new(1); // 1 second TTL

        let element = Element {
            name: "Test Button".to_string(),
            role: "PushButton".to_string(),
            description: "".to_string(),
            states: vec![],
            bounds: None,
            path: "/org/a11y/atspi/accessible/1234".to_string(),
        };

        cache.set("test_key".to_string(), element).await;

        // Should be cached
        assert!(cache.get("test_key").await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Should be expired
        assert!(cache.get("test_key").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let cache = ElementCache::new(60);

        let element = Element {
            name: "Test Button".to_string(),
            role: "PushButton".to_string(),
            description: "".to_string(),
            states: vec![],
            bounds: None,
            path: "/org/a11y/atspi/accessible/1234".to_string(),
        };

        cache.set("test_key".to_string(), element).await;
        assert!(cache.get("test_key").await.is_some());

        cache.invalidate("test_key").await;
        assert!(cache.get("test_key").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = ElementCache::new(60);

        let element = Element {
            name: "Test Button".to_string(),
            role: "PushButton".to_string(),
            description: "".to_string(),
            states: vec![],
            bounds: None,
            path: "/org/a11y/atspi/accessible/1234".to_string(),
        };

        cache.set("key1".to_string(), element.clone()).await;
        cache.set("key2".to_string(), element).await;

        let stats = cache.stats().await;
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.active_entries, 2);
    }
}
