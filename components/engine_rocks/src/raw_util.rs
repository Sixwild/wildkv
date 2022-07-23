use engine_traits::{Result};
use rocksdb::{
    load_latest_options, CColumnFamilyDescriptor, ColumnFamilyOptions, DBOptions, Env, DB,
};


// pub fn new_engine_opt(path: &str) -> Result<DB> {
//     return OK(());
// }

pub fn new_engine_default(path: &str) -> Result<DB> {
    let mut db = DB::open_default(path)?;
    Ok(db)
}