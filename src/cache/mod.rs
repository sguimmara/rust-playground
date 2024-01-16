use std::collections::HashMap;

type K = String;

pub struct Cache<T> {
    entries: HashMap<K, T>,
    accesses: HashMap<K, usize>,
    max_entries: Option<usize>
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::default(),
            max_entries: None,
            accesses: HashMap::default(),
        }
    }
}

impl<T> Cache<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_entries(value: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(value),
            max_entries: Some(value),
            accesses: HashMap::with_capacity(value),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn trim(&mut self) {
        if let Some(limit) = self.max_entries {
            let len = self.len();
            if limit < len {
                let to_remove = len - limit;
                let mut vec: Vec<(K, usize)> = Vec::with_capacity(self.accesses.len());

                for (k, v) in &self.accesses {
                    vec.push((k.clone(), *v));
                }

                vec.sort_unstable_by_key(|k| { k.1 });

                for i in 0..to_remove {
                    let (k, _) = &vec[i];
                    self.entries.remove(k);
                    self.accesses.remove(k);
                }
            }
        }
    }

    pub fn insert(&mut self, key: &str, value: T) -> Option<T> {
        if self.max_entries.is_some() {
            self.accesses.insert(key.to_string(), 0);
        }
        return self.entries.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.entries.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        self.accesses.remove(key);
        return self.entries.remove(key);
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }
}

impl<T: Clone> Clone for Cache<T> {
    fn clone(&self) -> Self {
        let mut entries = HashMap::new();

        for (k, v) in &self.entries  {
            entries.insert(k.clone(), v.clone());
        }

        Self {
            entries,
            max_entries: self.max_entries,
            accesses: HashMap::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cache::Cache;

    #[test]
    fn default() {
        let cache: Cache<usize> = Cache::default();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn insert() {
        let mut cache: Cache<usize> = Cache::default();

        cache.insert("hello", 80);

        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn clear() {
        let mut cache: Cache<usize> = Cache::default();

        cache.insert("hello", 80);
        cache.insert("hello 2", 80);
        cache.insert("hello 3", 80);

        assert_eq!(cache.len(), 3);

        cache.clear();

        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn trim() {
        let mut cache: Cache<usize> = Cache::with_max_entries(4);

        cache.insert("k0", 0);
        cache.insert("k1", 1);
        cache.insert("k2", 2);
        cache.insert("k3", 3);
        cache.insert("k4", 4);
        cache.insert("k5", 5);

        assert_eq!(6, cache.len());

        cache.trim();

        assert_eq!(4, cache.len());
    }
}
