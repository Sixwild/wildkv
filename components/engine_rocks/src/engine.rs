use engine_traits:: {
    KvEngine, Result,
};
use rocksdb::{DBIterator, Writable, DB};
use std::{sync::Arc};


#[derive(Clone, Debug)]
pub struct RocksEngine {
    db:Arc<DB>
}

impl RocksEngine {
    pub fn from_db(db: Arc<DB>) -> Self {
        RocksEngine {
            db,
        }
    }
}

impl KvEngine for RocksEngine {
}