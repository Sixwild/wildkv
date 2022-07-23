use std::fmt::Debug;

use crate::*;

pub trait KvEngine :
    Send
    + Sync
    + Clone
    + Debug
    + Unpin
    +'static
{

}