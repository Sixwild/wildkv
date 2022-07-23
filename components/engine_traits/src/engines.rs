use crate::{
    KvEngine,
};

#[derive(Clone, Debug)]
pub struct Engines<K> {
    pub kv: K,
}

impl <K: KvEngine> Engines<K> {
    pub fn new(kv_engine: K) -> Self {
        Engines {
            kv: kv_engine,
        }
    }
}