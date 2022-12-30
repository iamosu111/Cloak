use std::collections::{HashMap, BTreeMap};
use log::info;
use crate::Digest;
use super::*;

///
/// 
/// For BlockData
/// --
/// intra-index is sorted & storing the first position of distinguished key.
/// 
/// aggre_signs is a series of aggregated signature based on transaction's unique key.
/// 
/// For BlockHeader
/// --
/// 
pub fn build_block<'a>(
    block_id: IdType,
    pre_hash: Digest,
    raw_txs: impl Iterator<Item = &'a RawTransaction>,
    chain: &mut (impl ReadInterface + WriteInterface),
) -> Result<BlockHeader> {    
    // let param = chain.get_parameter()?;
    let txs: Vec<Transaction> = raw_txs.map(|rtx: &RawTransaction| Transaction::create(rtx )).collect();
    let mut _time_stamp: TsType = Default::default();
    let mut tx_ids: Vec<IdType> = Vec::new();
    _time_stamp = txs[0].value.time_stamp;
    for tx in txs{
        chain.write_transaction(tx.clone())?;
        tx_ids.push(tx.id);
    }
    let block_header = BlockHeader{
        block_id,
        pre_hash,
        time_stamp: _time_stamp,
    };

    let block_data = BlockData {
        block_id,
        tx_ids,
    };

    chain.write_block_header(block_header.clone())?;
    chain.write_block_data(block_data.clone())?;

    Ok(block_header)
}

