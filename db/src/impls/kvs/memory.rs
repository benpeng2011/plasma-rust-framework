use crate::error::Error;
use crate::traits::db::DatabaseTrait;
use crate::traits::kvs::{BaseDbKey, Batch, Bucket, KeyValue, KeyValueStore};
use parking_lot::RwLock;
use std::collections::BTreeMap;

pub struct CoreDbMemoryImpl {
    db: RwLock<BTreeMap<BaseDbKey, Vec<u8>>>,
}

impl DatabaseTrait for CoreDbMemoryImpl {
    fn open(_dbname: &str) -> Self {
        Self {
            db: RwLock::new(BTreeMap::new()),
        }
    }
    fn close(&self) {}
}

impl KeyValueStore for CoreDbMemoryImpl {
    fn get(&self, key: &BaseDbKey) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.db.read().get(key).map(|v| v.to_vec()))
    }
    fn put(&self, key: &BaseDbKey, value: &[u8]) -> Result<(), Error> {
        self.db.write().insert(key.clone(), value.to_vec());
        Ok(())
    }
    fn del(&self, key: &BaseDbKey) -> Result<(), Error> {
        self.db.write().remove(key);
        Ok(())
    }
    fn has(&self, _key: &BaseDbKey) -> Result<bool, Error> {
        Ok(true)
    }
    fn batch(&self, operations: &[Batch]) -> Result<(), Error> {
        let mut write_lock = self.db.write();
        for op in operations.iter() {
            match op {
                Batch::BatchPut { key, value } => write_lock.insert(key.clone(), value.clone()),
                Batch::BatchDel { key } => write_lock.remove(key),
            };
        }
        Ok(())
    }
    fn iter_all_with_prefix(
        &self,
        prefix: &BaseDbKey,
        start: &BaseDbKey,
        mut f: Box<dyn FnMut(&BaseDbKey, &Vec<u8>) -> bool>,
    ) -> Vec<KeyValue> {
        let read_lock = self.db.read();
        let iter = read_lock.iter();
        let mut result = vec![];
        for (k, v) in iter {
            if *k > prefix.concat(start) {
                if k.0.starts_with(&prefix.0) && f(&k, &v) {
                    result.push(KeyValue::new(k.clone(), v.clone()));
                    continue;
                } else {
                    break;
                }
            }
        }
        result
    }
    fn iter_all(
        &self,
        start: &BaseDbKey,
        mut f: Box<dyn FnMut(&BaseDbKey, &Vec<u8>) -> bool>,
    ) -> Vec<KeyValue> {
        let read_lock = self.db.read();
        let iter = read_lock.iter();
        let mut result = vec![];
        for (k, v) in iter {
            if k > start {
                if f(&k, &v) {
                    result.push(KeyValue::new(k.clone(), v.clone()));
                    continue;
                } else {
                    break;
                }
            }
        }
        result
    }
    fn bucket<'a>(&'a self, prefix: &BaseDbKey) -> Bucket<'a> {
        Bucket::new(prefix.clone(), self)
    }
}

#[cfg(test)]
mod tests {
    use super::CoreDbMemoryImpl;
    use crate::traits::db::DatabaseTrait;
    use crate::traits::kvs::{Bucket, KeyValueStore};

    #[test]
    fn test_bucket() {
        let core_db = CoreDbMemoryImpl::open("test");
        let root: Bucket = core_db.root();
        let bucket: Bucket = root.bucket(&b"a"[..].into());
        assert_eq!(bucket.put(&b"b"[..].into(), &b"value"[..]).is_ok(), true);
        let result = root.get(&b"ab"[..].into());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.ok().unwrap().unwrap(), b"value".to_vec());
    }

    #[test]
    fn test_iter() {
        let core_db = CoreDbMemoryImpl::open("test");
        let root: Bucket = core_db.root();
        let bucket_a: Bucket = root.bucket(&"a".into());
        let bucket_b: Bucket = root.bucket(&"b".into());
        assert_eq!(bucket_a.put(&"0".into(), &b"value"[..]).is_ok(), true);
        assert_eq!(bucket_a.put(&"1".into(), &b"value"[..]).is_ok(), true);
        assert_eq!(bucket_b.put(&"0".into(), &b"value"[..]).is_ok(), true);
        let result = bucket_a.iter_all(&"".into(), Box::new(move |_k, _v| true));
        assert_eq!(result.len(), 2);
    }
}
