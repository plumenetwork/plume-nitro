// Copyright 2022-2024, Offchain Labs, Inc.
// For license information, see https://github.com/OffchainLabs/nitro/blob/master/LICENSE

use arbutil::Bytes32;
use eyre::Result;
use lazy_static::lazy_static;
use lru_mem::{HeapSize, LruCache};
use parking_lot::Mutex;
use prover::programs::config::CompileConfig;
use std::collections::HashMap;
use wasmer::{Engine, Module, Store};

use crate::target_cache::target_native;

lazy_static! {
    static ref INIT_CACHE: Mutex<InitCache> = Mutex::new(InitCache::new(256 * 10 * 1024));
}

macro_rules! cache {
    () => {
        INIT_CACHE.lock()
    };
}

pub struct InitCache {
    long_term: HashMap<CacheKey, CacheItem>,
    lru: LruCache<CacheKey, CacheItem>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct CacheKey {
    module_hash: Bytes32,
    version: u16,
    debug: bool,
}

impl CacheKey {
    fn new(module_hash: Bytes32, version: u16, debug: bool) -> Self {
        Self {
            module_hash,
            version,
            debug,
        }
    }
}

impl HeapSize for CacheKey {
    fn heap_size(&self) -> usize {
        0
    }
}

#[derive(Clone)]
struct CacheItem {
    module: Module,
    engine: Engine,
    asm_size_estimate: u32,
}

impl CacheItem {
    fn new(module: Module, engine: Engine, asm_size_estimate: u32) -> Self {
        Self { module, engine, asm_size_estimate }
    }

    fn data(&self) -> (Module, Store) {
        (self.module.clone(), Store::new(self.engine.clone()))
    }
}

impl HeapSize for CacheItem {
    fn heap_size(&self) -> usize {
        return self.asm_size_estimate.try_into().unwrap();
    }
}

impl InitCache {
    // current implementation only has one tag that stores to the long_term
    // future implementations might have more, but 0 is a reserved tag
    // that will never modify long_term state
    const ARBOS_TAG: u32 = 1;

    fn new(size: usize) -> Self {
        Self {
            long_term: HashMap::new(),
            lru: LruCache::new(size),
        }
    }

    // TODO: Check if needs to shrink capacity
    pub fn set_lru_size(size: u32) {
        cache!()
            .lru
            .set_max_size(size.try_into().unwrap())
    }

    /// Retrieves a cached value, updating items as necessary.
    pub fn get(module_hash: Bytes32, version: u16, debug: bool) -> Option<(Module, Store)> {
        let mut cache = cache!();
        let key = CacheKey::new(module_hash, version, debug);

        // See if the item is in the long term cache
        if let Some(item) = cache.long_term.get(&key) {
            return Some(item.data());
        }

        // See if the item is in the LRU cache, promoting if so
        if let Some(item) = cache.lru.get(&key) {
            return Some(item.data());
        }
        None
    }

    /// Inserts an item into the long term cache, cloning from the LRU cache if able.
    /// If long_term_tag is 0 will only insert to LRU
    pub fn insert(
        module_hash: Bytes32,
        module: &[u8],
        asm_size_estimate: u32,
        version: u16,
        long_term_tag: u32,
        debug: bool,
    ) -> Result<(Module, Store)> {
        let key = CacheKey::new(module_hash, version, debug);

        // if in LRU, add to ArbOS
        let mut cache = cache!();
        if let Some(item) = cache.long_term.get(&key) {
            return Ok(item.data());
        }
        if let Some(item) = cache.lru.peek(&key).cloned() {
            if long_term_tag == Self::ARBOS_TAG {
                cache.long_term.insert(key, item.clone());
            } else {
                cache.lru.touch(&key)
            }
            return Ok(item.data());
        }
        drop(cache);

        let engine = CompileConfig::version(version, debug).engine(target_native());
        let module = unsafe { Module::deserialize_unchecked(&engine, module)? };

        let item = CacheItem::new(module, engine, asm_size_estimate);
        let data = item.data();
        let mut cache = cache!();
        if long_term_tag != Self::ARBOS_TAG {
            // TODO: handle result
            let _ = cache.lru.insert(key, item);
        } else {
            cache.long_term.insert(key, item);
        }
        Ok(data)
    }

    /// Evicts an item in the long-term cache.
    pub fn evict(module_hash: Bytes32, version: u16, long_term_tag: u32, debug: bool) {
        if long_term_tag != Self::ARBOS_TAG {
            return;
        }
        let key = CacheKey::new(module_hash, version, debug);
        let mut cache = cache!();
        if let Some(item) = cache.long_term.remove(&key) {
            // TODO: handle result
            let _ = cache.lru.insert(key, item);
        }
    }

    pub fn clear_long_term(long_term_tag: u32) {
        if long_term_tag != Self::ARBOS_TAG {
            return;
        }
        let mut cache = cache!();
        let cache = &mut *cache;
        for (key, item) in cache.long_term.drain() {
            // TODO: handle result
            let _ = cache.lru.insert(key, item); // not all will fit, just a heuristic
        }
    }
}
