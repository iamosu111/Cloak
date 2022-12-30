use super::{IdType, TsType, PkType};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{digest::*, KeyType, FloatType};

// static INDEX_ID_CNT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockData {
    pub block_id: IdType,
    pub tx_ids: Vec<IdType>,
}

//block_id == block_height, data_root = data.hash()
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_id: IdType,
    pub pre_hash: Digest,
    pub time_stamp: TsType,
}


impl Digestible for BlockHeader {
    fn to_digest(&self) -> Digest{
        let mut state = blake2().to_state();
        state.update(&self.block_id.to_le_bytes());
        state.update(&self.pre_hash.0);
        state.update(&self.time_stamp.to_le_bytes());
        Digest::from(state.finalize())
    }
}
